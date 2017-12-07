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
        sfml::system::Vector2::new(
            2.2 / window_height as f32 * window_width as f32,
            2.2,
        ),
    ));

    let mut vertex_array_top = Vec::with_capacity(display_columns);
    let mut vertex_array_bottom = Vec::with_capacity(display_columns);
    let mut vertex_array_ttb = Vec::with_capacity(display_columns * 2);

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

        let factor = 0.3;
        let radius = 0.6;

        vertex_array_top.clear();
        vertex_array_bottom.clear();
        vertex_array_ttb.clear();

        // Right
        for si in 0..display_columns {
            let reshape = 1.0; // - 1.0 / ((si as f32) / (display_columns as f32) * 10.0).exp();


            let angle = si as f32 / (display_columns * 2 - 2) as f32 * 2.0 *
                ::std::f32::consts::PI - ::std::f32::consts::PI / 2.0;

            let vr = ai.columns_right[si] * reshape;
            let size = vr * factor;
            vertex_array_top.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius + size),
                angle.sin() * (radius + size),
            )));
            vertex_array_bottom.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius - size),
                angle.sin() * (radius - size),
            )));

            vertex_array_ttb.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius + size),
                angle.sin() * (radius + size),
            )));
            vertex_array_ttb.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius - size),
                angle.sin() * (radius - size),
            )));
        }

        // Left
        for si in 0..display_columns {
            let reshape = 1.0; // - 1.0 / ((si as f32) / (display_columns as f32) * 10.0).exp();


            let angle = (si) as f32 / (display_columns * 2 - 2) as f32 * 2.0 *
                ::std::f32::consts::PI + ::std::f32::consts::PI / 2.0;

            let vl = ai.columns_left[display_columns - si - 1] * reshape;
            let size = vl * factor;
            vertex_array_top.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius + size),
                angle.sin() * (radius + size),
            )));
            vertex_array_bottom.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius - size),
                angle.sin() * (radius - size),
            )));

            vertex_array_ttb.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius + size),
                angle.sin() * (radius + size),
            )));
            vertex_array_ttb.push(sfml::graphics::Vertex::with_pos((
                angle.cos() * (radius - size),
                angle.sin() * (radius - size),
            )));
        }
        window.draw_primitives(
            &vertex_array_top,
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );
        window.draw_primitives(
            &vertex_array_bottom,
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );

        window.draw_primitives(
            &vertex_array_ttb,
            sfml::graphics::PrimitiveType::Lines,
            sfml::graphics::RenderStates::default(),
        );

        window.display();
        ::std::thread::sleep(::std::time::Duration::from_millis(25));
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/sfml.toml", visualizer);
}
