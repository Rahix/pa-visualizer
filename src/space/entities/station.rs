use ecs;
use toml;
use glium;
use std::rc;
use std::cell;
use na;
use obj;

use info;
use components;

#[derive(Debug, Clone)]
pub struct Station(pub rc::Rc<cell::RefCell<StationObj>>);

#[derive(Debug)]
pub struct StationObj {
    program: glium::Program,
    vbuf1: glium::VertexBuffer<Vertex>,
    ibuf1: glium::IndexBuffer<u16>,
    vbuf2: glium::VertexBuffer<Vertex>,
    ibuf2: glium::IndexBuffer<u16>,

    pub rotation: f32,
    color: na::Vector4<f32>,
    last_beat: f32,
    rotation_factor: f32,
    base_rotation: f32,
}

#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 4],
}

implement_vertex!(Vertex, position);

impl Station {
    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        config: ::std::sync::Arc<toml::Value>,
    ) -> ecs::Entity {
        let ent = sys.new_entity();

        let rotation_factor = config
            .get("SPACE_STATION_ROT_FACTOR")
            .map(|v| {
                v.as_float().expect(
                    "SPACE_STATION_ROT_FACTOR must be a float",
                )
            })
            .unwrap_or(0.1) as f32;
        info!("SPACE_STATION_ROT_FACTOR = {}", rotation_factor);

        let base_rotation = config
            .get("SPACE_STATION_BASE_ROT")
            .map(|v| {
                v.as_float().expect(
                    "SPACE_STATION_BASE_ROT must be a float",
                )
            })
            .unwrap_or(0.5) as f32;
        info!("SPACE_STATION_BASE_ROT = {}", base_rotation);

        let station_object = obj::Obj::<obj::SimplePolygon>::load_buf(
            &mut ::std::io::BufReader::new(
                ::std::io::Cursor::new(&include_bytes!("station.obj")[..]),
            ),
        ).unwrap();

        let (vbuf1, ibuf1) = {
            let vertices = station_object
                .position
                .iter()
                .map(|p| Vertex { position: [p[0], p[1], p[2], 1.0] })
                .collect::<Vec<Vertex>>();

            let mut indices = vec![];

            for object in station_object.objects.iter() {
                for group in object.groups.iter() {
                    for poly in group.polys.iter() {
                        for v in poly.iter() {
                            indices.push(v.0 as u16);
                        }
                    }
                }
            }

            (
                glium::VertexBuffer::new(display, &vertices).unwrap(),
                glium::IndexBuffer::new(
                    display,
                    glium::index::PrimitiveType::TrianglesList,
                    &indices,
                ).unwrap(),
            )
        };

        let station_object = obj::Obj::<obj::SimplePolygon>::load_buf(
            &mut ::std::io::BufReader::new(::std::io::Cursor::new(
                &include_bytes!("station-wireframe.obj")[..],
            )),
        ).unwrap();

        let (vbuf2, ibuf2) = {
            let vertices = station_object
                .position
                .iter()
                .map(|p| Vertex { position: [p[0], p[1], p[2], 1.0] })
                .collect::<Vec<Vertex>>();

            let mut indices = vec![];

            for object in station_object.objects.iter() {
                for group in object.groups.iter() {
                    for poly in group.polys.iter() {
                        for v in poly.iter() {
                            indices.push(v.0 as u16);
                        }
                    }
                }
            }

            (
                glium::VertexBuffer::new(display, &vertices).unwrap(),
                glium::IndexBuffer::new(
                    display,
                    glium::index::PrimitiveType::TrianglesList,
                    &indices,
                ).unwrap(),
            )
        };

        let s = StationObj {
            program: shader_program_ent!(display, "shaders/station.vert", "shaders/station.frag"),
            vbuf1,
            ibuf1,
            vbuf2,
            ibuf2,

            rotation: 0.0,
            color: na::Vector4::new(0.0, 0.0, 0.0, 0.0),
            last_beat: -10.0,
            rotation_factor,
            base_rotation,
        };

        sys.add(ent, Station(rc::Rc::new(cell::RefCell::new(s))))
            .unwrap();
        sys.add(ent, components::Drawable::new(Station::draw))
            .unwrap();
        sys.add(ent, components::Updateable::new(Station::update))
            .unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        let mut s = sys.borrow_mut::<Station>(ent).unwrap().0.borrow_mut();

        s.rotation += info.delta * (info.volume * s.rotation_factor + s.base_rotation);

        if info.beat2 {
            s.last_beat = info.time;
        }

        let fact = 1.0 / (((info.time - s.last_beat) * 10.0) + 1.0);
        s.color.x = (1.0 - fact) * (0.02 / 3.0) + fact * 0.00;
        s.color.y = (1.0 - fact) * (0.22 / 3.0) + fact * 0.63;
        s.color.z = (1.0 - fact) * (0.51 / 3.0) + fact * 1.00;
        s.color.w = 1.0;
    }

    pub fn draw(
        sys: &ecs::System,
        ent: ecs::Entity,
        info: &info::Info,
        target: &mut glium::framebuffer::MultiOutputFrameBuffer,
    ) {
        use glium::Surface;

        let s = sys.borrow::<Station>(ent).unwrap().0.borrow();

        let model = na::Translation3::from_vector(na::Vector3::new(-2.0, 0.0, 0.0))
            .to_homogeneous() *
            na::Rotation3::new(na::Vector3::new(s.rotation, 0.0, 0.0)).to_homogeneous();

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(model),
            color: [0.0f32, 0.0, 0.0, 1.0],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        target
            .draw(&s.vbuf1, &s.ibuf1, &s.program, &uniforms, &params)
            .unwrap();

        let uniforms =
            uniform! {
            perspective_matrix: Into::<[[f32; 4]; 4]>::into(info.perspective),
            view_matrix: Into::<[[f32; 4]; 4]>::into(info.view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(
                model * na::Similarity3::from_scaling(1.005).to_homogeneous()),
            color: [s.color.x, s.color.y, s.color.z, s.color.w],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            polygon_mode: glium::PolygonMode::Line,
            line_width: Some(2.0),
            ..Default::default()
        };

        target
            .draw(&s.vbuf2, &s.ibuf2, &s.program, &uniforms, &params)
            .unwrap();
    }
}
