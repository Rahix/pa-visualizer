extern crate dft;
#[macro_use]
extern crate log;
extern crate pulse_simple;
extern crate rb;
extern crate toml;

mod recorder;
mod spectralizer;

pub use spectralizer::AudioInfo;

// Start the framework
pub fn start<
    P: AsRef<std::path::Path>,
    F: Fn(::std::sync::Arc<toml::Value>,
       ::std::sync::Arc<::std::sync::RwLock<AudioInfo>>),
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

    let audio_buffer: rb::SpscRb<[f32; 2]> = rb::SpscRb::new(buffer_size);

    let audio_producer = audio_buffer.producer();

    let audio_info = std::sync::Arc::new(std::sync::RwLock::new(spectralizer::AudioInfo {
        columns_left: vec![],
        columns_right: vec![],
        raw_spectrum_left: vec![],
        raw_spectrum_right: vec![],
    }));

    let config2 = config.clone();
    std::thread::spawn(move || recorder::recorder(config2, audio_producer));

    let config3 = config.clone();
    let audio_info2 = audio_info.clone();
    std::thread::spawn(move || {
        spectralizer::spectralizer(config3, audio_buffer, audio_info2)
    });

    visualizer(config, audio_info);
}
