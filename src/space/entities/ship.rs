use ecs;
use glium;
use std::rc;
use na;
use rand;

use info;
use components;
use entities;

const COLORS: [[f32; 3]; 6] = [
    [0.76, 0.01, 0.0],
    [0.98, 0.83, 0.2],
    [0.44, 0.64, 0.0],
    [0.25, 0.45, 0.75],
    [0.54, 0.26, 0.8],
    [0.9, 0.55, 0.08],
];

#[derive(Debug)]
pub struct ShipSharedData {
    program: glium::Program,
    vbuf: glium::VertexBuffer<Vertex>,
    ibuf1: glium::IndexBuffer<u16>,
    ibuf2: glium::IndexBuffer<u16>,
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
    mailslot_deviation: f32,
}

impl Ship {
    pub fn create(sys: &mut ecs::System, display: &glium::Display) -> ecs::Entity {
        let ent = sys.new_entity();

        let shared = if let Some(s) = if let Ok(ents) = sys.entities_with::<Ship>() {
            if ents.len() > 0 {
                Some(sys.borrow::<Ship>(ents[0]).unwrap().shared.clone())
            } else {
                None
            }
        } else {
            None
        }
        {
            s
        } else {
            let vbuf = glium::VertexBuffer::new(
                display,
                &vec![
                    Vertex { position: [0.0, 0.0, 1.0, 1.0] },
                    Vertex { position: [-0.2, 0.0, -1.0, 1.0] },
                    Vertex { position: [0.2, 0.0, -1.0, 1.0] },
                ],
            ).unwrap();

            let ibuf1 = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &vec![0, 1, 2],
            ).unwrap();

            let ibuf2 = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::LinesList,
                &vec![0, 1, 1, 2, 0, 2],
            ).unwrap();

            let program =
                shader_program_ent!(display, "shaders/station.vert", "shaders/station.frag");

            rc::Rc::new(ShipSharedData {
                program,
                vbuf,
                ibuf1,
                ibuf2,
            })
        };

        let color = COLORS[(rand::random::<f32>() * 6.0) as usize];
        let color = na::Vector3::new(color[0], color[1], color[2]);

        let s = Ship {
            color,
            shared,
            mailslot_deviation: rand::random::<f32>() * 2.0 - 1.0,
        };

        sys.add(ent, s).unwrap();
        sys.add(ent, components::Position(na::Point3::new(0.0, 0.0, 0.0)))
            .unwrap();
        sys.add(ent, components::Velocity(na::Vector3::new(-0.1, 0.0, 0.0)))
            .unwrap();
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
        let vel = sys.borrow::<components::Velocity>(ent).unwrap().0;

        let station = sys.borrow::<entities::Station>(info.station)
            .unwrap()
            .0
            .borrow();
        let mailslot_vector = na::Vector3::new(
            0.0,
            -station.rotation.sin() * s.mailslot_deviation * 0.06,
            station.rotation.cos() * s.mailslot_deviation * 0.06,
        );

        let fac = if vel.x > 0.0 { 1.0 } else { -1.0 };
        let model = (na::Translation3::from_vector(pos.coords + mailslot_vector) *
                         na::UnitQuaternion::new_observer_frame(
                &na::normalize(&vel),
                &na::Vector3::new(0.0, 1.0, 0.0),
            ) *
                         na::UnitQuaternion::new(
                na::Vector3::new(0.0, 0.0, fac * station.rotation),
            ) * na::Similarity3::from_scaling(0.05)).to_homogeneous();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            ..Default::default()
        };

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
            color: [0.0f32, 0.0, 0.0, 1.0],
        };

        target
            .draw(
                &s.shared.vbuf,
                &s.shared.ibuf1,
                &s.shared.program,
                &uniforms,
                &params,
            )
            .unwrap();

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
            color: [s.color.x, s.color.y, s.color.z, 1.0],
        };

        target
            .draw(
                &s.shared.vbuf,
                &s.shared.ibuf2,
                &s.shared.program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
}
