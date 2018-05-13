use ecs;
use glium;
use std::rc;
use na;

use info;
use components;
use CONFIG;

#[derive(Debug)]
pub struct RingSharedData {
    program: glium::Program,
    vbuf: glium::VertexBuffer<Vertex>,
    ibuf: glium::IndexBuffer<u16>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 4],
    pub idx: f32,
}

implement_vertex!(Vertex, position, idx);

pub type RingShared = rc::Rc<RingSharedData>;

#[derive(Debug, Clone)]
pub struct Ring {
    radius: f32,
    volume: f32,
    listen_columns: (usize, usize),
    color: na::Vector4<f32>,
    shared: RingShared,
}

impl Ring {
    pub fn create_multiple(sys: &mut ecs::System, display: &glium::Display, num: usize) {
        let columns = ezconf_int!(CONFIG: "WINDOW_SIZE", 2048) as usize / 2;
        let columns_1 = columns / num;
        for i in 1..num {
            let t = i as f32 / (num - 1) as f32;
            Ring::create(
                sys,
                display,
                t,
                ezconf_float!(CONFIG: "ring.radius_base", 13.0) as f32
                + ezconf_float!(CONFIG: "ring.width", 5.0) as f32 * t,
                (i * columns_1, (i + 1) * columns_1),
            );
        }
    }

    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        t: f32,
        radius: f32,
        listen_columns: (usize, usize),
    ) -> ecs::Entity {
        let ent = sys.new_entity();

        let shared = if let Ok(ents) = sys.entities_with::<RingShared>() {
            sys.get::<RingShared>(ents[0]).unwrap()
        } else {
            let shared_ent = sys.new_entity();

            let num_verts = ezconf_int!(CONFIG: "ring.num_verts", 24) as usize;

            let vertices_buf = (0..num_verts)
                .map(|i| {
                    let index = i as f32 / num_verts as f32;
                    let angle = index * ::std::f32::consts::PI * 2.0;
                    Vertex {
                        idx: index,
                        position: [angle.cos(), 0.0, angle.sin(), 1.0],
                    }
                })
                .collect::<Vec<Vertex>>();

            let vbuf = glium::VertexBuffer::new(display, &vertices_buf).unwrap();

            let ibuf = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::LineLoop,
                &(0..num_verts as u16).collect::<Vec<u16>>(),
            ).unwrap();

            let shared = rc::Rc::new(RingSharedData {
                program: shader_program_ent!(display, "shaders/ring.vert", "shaders/ring.frag"),
                vbuf,
                ibuf,
            });

            sys.add(shared_ent, shared.clone()).unwrap();

            shared
        };

        sys.add(
            ent,
            components::Position(na::Point3::new(
                ezconf_float!(CONFIG: "planet.x", 5.8) as f32,
                ezconf_float!(CONFIG: "planet.y", -6.4) as f32,
                ezconf_float!(CONFIG: "planet.z", -14.0) as f32,
            )),
        ).unwrap();

        let color1 = na::Vector4::new(0.071111, 1.000000, 0.244152, 1.0);
        let color2 = na::Vector4::new(0.764767, 0.878447, 0.172795, 1.0);

        let ring = Ring {
            radius,
            listen_columns,
            volume: 0.0,
            color: t * color1 + (1.0 - t) * color2,
            shared,
        };

        sys.add(ent, ring).unwrap();

        sys.add(ent, components::Updateable::new(Ring::update))
            .unwrap();
        sys.add(ent, components::Drawable::new(Ring::draw)).unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        let spectrum = info.spectrum.unwrap();

        let ring = sys.borrow_mut::<Ring>(ent).unwrap();

        let mut acc = 0.0;
        for i in ring.listen_columns.0..ring.listen_columns.1 {
            acc += spectrum.0[i] + spectrum.1[i];
        }
        let acc = acc / 2.0;

        let new_volume = acc / 760.0 * ezconf_float!(CONFIG: "ring.amplitude", 1.0) as f32;

        ring.volume = new_volume.max(
            ring.volume * ezconf_float!(CONFIG: "ring.dampen", 0.99) as f32,
        );
    }

    pub fn draw(
        sys: &ecs::System,
        ent: ecs::Entity,
        info: &info::Info,
        target: &mut glium::framebuffer::MultiOutputFrameBuffer,
    ) {
        use glium::Surface;

        let r = sys.borrow::<Ring>(ent).unwrap();

        let model = na::Translation3::from_vector(na::Vector3::new(
            ezconf_float!(CONFIG: "planet.x", 5.8) as f32,
            ezconf_float!(CONFIG: "planet.y", -6.4) as f32,
            ezconf_float!(CONFIG: "planet.z", -14.0) as f32,
        )).to_homogeneous() *
            na::Rotation3::new(na::Vector3::new(
                ezconf_float!(CONFIG: "ring.rot_x", 0.0) as f32,
                ezconf_float!(CONFIG: "ring.rot_y", 0.0) as f32,
                ezconf_float!(CONFIG: "ring.rot_z", 0.0) as f32,
            )).to_homogeneous() * na::Matrix4::new_scaling(r.radius);


        let camera_position = na::Point3::new(
            3.0,
            ezconf_float!(CONFIG: "SPACE_CAMERA_HEIGHT", 0.0) as f32,
            2.0,
        );
        let view = na::Matrix4::look_at_rh(
            &camera_position,
            &(camera_position + na::Vector3::new(-2.0, 0.0, -1.0)),
            &na::Vector3::new(0.0, 1.0, 0.0),
        );

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
            color: [r.color.x, r.color.y, r.color.z, r.color.w],
            volume: r.volume,
            time: info.time,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            polygon_mode: glium::PolygonMode::Line,
            line_width: Some(ezconf_float!(CONFIG: "ring.thickness", 2.0) as f32),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target
            .draw(
                &r.shared.vbuf,
                &r.shared.ibuf,
                &r.shared.program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
}
