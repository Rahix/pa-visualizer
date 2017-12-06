//! Based on the Demo "No Ambition" by "Quite & T-Rex"
//! Link: http://www.pouet.net/prod.php?which=6973
extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate sfml;
extern crate toml;

#[macro_use]
extern crate glium;
extern crate nalgebra as na;

use glium::glutin;

macro_rules! shader_program {
    ($display:expr, $vert_file:expr, $frag_file:expr) => ({
        // Use this for debug
        let vert_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(format!("src/noambition/{}", $vert_file)).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        let frag_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(format!("src/noambition/{}", $frag_file)).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        glium::Program::from_source($display,
                &vert_src,
                &frag_src,
                None).unwrap()
        // Use this for release
        //glium::Program::from_source($display,
        //    include_str!($vert_file),
        //    include_str!($frag_file),
        //    None).unwrap()
    })
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

implement_vertex!(Vertex, position, color);

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
    let display_columns = display_columns - 1;

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

    let rows = config
        .get("NA_ROWS")
        .map(|v| v.as_integer().expect("NA_ROWS must be an integer"))
        .unwrap_or(100) as usize;
    info!("NA_ROWS = {}", rows);

    let base_height = config
        .get("NA_BASE_HEIGHT")
        .map(|v| v.as_float().expect("NA_BASE_HEIGHT must be a float"))
        .unwrap_or(0.5) as f32;
    info!("NA_BASE_HEIGHT = {}", base_height);

    let amplitude_top = config
        .get("NA_AMPLITUDE_TOP")
        .map(|v| v.as_float().expect("NA_AMPLITUDE_TOP must be a float"))
        .unwrap_or(0.5) as f32;
    info!("NA_AMPLITUDE_TOP = {}", amplitude_top);

    let cam_height = config
        .get("NA_CAM_HEIGHT")
        .map(|v| v.as_float().expect("NA_CAM_HEIGHT must be a float"))
        .unwrap_or(1.0) as f32;
    info!("NA_CAM_HEIGHT = {}", cam_height);

    let amplitude_bottom = config
        .get("NA_AMPLITUDE_BOTTOM")
        .map(|v| {
            v.as_float().expect("NA_AMPLITUDE_BOTTOM must be a float")
        })
        .unwrap_or(0.5) as f32;
    info!("NA_AMPLITUDE_BOTTOM = {}", amplitude_bottom);

    let mid_dist = config
        .get("NA_MID_DISTANCE")
        .map(|v| v.as_float().expect("NA_MID_DISTANCE must be a float"))
        .unwrap_or(0.5) as f32;
    info!("NA_MID_DISTANCE = {}", mid_dist);

    let speed = config
        .get("NA_SPEED")
        .map(|v| v.as_float().expect("NA_SPEED must be a float"))
        .unwrap_or(1.0) as f32;
    info!("NA_SPEED = {}", speed);

    let depth = config
        .get("NA_DEPTH")
        .map(|v| v.as_float().expect("NA_DEPTH must be a float"))
        .unwrap_or(30.0) as f32;
    info!("NA_DEPTH = {}", depth);

    let mut events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_width, window_height)
        .with_maximized(true)
        .with_decorations(false)
        //.with_fullscreen(Some(events_loop.get_primary_monitor()))
        .with_title("PulseAudio Visualizer - No Ambition");

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_multisampling(0);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let tex_position1 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();
    let tex_screen_position1 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();
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

    let output1 = &[
        ("fg_position", &tex_position1),
        ("fg_screen_position", &tex_screen_position1),
        ("fg_color", &tex_color1),
    ];

    let mut framebuffer1 = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(
        &display,
        output1.iter().cloned(),
        &depth_texture1,
    ).unwrap();


    let tex_position2 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
    ).unwrap();
    let tex_screen_position2 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        window_width,
        window_height,
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

    let output1 = &[
        ("fg_position", &tex_position2),
        ("fg_screen_position", &tex_screen_position2),
        ("fg_color", &tex_color2),
    ];

    let mut framebuffer2 = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(
        &display,
        output1.iter().cloned(),
        &depth_texture2,
    ).unwrap();

    let prepass_program = shader_program!(&display, "shaders/prepass.vert", "shaders/prepass.frag");
    let background_program = shader_program!(
        &display,
        "shaders/postprocess.vert",
        "shaders/background.frag"
    );
    let gauss_program = shader_program!(&display, "shaders/postprocess.vert", "shaders/gauss.frag");
    let bokeh_program = shader_program!(&display, "shaders/postprocess.vert", "shaders/bokeh.frag");
    let final_program = shader_program!(&display, "shaders/postprocess.vert", "shaders/final.frag");

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


    let mut lines_buffers = {
        let mut v_buf = Vec::with_capacity(rows * display_columns * 4);
        let mut i_buf = Vec::with_capacity(rows * display_columns * 4);
        let mut index = 0u16;
        let color = [7.0 / 255.0, 5.0 / 255.0, 40.0 / 255.0, 0.8];
        for row in 0..rows {
            //Left
            for column in 0..display_columns {
                let x = -(column as f32 / (display_columns - 1) as f32 * 10.0) - mid_dist;
                let y = row as f32 / (rows - 1) as f32 * depth;
                v_buf.push(Vertex {
                    position: [x, y, - base_height / 2.0, 1.0],
                    color: color,
                });
                v_buf.push(Vertex {
                    position: [x, y, base_height / 2.0, 1.0],
                    color: color,
                });
                i_buf.push(index);
                index += 1;
                i_buf.push(index);
                index += 1;
            }

            //Right
            for column in 0..display_columns {
                let x = column as f32 / (display_columns - 1) as f32 * 10.0 + mid_dist;
                let y = row as f32 / (rows - 1) as f32 * depth;
                v_buf.push(Vertex {
                    position: [x, y, - base_height / 2.0, 1.0],
                    color: color,
                });
                v_buf.push(Vertex {
                    position: [x, y, base_height / 2.0, 1.0],
                    color: color,
                });
                i_buf.push(index);
                index += 1;
                i_buf.push(index);
                index += 1;
            }
        }

        (
            glium::VertexBuffer::dynamic(&display, &v_buf).unwrap(),
            glium::IndexBuffer::new(&display, glium::index::PrimitiveType::LinesList, &i_buf)
                .unwrap(),
            v_buf,
        )
    };

    let mut points_buffers = {
        let mut v_buf = Vec::with_capacity(rows * display_columns * 4);
        let mut i_buf = Vec::with_capacity(rows * display_columns * 4);
        let mut index = 0u16;
        let color = [10.0 / 255.0, 10.0 / 255.0, 50.0 / 255.0, 1.0];
        for row in 0..rows {
            //Left
            for column in 0..display_columns {
                let x = -(column as f32 / (display_columns - 1) as f32 * 10.0) - mid_dist;
                let y = row as f32 / (rows - 1) as f32 * depth;
                v_buf.push(Vertex {
                    position: [x, y, - base_height / 2.0, 1.0],
                    color: color,
                });
                v_buf.push(Vertex {
                    position: [x, y, base_height / 2.0, 1.0],
                    color: color,
                });
                i_buf.push(index);
                index += 1;
                i_buf.push(index);
                index += 1;
            }

            //Right
            for column in 0..display_columns {
                let x = column as f32 / (display_columns - 1) as f32 * 10.0 + mid_dist;
                let y = row as f32 / (rows - 1) as f32 * depth;
                v_buf.push(Vertex {
                    position: [x, y, - base_height / 2.0, 1.0],
                    color: color,
                });
                v_buf.push(Vertex {
                    position: [x, y, base_height / 2.0, 1.0],
                    color: color,
                });
                i_buf.push(index);
                index += 1;
                i_buf.push(index);
                index += 1;
            }
        }

        (
            glium::VertexBuffer::dynamic(&display, &v_buf).unwrap(),
            glium::IndexBuffer::new(&display, glium::index::PrimitiveType::Points, &i_buf).unwrap(),
            v_buf,
        )
    };



    let perspective = na::Matrix4::new_perspective(
        window_width as f32 / window_height as f32,
        ::std::f32::consts::PI / 4.0,
        0.0001,
        100.0,
    );

    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(0.0, -1.0, cam_height),
        &na::Point3::new(0.0, 10.0, cam_height),
        &na::Vector3::new(0.0, 0.0, 1.0),
    );

    let alter_row = rows * 3 / 4;
    let row_size = display_columns * 4;
    let row_distance = 30.0 / (rows - 1) as f32;
    let row_time = row_distance / speed;
    let offset_left = row_size * alter_row;
    let offset_right = row_size * alter_row + display_columns * 2;
    let mut previous_offset = 0.0;
    let mut row_buffer = Vec::with_capacity(display_columns * 4);
    let mut accumulate_buffer = (
        vec![0.0; display_columns + 1],
        vec![0.0; display_columns + 1],
    );

    let start_time = ::std::time::Instant::now();

    'mainloop: loop {
        use glium::Surface;

        let frame_time = ::std::time::Instant::now();
        let time = {
            let t = frame_time.duration_since(start_time);
            t.as_secs() as f32 + t.subsec_nanos() as f32 * 1e-9
        };

        let offset = speed * (time % row_time);

        let model = na::Translation3::from_vector(na::Vector3::new(0.0, -offset, 0.0))
            .to_homogeneous();

        let beat = {
            let ai = audio_info.read().expect("Couldn't read audio info");
            for i in 0..display_columns {
                let fact = i as f32 / display_columns as f32 * 5.0 + 1.0;
                accumulate_buffer.0[i] = f32::max(accumulate_buffer.0[i], ai.spectrum_left[i] * fact);
                accumulate_buffer.1[i] = f32::max(accumulate_buffer.1[i], ai.spectrum_right[i] * fact);
            }
            (ai.spectrum_left[1] + ai.spectrum_right[1]) / 2.0
        };

        if previous_offset > offset {
            // We are jumping this frame
            // Save first row
            row_buffer.clear();
            for i in 0..(display_columns * 4) {
                row_buffer.push((
                    lines_buffers.2[i].position[2],
                    points_buffers.2[i].position[2],
                ));
            }
            // Rotate buffers
            for row in 0..(rows - 1) {
                let index_offset = row_size * row;
                let index_offset_next = row_size * (row + 1);
                // Copy positions from next row
                for i in 0..(display_columns * 4) {
                    lines_buffers.2[i + index_offset].position[2] =
                        lines_buffers.2[i + index_offset_next].position[2];
                    points_buffers.2[i + index_offset].position[2] =
                        points_buffers.2[i + index_offset_next].position[2];
                }
            }
            // Write first row into last row
            let last_row_offset = row_size * (rows - 1);
            for i in 0..(display_columns * 4) {
                lines_buffers.2[i + last_row_offset].position[2] = row_buffer[i].0;
                points_buffers.2[i + last_row_offset].position[2] = row_buffer[i].1;
            }
            // Write new data
            for c in 0..display_columns {
                let c2 = c * 2;
                lines_buffers.2[c2 + offset_left].position[2] =
                    -accumulate_buffer.0[c + 1] * amplitude_bottom -
                        base_height / 2.0;
                lines_buffers.2[c2 + 1 + offset_left].position[2] =
                    accumulate_buffer.0[c + 1] * amplitude_top +
                        base_height / 2.0;
                lines_buffers.2[c2 + offset_right].position[2] =
                    -accumulate_buffer.1[c + 1] * amplitude_bottom -
                        base_height / 2.0;
                lines_buffers.2[c2 + 1 + offset_right].position[2] =
                    accumulate_buffer.1[c + 1] * amplitude_top +
                        base_height / 2.0;

                points_buffers.2[c2 + offset_left].position[2] =
                    -accumulate_buffer.0[c + 1] * amplitude_bottom -
                        base_height / 2.0;
                points_buffers.2[c2 + 1 + offset_left].position[2] =
                    accumulate_buffer.0[c + 1] * amplitude_top +
                        base_height / 2.0;
                points_buffers.2[c2 + offset_right].position[2] =
                    -accumulate_buffer.1[c + 1] * amplitude_bottom -
                        base_height / 2.0;
                points_buffers.2[c2 + 1 + offset_right].position[2] =
                    accumulate_buffer.1[c + 1] * amplitude_top +
                        base_height / 2.0;
            }

            // Clear buffer
            for i in 0..display_columns {
                accumulate_buffer.0[i] = 0.0;
                accumulate_buffer.1[i] = 0.0;
            }

            lines_buffers.0.write(&lines_buffers.2);
            points_buffers.0.write(&points_buffers.2);
        }

        {
            // Print framerate
            //let d = time - previous_time;
            //debug!("{} fps", 1.0 / d);
        }

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
        };

        let draw_params = glium::DrawParameters {
            line_width: Some(1.0),
            point_size: Some(2.0),
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            ..Default::default()
        };
        framebuffer1.clear_color(0.0, 0.0, 0.0, 0.0);
        framebuffer2.clear_color(0.0, 0.0, 0.0, 0.0);
        framebuffer1
            .draw(
                &lines_buffers.0,
                &lines_buffers.1,
                &prepass_program,
                &uniforms,
                &draw_params,
            )
            .unwrap();
        framebuffer1
            .draw(
                &points_buffers.0,
                &points_buffers.1,
                &prepass_program,
                &uniforms,
                &draw_params,
            )
            .unwrap();

        let uniforms1 =
            uniform! {
            tex_position: &tex_position1,
            tex_screen_position: &tex_screen_position1,
            tex_color: &tex_color1,
            time: time,
            beat: beat,
            resolution: [window_width as f32, window_height as f32],
        };

        let uniforms2 =
            uniform! {
            tex_position: &tex_position2,
            tex_screen_position: &tex_screen_position2,
            tex_color: &tex_color2,
            time: time,
            beat: beat,
            resolution: [window_width as f32, window_height as f32],
        };

        // Background pass
        framebuffer2
            .draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &background_program,
                &uniforms1,
                &Default::default(),
            )
            .unwrap();

        // Bokeh pass
        framebuffer1
            .draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &bokeh_program,
                &uniforms2,
                &Default::default(),
            )
            .unwrap();

        // Blur pass
        framebuffer2
            .draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &gauss_program,
                &uniforms1,
                &Default::default(),
            )
            .unwrap();

        // Final render pass
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &final_program,
                &uniforms2,
                &Default::default(),
            )
            .unwrap();
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
                    _ => (),
                }
            }
            _ => (),
        });
        if !running {
            break 'mainloop;
        }

        previous_offset = offset;

        ::std::thread::sleep(::std::time::Duration::from_millis(10));
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/noambition.toml", visualizer);
}
