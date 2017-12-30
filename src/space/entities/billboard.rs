use ecs;
use glium;
use std::rc;
use na;

use components;
use info;

#[derive(Debug)]
struct BillboardSharedData {
    program: glium::Program,
    vbuf: glium::VertexBuffer<Vertex>,
    ibuf: glium::IndexBuffer<u16>,
}

type BillboardShared = rc::Rc<BillboardSharedData>;

#[derive(Debug, Clone)]
pub struct Billboard {
    width: f32,
    height: f32,
    shared: BillboardShared,
    start_time: f32,
    prog: Option<rc::Rc<glium::Program>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

impl Billboard {
    pub fn create(sys: &mut ecs::System,
            ent_opt: Option<ecs::Entity>,
            display: &glium::Display,
            width: f32, height: f32,
            info: &info::Info,
            prog: Option<rc::Rc<glium::Program>>,
    ) -> ecs::Entity {
        let ent = if let Some(e) = ent_opt { e } else { sys.new_entity() };

        let shared = if let Ok(ents) = sys.entities_with::<BillboardShared>() {
            sys.get::<BillboardShared>(ents[0]).unwrap()
        } else {
            let shared_ent = sys.new_entity();

            let vbuf = glium::VertexBuffer::new(display, &vec![
                Vertex {
                    position: [-1.0, -1.0], tex_coord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0], tex_coord: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0], tex_coord: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0], tex_coord: [1.0, 0.0],
                },
            ]).unwrap();

            let ibuf = glium::IndexBuffer::new(display,
                glium::index::PrimitiveType::TrianglesList,
                &vec![0, 2, 1, 0, 3, 2]).unwrap();

            let shared = rc::Rc::new(BillboardSharedData {
                program: shader_program_ent!(display, "shaders/billboard.vert", "shaders/billboard.frag"),
                vbuf,
                ibuf,
            });

            sys.add(shared_ent, shared.clone()).unwrap();

            shared
        };

        if !sys.has::<components::Position>(ent) {
            sys.add(ent, components::Position(na::Point3::new(0.0, 0.0, 0.0))).unwrap();
        }

        let bb = Billboard {
            width,
            height,
            start_time: info.time,
            shared,
            prog,
        };

        sys.add(ent, bb).unwrap();
        sys.add(ent, components::Drawable::new(Billboard::draw)).unwrap();

        ent
    }

    pub fn draw(
        sys: &ecs::System,
        ent: ecs::Entity,
        info: &info::Info,
        target: &mut glium::framebuffer::MultiOutputFrameBuffer,
    ) {
        use glium::Surface;

        let bb = sys.borrow::<Billboard>(ent).unwrap();
        let position = sys.borrow::<components::Position>(ent).unwrap().0;

        let uniforms = uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            bb_position: Into::<[f32; 4]>::into(position.to_homogeneous()),
            size: [bb.width, bb.height],
            time: info.time,
            start_time: bb.start_time,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: false,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&bb.shared.vbuf, &bb.shared.ibuf, if let Some(ref p) = bb.prog { p } else { &bb.shared.program }, &uniforms, &params)
            .unwrap();
    }
}
