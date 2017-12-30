use ecs;
use glium;
use std::rc;
use na;

use info;
use components;

#[derive(Debug)]
pub struct ShipSharedData {
    program: glium::Program,
    vbuf: glium::VertexBuffer<Vertex>,
    ibuf: glium::IndexBuffer<u16>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 4],
}

implement_vertex!(Vertex, position);

pub type ShipShared = rc::Rc<ShipSharedData>;

#[derive(Debug, Clone)]
pub struct Ship {
    color: na::Vector3<f32>,
    shared: ShipShared,
}

impl Ship {
    pub fn create(sys: &mut ecs::System, display: &glium::Display, color: na::Vector3<f32>) -> ecs::Entity {
        let ent = sys.new_entity();

        let shared = if let Some(s) = if let Ok(ents) = sys.entities_with::<Ship>() {
                if ents.len() > 0 {
                    Some(sys.borrow::<Ship>(ents[0]).unwrap().shared.clone())
                } else {
                    None
                }
            } else {
                None
            } {
            s
        } else {
            let vbuf = glium::VertexBuffer::new(display, &vec![Vertex {
                position: [0.0, 0.0, 0.0, 1.0],
            }]).unwrap();

            let ibuf = glium::IndexBuffer::new(display, glium::index::PrimitiveType::Points, &vec![0]).unwrap();

            let program = shader_program_ent!(display, "shaders/station.vert", "shaders/station.frag");

            rc::Rc::new(ShipSharedData {
                program,
                vbuf,
                ibuf,
            })
        };

        let s = Ship {
            color,
            shared,
        };

        sys.add(ent, s).unwrap();
        sys.add(ent, components::Position(na::Point3::new(0.0, 0.0, 0.0))).unwrap();
        sys.add(ent, components::Velocity(na::Vector3::new(-0.1, 0.0, 0.0))).unwrap();
        sys.add(ent, components::Drawable::new(Ship::draw)).unwrap();

        ent
    }

    pub fn draw(
        sys: &ecs::System,
        ent: ecs::Entity,
        info: &info::Info,
        target: &mut glium::framebuffer::MultiOutputFrameBuffer,
    ) {
        use glium::Surface;

        let s = sys.borrow::<Ship>(ent).unwrap();
        let pos = sys.borrow::<components::Position>(ent).unwrap().0;

        let model = na::Translation3::from_vector(pos.coords).to_homogeneous();

        let ps_view = info.view * pos.to_homogeneous();
        let ps_check_point = info.perspective * na::Vector4::new(1.0, 0.0, ps_view.z, 1.0);
        let point_size = ps_check_point.x / ps_check_point.w;
        let point_size = 2.0 + ((point_size - 0.2) / 0.2) * 8.0;
        debug!("{}", point_size);

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
            color: [s.color.x, s.color.y, s.color.z, 1.0],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            point_size: Some(point_size),
            .. Default::default()
        };

        target.draw(&s.shared.vbuf, &s.shared.ibuf, &s.shared.program, &uniforms, &params)
            .unwrap();
    }
}
