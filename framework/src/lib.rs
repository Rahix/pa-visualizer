extern crate dft;
extern crate hound;
#[macro_use]
extern crate log;
extern crate pulse_simple;
extern crate rb;
extern crate toml;

mod recorder;
mod playback;
mod spectralizer;

pub use spectralizer::AudioInfo;

pub struct RenderingInfo<'a> {
    pub fps: f32,
    pub frame_time: f32,
    pub outdir: String,
    pub frame: usize,
    sample_rate: u32,
    spectralizer: spectralizer::Spectralizer,
    samples: hound::WavSamples<'a, std::io::BufReader<std::fs::File>, f32>,
    audio_producer: rb::Producer<[f32; 2]>,
}

pub enum RunMode<'a> {
    /// Running live on the screen
    Live,
    /// Rendering a video of the vis
    Rendering(RenderingInfo<'a>),
}

// Start the framework
pub fn start<
    P: AsRef<std::path::Path>,
    F: Fn(::std::sync::Arc<toml::Value>,
       ::std::sync::Arc<::std::sync::RwLock<AudioInfo>>,
       RunMode),
>(
    cfg: P,
    visualizer: F,
) {
    use rb::RB;

    let config = {
        use std::io::Read;

        let mut cfg_string = String::new();
        std::fs::File::open(cfg)
            .expect("Can't open config file")
            .read_to_string(&mut cfg_string)
            .expect("Can't read config file contents");

        std::sync::Arc::new(cfg_string.parse::<toml::Value>().expect(
            "Can't parse config file",
        ))
    };

    let buffer_size = config
        .get("RECORD_BUFFER_SIZE")
        .map(|v| {
            v.as_integer().expect(
                "RECORD_BUFFER_SIZE must be an integer",
            )
        })
        .unwrap_or(1024 * 8) as usize;
    info!("RECORD_BUFFER_SIZE = {}", buffer_size);

    let is_render_mode = config
        .get("RENDER_MODE")
        .map(|v| v.as_bool().expect("RENDER_MODE must be a boolean"))
        .unwrap_or(false) as bool;
    info!("RENDER_MODE = {}", is_render_mode);

    let audio_buffer: rb::SpscRb<[f32; 2]> = rb::SpscRb::new(buffer_size);

    let audio_producer = audio_buffer.producer();

    let audio_info = std::sync::Arc::new(std::sync::RwLock::new(spectralizer::AudioInfo {
        columns_left: vec![],
        columns_right: vec![],
        raw_spectrum_left: vec![],
        raw_spectrum_right: vec![],
    }));

    if !is_render_mode {
        let config2 = config.clone();
        std::thread::spawn(move || recorder::recorder(config2, audio_producer));

        let config3 = config.clone();
        let audio_info2 = audio_info.clone();
        std::thread::spawn(move || {
            let mut s = spectralizer::Spectralizer::new(config3, audio_buffer, audio_info2);

            loop {
                s.spectralize();
                ::std::thread::sleep(::std::time::Duration::from_millis(15));
            }
        });

        visualizer(config, audio_info, RunMode::Live);
    } else {
        // Record mode
        let render_fps = config
            .get("RENDER_FPS")
            .map(|v| v.as_float().expect("RENDER_FPS must be a float"))
            .unwrap_or(30.0) as f32;
        info!("RENDER_FPS = {}", render_fps);

        let soundfile = config
            .get("RENDER_SOUNDFILE")
            .map(|v| {
                String::from(v.as_str().expect("RENDER_SOUNDFILE must be a string"))
            })
            .expect("A soundfile (RENDER_SOUNDFILE config option) is required");
        info!("RENDER_SOUNDFILE = \"{}\"", soundfile);

        let outdir = config
            .get("RENDER_OUTPUT_DIRECTORY")
            .map(|v| {
                String::from(v.as_str().expect(
                    "RENDER_OUTPUT_DIRECTORY must be a string",
                ))
            })
            .unwrap_or(String::from("render_frames"));
        info!("RENDER_OUTPUT_DIRECTORY = \"{}\"", outdir);

        let mut file = hound::WavReader::open(soundfile).expect("Could not open sound file");
        let sample_rate = file.spec().sample_rate;
        info!("Sample rate is {}", sample_rate);
        let samples = file.samples::<f32>();

        let config3 = config.clone();
        let audio_info2 = audio_info.clone();
        let spectralizer = spectralizer::Spectralizer::new(config3, audio_buffer, audio_info2);

        let mut render_info = RenderingInfo {
            fps: render_fps,
            frame: 0,
            frame_time: 1.0 / render_fps,
            sample_rate,
            outdir,
            samples,
            spectralizer,
            audio_producer,
        };

        /*if !playback::playback_frame(&mut render_info) {
            panic!("Sound file is too short for rendering!");
        }

        render_info.spectralizer.spectralize();*/

        visualizer(config, audio_info, RunMode::Rendering(render_info));
    }
}

pub fn sleep(mode: &mut RunMode) {
    match *mode {
        // If we are live, yield to the os
        RunMode::Live => std::thread::sleep(std::time::Duration::from_millis(2)),
        // If we are rendering, advance the recorder and spectralizer
        RunMode::Rendering(ref mut render_info) => {
            info!("Rendering frame {} ...", render_info.frame);
            if !playback::playback_frame(render_info) {
                info!("Done!");
                std::process::exit(0);
            }
            render_info.spectralizer.spectralize();
            render_info.frame += 1;
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }
}
