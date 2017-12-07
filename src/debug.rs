extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
) {
    use std::io::Write;

    let time = config
        .get("TIME")
        .map(|v| {
            v.as_float().expect("TIME must be a float")
        })
        .unwrap_or(60.0) as f32;
    info!("TIME = {}s", time);

    let filename = config.get("FILENAME")
        .map(|v| String::from(v.as_str().expect("FILENAME must be a string")))
        .unwrap_or(String::from("data.m"));
    info!("FILENAME = \"{}\"", filename);

    let iterations = (time / 0.025) as usize;
    info!("Running for {} iterations", iterations);

    let mut f = std::fs::File::create(&filename).unwrap();
    writeln!(f, "data = [").unwrap();
    for i in 0..iterations {
        // Output raw spectrum data every 25ms
        let ai = audio_info.read().expect("Can't read audio info");
        write!(f, "{:?}", ai.raw_spectrum_left).unwrap();
        if i != (iterations - 1) {
            writeln!(f, ",").unwrap();
        } else {
            writeln!(f, "];").unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

    info!("Done!");
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/debug.toml", visualizer);
}
