extern crate framework;
extern crate pretty_env_logger;
extern crate toml;

fn visualizer(
    _config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
) {
    print!("\x1B[2J");
    loop {
        let ai = audio_info.read().expect("Couldn't read audio info");
        print!("\x1B[H");
        for si in 0..ai.columns_left.len() {
            let reshape = 1.0; //(si as f32) / (ai.spectrum_left.len() as f32).sqrt();
            let vl = (30.0 * ai.columns_left[si] * reshape) as i8;
            let vr = (30.0 * ai.columns_right[si] * reshape) as i8;
            for _ in 0..(31 - vl) {
                print!(" ");
            }
            for _ in 0..vl {
                print!("#");
            }
            for _ in 0..vr {
                print!("#");
            }
            for _ in 0..(31 - vr) {
                print!(" ");
            }
            println!("");
        }
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/ascii.toml", visualizer);
}
