extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate sfml;
extern crate toml;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
) {
    use sfml::graphics::RenderTarget;
    use sfml::graphics::Transformable;
    use sfml::graphics::Shape;

    let display_columns = config
        .get("DISPLAY_COLUMNS")
        .map(|v| {
            v.as_integer().expect("DISPLAY_COLUMNS must be an integer")
        })
        .unwrap_or(50) as usize;
    info!("DISPLAY_COLUMNS = {}", display_columns);

    let window_height = config
        .get("WINDOW_HEIGHT")
        .map(|v| {
            v.as_integer().expect("WINDOW_HEIGHT must be an integer")
        })
        .unwrap_or(900) as u32;
    info!("WINDOW_HEIGHT = {}", window_height);

    let window_width = config
        .get("WINDOW_WIDTH")
        .map(|v| v.as_integer().expect("WINDOW_WIDTH must be an integer"))
        .unwrap_or(900) as u32;
    info!("WINDOW_WIDTH = {}", window_width);

    let right_channel = config
        .get("SIMPLE_RIGHT_CHANNEL")
        .map(|v| {
            v.as_bool().expect("SIMPLE_RIGHT_CHANNEL must be a boolean")
        })
        .unwrap_or(false);
    info!("SIMPLE_RIGHT_CHANNEL = {}", right_channel);

    let right_channel_middle = config
        .get("SIMPLE_RIGHT_CHANNEL_MIDDLE")
        .map(|v| {
            v.as_bool().expect(
                "SIMPLE_RIGHT_CHANNEL_MIDDLE must be a boolean",
            )
        })
        .unwrap_or(false);
    info!("SIMPLE_RIGHT_CHANNEL_MIDDLE = {}", right_channel_middle);

    let right_channel_color_offset = config
        .get("SIMPLE_RIGHT_CHANNEL_COLOR_OFFSET")
        .map(|v| {
            v.as_bool().expect(
                "SIMPLE_RIGHT_CHANNEL_COLOR_OFFSET must be a boolean",
            )
        })
        .unwrap_or(false);
    info!(
        "SIMPLE_RIGHT_CHANNEL_COLOR_OFFSET = {}",
        right_channel_color_offset
    );

    let symmetric = config
        .get("SIMPLE_SYMMETRIC_MODE")
        .map(|v| {
            v.as_bool().expect(
                "SIMPLE_SYMMETRIC_MODE must be a boolean",
            )
        })
        .unwrap_or(false);
    info!("SIMPLE_SYMMETRIC_MODE = {}", symmetric);


    let mut settings = sfml::window::ContextSettings::default();
    settings.antialiasing_level = 8;

    let mut window = sfml::graphics::RenderWindow::new(
        sfml::window::VideoMode::new(window_width, window_height, 32),
        "PulseAudio Visualizer",
        sfml::window::Style::NONE,
        &settings,
    );
    window.set_view(&sfml::graphics::View::new(
        sfml::system::Vector2::new(0.0, 0.0),
        sfml::system::Vector2::new(2.2, 2.2),
    ));

    let mut rect = sfml::graphics::RectangleShape::new();
    let width = if symmetric {
        0.5 / display_columns as f32
    } else {
        2.0 / display_columns as f32 - 0.01
    };

    let c1 = sfml::graphics::Color::rgb(0x9C, 0x9C, 0x9C);
    let c2 = sfml::graphics::Color::rgb(0xCC, 0xCC, 0xCC);

    'mainloop: loop {
        // Event handling
        while let Some(ref ev) = window.poll_event() {
            match *ev {
                sfml::window::Event::Closed => break 'mainloop,
                sfml::window::Event::KeyReleased { code: sfml::window::Key::Escape, .. } => {
                    break 'mainloop
                }
                _ => (),
            }
        }

        let ai = audio_info.read().expect("Couldn't read audio info");

        window.clear(&sfml::graphics::Color::rgb(0x18, 0x15, 0x11));

        let factor = if right_channel_middle { 1.0 } else { 2.0 };
        let offset = if right_channel_middle { -0.01 } else { 1.0 };

        for si in 0..display_columns {
            //let reshape = ((si as f32) / (display_columns as f32)).sqrt();
            let reshape = 1.0; // - 1.0 / ((si as f32) / (display_columns as f32) * 10.0).exp();
            let vl = ai.spectrum_left[si] * reshape;
            let size = vl * factor;
            rect.set_size((width, size));
            if symmetric {
                rect.set_position((
                    1.0 - (si as f32 / display_columns as f32) - 1.0,
                    -size / 2.0,
                ));
            } else {
                rect.set_position((
                    2.0 * (si as f32 / display_columns as f32) - 1.0,
                    -size + offset,
                ));
            }
            if si % 2 == right_channel_color_offset as usize {
                rect.set_fill_color(&c1);
            } else {
                rect.set_fill_color(&c2);
            }
            window.draw(&rect);
            if right_channel || symmetric {
                let vr = ai.spectrum_right[si] * reshape;
                let size = vr * factor;
                rect.set_size((width, size));
                if symmetric {
                    rect.set_position(((si as f32 / display_columns as f32), -size / 2.0));
                } else {
                    rect.set_position((2.0 * (si as f32 / display_columns as f32) - 1.0, -offset));
                }
                if si % 2 == 0 {
                    rect.set_fill_color(&c1);
                } else {
                    rect.set_fill_color(&c2);
                }
                window.draw(&rect);
            }
        }

        window.display();
        ::std::thread::sleep(::std::time::Duration::from_millis(25));
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/sfml.toml", visualizer);
}
