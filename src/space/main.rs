extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;

#[macro_use]
extern crate glium;
extern crate nalgebra as na;
extern crate rand;
extern crate image;
extern crate eagre_ecs as ecs;
extern crate obj;

use glium::glutin;

#[macro_use]
pub mod macros;

mod entities;
mod components;
mod info;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
    mut run_mode: framework::RunMode,
) {
    let display_columns = config
        .get("DISPLAY_COLUMNS")
        .map(|v| {
            v.as_integer().expect("DISPLAY_COLUMNS must be an integer")
        })
        .unwrap_or(20) as usize;
    info!("DISPLAY_COLUMNS = {}", display_columns);
    let display_columns = display_columns - 1;

    let window_height = config
        .get("WINDOW_HEIGHT")
        .map(|v| {
            v.as_integer().expect("WINDOW_HEIGHT must be an integer")
        })
        .unwrap_or(720) as u32;
    info!("WINDOW_HEIGHT = {}", window_height);

    let window_width = config
        .get("WINDOW_WIDTH")
        .map(|v| v.as_integer().expect("WINDOW_WIDTH must be an integer"))
        .unwrap_or(1280) as u32;
    info!("WINDOW_WIDTH = {}", window_width);

    let beat_columns = config
        .get("SPACE_BEAT_COLS")
        .map(|a| {
            a.as_array()
                .expect("SPACE_BEAT_COLS must be an array")
                .iter()
                .map(|t| {
                    let table = t.as_table().expect(
                        "SPACE_BEAT_COLS must be an array of tables",
                    );
                    (
                        table
                            .get("c")
                            .expect("SPACE_BEAT_COLS element is missing \"c\" (column)")
                            .as_integer()
                            .expect("SPACE_BEAT_COLS element \"c\" must be an integer") as
                            usize,
                        table
                            .get("s")
                            .expect("SPACE_BEAT_COLS element is missing \"s\" (sensitivity)")
                            .as_float()
                            .expect("SPACE_BEAT_COLS element \"s\" must be a float") as
                            f32,
                    )
                })
                .collect()
        })
        .unwrap_or(vec![(16usize, 0.25)]);
    info!("SPACE_BEAT_COLS = {:?}", beat_columns);

    let beat_min_volume = config
        .get("SPACE_BEAT_MINVOLUME")
        .map(|v| {
            v.as_float().expect("SPACE_BEAT_MINVOLUME must be a float")
        })
        .unwrap_or(70.0) as f32;
    info!("SPACE_BEAT_MINVOLUME = {}", beat_min_volume);

    let timeout = config
        .get("SPACE_SHIP_TIMEOUT")
        .map(|v| v.as_float().expect("SPACE_SHIP_TIMEOUT must be a float"))
        .unwrap_or(0.06) as f32;
    info!("SPACE_SHIP_TIMEOUT = {}s", timeout);

    let mut events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_width, window_height)
        .with_maximized(if let framework::RunMode::Live = run_mode {
            true
        } else {
            false
        })
        .with_decorations(false)
        .with_fullscreen(Some(events_loop.get_primary_monitor()))
        .with_title("PulseAudio Visualizer - Space");

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_multisampling(0);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let tex_color1 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();

    let depth_texture1 = glium::texture::DepthTexture2d::empty_with_format(
        &display,
        glium::texture::DepthFormat::F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();

    let output1 = &[("frg_color", &tex_color1)];

    let mut framebuffer1 = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(
        &display,
        output1.iter().cloned(),
        &depth_texture1,
    ).unwrap();

    let tex_color2 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();

    let depth_texture2 = glium::texture::DepthTexture2d::empty_with_format(
        &display,
        glium::texture::DepthFormat::F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();

    let output2 = &[("frg_color", &tex_color2)];

    let mut framebuffer2 = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(
        &display,
        output2.iter().cloned(),
        &depth_texture2,
    ).unwrap();

    // Postprocess screen rect
    let quad_vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 4],
            texcoord: [f32; 2],
        }

        implement_vertex!(Vertex, position, texcoord);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 0.0, 1.0],
                    texcoord: [1.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0, 0.0, 1.0],
                    texcoord: [1.0, 1.0],
                },
                Vertex {
                    position: [-1.0, 1.0, 0.0, 1.0],
                    texcoord: [0.0, 1.0],
                },
            ],
        ).unwrap()
    };

    let quad_index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2, 0, 2, 3],
    ).unwrap();

    let fxaa_program = shader_program!(&display, "shaders/postprocess.vert", "shaders/fxaa.frag");

    let mut system = ecs::System::new();

    let station_ent = entities::Station::create(&mut system, &display, config.clone());

    let start_time = ::std::time::Instant::now();

    let perspective = na::Matrix4::new_perspective(
        window_width as f32 / window_height as f32,
        ::std::f32::consts::PI / 3.0,
        0.0001,
        100.0,
    );

    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(1.0, 2.0, 2.0),
        &na::Point3::new(0.0, 0.0, 0.0),
        &na::Vector3::new(0.0, 1.0, 0.0),
    );

    let mut volume: f32 = 0.0;
    let mut previous_time: f32 = 0.0;
    let mut is_beat_previous = vec![false; beat_columns.len()];

    let mut last_ship: f32 = 0.0;
    let mut do_ship = false;

    'mainloop: loop {
        use glium::Surface;

        let frame_time = ::std::time::Instant::now();
        let time = if let framework::RunMode::Rendering(ref mut render_info) = run_mode {
            render_info.frame as f32 * render_info.frame_time
        } else {
            let t = frame_time.duration_since(start_time);
            t.as_secs() as f32 + t.subsec_nanos() as f32 * 1e-9
        };

        // Read audio info
        let (beat, is_beat) = {
            let ai = audio_info.read().expect("Couldn't read audio info");
            let mut current_volume = 0.0;
            let max_l = ai.raw_spectrum_left
                .iter()
                .map(|f| {
                    current_volume += f;
                    f
                })
                .cloned()
                .fold(0. / 0., f32::max)
                .max(beat_min_volume);
            let max_r = ai.raw_spectrum_right
                .iter()
                .map(|f| {
                    current_volume += f;
                    f
                })
                .cloned()
                .fold(0. / 0., f32::max)
                .max(beat_min_volume);
            volume = volume.max(current_volume / ai.raw_spectrum_left.len() as f32 / 2.0);
            let is_beat = beat_columns
                .iter()
                .map(|&(c, s)| {
                    (ai.raw_spectrum_left[c] / max_l + ai.raw_spectrum_right[c] / max_r) / 2.0 > s
                })
                .collect::<Vec<bool>>();
            ((ai.columns_left[1] + ai.columns_right[1]) / 2.0, is_beat)
        };

        let mut beat2 = false;
        if (time - last_ship) > timeout {
            for (current, previous) in is_beat.iter().zip(is_beat_previous.iter()) {
                if *current == true && *previous == false {
                    do_ship = true;
                    beat2 = true;
                    break;
                }
            }
        }

        let inf = info::Info {
            time,
            delta: time - previous_time,
            perspective,
            view,
            beat,
            volume,
            is_beat,
            is_beat_previous,
            beat2,

            station: station_ent,
        };

        if do_ship {
            entities::ShipInbound::create(&mut system, &display, &inf);
            last_ship = time;
            do_ship = false;
        }

        // Clear both framebuffers
        framebuffer1.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        framebuffer2.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        let uniforms1 =
            uniform! {
            tex_color: tex_color1.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Mirror),
            resolution: [window_width as f32, window_height as f32],
        };

        let uniforms2 =
            uniform! {
            tex_color: tex_color2.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Mirror),
            resolution: [window_width as f32, window_height as f32],
        };


        components::updateable::update(&mut system, &inf);
        components::physics::update(&mut system, &inf);

        components::drawable::draw(&system, &inf, &mut framebuffer1);

        // Postprocess
        framebuffer2
            .draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &fxaa_program,
                &uniforms1,
                &Default::default(),
            )
            .unwrap();

        let target = display.draw();
        let dims = target.get_dimensions();
        target.blit_from_simple_framebuffer(
            &tex_color2.as_surface(),
            &glium::Rect {
                left: 0,
                bottom: 0,
                width: window_width,
                height: window_height,
            },
            &glium::BlitTarget {
                left: 0,
                bottom: 0,
                width: dims.0 as i32,
                height: dims.1 as i32,
            },
            glium::uniforms::MagnifySamplerFilter::Linear,
        );
        target.finish().unwrap();

        // Event handling
        let mut running = true;
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            state: glutin::ElementState::Pressed,
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            state: glutin::ElementState::Pressed,
                            virtual_keycode: Some(_),
                            ..
                        },
                        ..
                    } => { entities::ShipOutbound::create(&mut system, &display, &inf); },
                    _ => (),
                }
            }
            _ => (),
        });
        if !running {
            break 'mainloop;
        }

        previous_time = time;
        is_beat_previous = inf.is_beat;

        if let framework::RunMode::Rendering(ref mut render_info) = run_mode {
            let image: glium::texture::RawImage2d<u8> = display.read_front_buffer();
            let image =
                image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned())
                    .unwrap();
            let image = image::DynamicImage::ImageRgba8(image).flipv();
            let mut output = std::fs::File::create(&std::path::Path::new(&format!(
                "{}/{:06}.png",
                render_info.outdir,
                render_info.frame
            ))).unwrap();
            image.save(&mut output, image::ImageFormat::PNG).unwrap();
        }
        framework::sleep(&mut run_mode);
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/space.toml", visualizer);
}
