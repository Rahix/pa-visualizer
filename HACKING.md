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

    // Visualizer goes here. For documentation on audio_info,
    // please take a look at framework/src/spectralizer.rs
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
