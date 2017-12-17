extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate sfml;
extern crate toml;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
    mut run_mode: framework::RunMode,
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

    let mut render_texture = sfml::graphics::RenderTexture::new(window_width, window_height, true)
        .unwrap();
    render_texture.set_view(&sfml::graphics::View::new(
        sfml::system::Vector2::new(0.0, 0.0),
        sfml::system::Vector2::new(2.2, 2.2),
    ));

    let mut vertex_array_left_top = Vec::with_capacity(display_columns);
    let mut vertex_array_right_top = Vec::with_capacity(display_columns);
    let mut vertex_array_left_bottom = Vec::with_capacity(display_columns);
    let mut vertex_array_right_bottom = Vec::with_capacity(display_columns);

    let mut vertex_array_left_ttb = Vec::with_capacity(display_columns * 2);
    let mut vertex_array_right_ttb = Vec::with_capacity(display_columns * 2);

    for i in 0..display_columns {
        vertex_array_left_top.push(sfml::graphics::Vertex::with_pos(
            (1.0 - (i as f32 / display_columns as f32) - 1.0, 0.0),
        ));
        vertex_array_right_top.push(sfml::graphics::Vertex::with_pos(
            ((i as f32 / display_columns as f32), 0.0),
        ));
        vertex_array_left_bottom.push(sfml::graphics::Vertex::with_pos(
            (1.0 - (i as f32 / display_columns as f32) - 1.0, 0.0),
        ));
        vertex_array_right_bottom.push(sfml::graphics::Vertex::with_pos(
            ((i as f32 / display_columns as f32), 0.0),
        ));

        vertex_array_left_ttb.push(sfml::graphics::Vertex::with_pos(
            (1.0 - (i as f32 / display_columns as f32) - 1.0, 0.0),
        ));
        vertex_array_left_ttb.push(sfml::graphics::Vertex::with_pos(
            (1.0 - (i as f32 / display_columns as f32) - 1.0, 0.0),
        ));
        vertex_array_right_ttb.push(sfml::graphics::Vertex::with_pos(
            ((i as f32 / display_columns as f32), 0.0),
        ));
        vertex_array_right_ttb.push(sfml::graphics::Vertex::with_pos(
            ((i as f32 / display_columns as f32), 0.0),
        ));
    }

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

        {
            let ai = audio_info.read().expect("Couldn't read audio info");

            render_texture.clear(&sfml::graphics::Color::rgb(0x18, 0x15, 0x11));

            let factor = 1.0;

            for si in 0..display_columns {
                let reshape = 1.0; // - 1.0 / ((si as f32) / (display_columns as f32) * 10.0).exp();

                let vl = ai.columns_left[si] * reshape;
                let size = vl * factor;
                vertex_array_left_top[si].position.y = -size / 2.0;
                vertex_array_left_bottom[si].position.y = size / 2.0;

                vertex_array_left_ttb[si * 2].position.y = -size / 2.0;
                vertex_array_left_ttb[si * 2 + 1].position.y = size / 2.0;

                let vr = ai.columns_right[si] * reshape;
                let size = vr * factor;
                vertex_array_right_top[si].position.y = -size / 2.0;
                vertex_array_right_bottom[si].position.y = size / 2.0;

                vertex_array_right_ttb[si * 2].position.y = -size / 2.0;
                vertex_array_right_ttb[si * 2 + 1].position.y = size / 2.0;
            }
        }
        render_texture.draw_primitives(
            &vertex_array_left_top[1..],
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );
        render_texture.draw_primitives(
            &vertex_array_right_top[1..],
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );
        render_texture.draw_primitives(
            &vertex_array_left_bottom[1..],
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );
        render_texture.draw_primitives(
            &vertex_array_right_bottom[1..],
            sfml::graphics::PrimitiveType::LineStrip,
            sfml::graphics::RenderStates::default(),
        );

        render_texture.draw_primitives(
            &vertex_array_left_ttb,
            sfml::graphics::PrimitiveType::Lines,
            sfml::graphics::RenderStates::default(),
        );
        render_texture.draw_primitives(
            &vertex_array_right_ttb,
            sfml::graphics::PrimitiveType::Lines,
            sfml::graphics::RenderStates::default(),
        );

        render_texture.display();
        window.clear(&sfml::graphics::Color::rgb(0x18, 0x15, 0x11));
        let sprite = sfml::graphics::Sprite::with_texture(render_texture.texture());
        window.draw(&sprite);
        window.display();
        if let framework::RunMode::Rendering(ref mut render_info) = run_mode {
            let img = render_texture.texture().copy_to_image().unwrap();
            img.save_to_file(&format!(
                "{}/{:06}.png",
                render_info.outdir,
                render_info.frame
            ));
        }
        framework::sleep(&mut run_mode);
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/sfml.toml", visualizer);
}
