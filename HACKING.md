HACKING
=======

To add a new visualizer, add a new source file with the following code:

```rust
extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
) {
    let display_columns = config
        .get("DISPLAY_COLUMNS")
        .map(|v| {
            v.as_integer().expect("DISPLAY_COLUMNS must be an integer")
        })
        .unwrap_or(50) as usize;
    info!("DISPLAY_COLUMNS = {}", display_columns);

    // Visualizer goes here. audio_info contains spectrum_left
    // and spectrum_right, two vectors of frequency buckets.
    // All of my visualizers only use the first display_columns
    // elements of these, but this is not a necessity.
    // Note that the first element of both vectors is always 0,
    // so I advise ignoring it
    error!("No visualizer here, yet :/");
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/your-visualizer-config-file.toml", visualizer);
}
```

You also need to create `configs/your-visualizer-config-file.toml` with the following
template:

```toml
RECORD_BUFFER_SIZE = 8000
RECORD_READ_BUFFER_SIZE = 100
RECORD_RATE = 8000

WINDOW_SIZE = 2048

DAMPEN = 0.9
MAX_DAMPEN = 0.99

COLUMNS = 75
DISPLAY_COLUMNS = 50
```
