use ecs;
use toml;
use glium;
use std::rc;
use std::cell;
use na;
use obj;

use info;
use components;
use CONFIG;

#[derive(Debug, Clone)]
pub struct Planet(pub rc::Rc<cell::RefCell<PlanetObj>>);

#[derive(Debug)]
pub struct PlanetObj {
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
    camera_height: f32,
}

#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 4],
}

implement_vertex!(Vertex, position);

impl Planet {
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

        let camera_height = config
            .get("SPACE_CAMERA_HEIGHT")
            .map(|v| {
                v.as_float().expect("SPACE_CAMERA_HEIGHT must be a float")
            })
            .unwrap_or(0.0) as f32;

        let planet_object = obj::Obj::<obj::SimplePolygon>::load_buf(
            &mut ::std::io::BufReader::new(
                ::std::io::Cursor::new(&include_bytes!("planet.obj")[..]),
            ),
        ).unwrap();

        let (vbuf1, ibuf1) = {
            let vertices = planet_object
                .position
                .iter()
                .map(|p| Vertex { position: [p[0], p[1], p[2], 1.0] })
                .collect::<Vec<Vertex>>();

            let mut indices = vec![];

            for object in planet_object.objects.iter() {
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

        let planet_object = obj::Obj::<obj::SimplePolygon>::load_buf(
            &mut ::std::io::BufReader::new(
                ::std::io::Cursor::new(&include_bytes!("planet.obj")[..]),
            ),
        ).unwrap();

        let (vbuf2, ibuf2) = {
            let vertices = planet_object
                .position
                .iter()
                .map(|p| Vertex { position: [p[0], p[1], p[2], 1.0] })
                .collect::<Vec<Vertex>>();

            let mut indices = vec![];

            for object in planet_object.objects.iter() {
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

        let s = PlanetObj {
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
            camera_height,
        };

        sys.add(ent, Planet(rc::Rc::new(cell::RefCell::new(s))))
            .unwrap();
        sys.add(ent, components::Drawable::new(Planet::draw))
            .unwrap();
        sys.add(ent, components::Updateable::new(Planet::update))
            .unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        let mut s = sys.borrow_mut::<Planet>(ent).unwrap().0.borrow_mut();

        s.rotation += info.delta * (info.volume * s.rotation_factor + s.base_rotation) / 8.0;

        if info.beat2 {
            s.last_beat = info.time;
        }

        let fact = 1.0 / (((info.time - s.last_beat) * 10.0) + 1.0);
        s.color.x = (1.0 - fact) * 0.040000 + fact * 0.2364631;
        s.color.y = (1.0 - fact) * 0.011211 + fact * 0.017778;
        s.color.z = (1.0 - fact) * 0.116401 + fact * 0.119093;
        s.color.w = 1.0;
    }

    pub fn draw(
        sys: &ecs::System,
        ent: ecs::Entity,
        info: &info::Info,
        target: &mut glium::framebuffer::MultiOutputFrameBuffer,
    ) {
        use glium::Surface;

        let s = sys.borrow::<Planet>(ent).unwrap().0.borrow();

        let model = na::Translation3::from_vector(na::Vector3::new(
            ezconf_float!(CONFIG: "planet.x", 5.8) as f32,
            ezconf_float!(CONFIG: "planet.y", -6.4) as f32,
            ezconf_float!(CONFIG: "planet.z", -14.0) as f32,
        )).to_homogeneous() *
            na::Rotation3::new(na::Vector3::new(0.0, s.rotation, 0.0)).to_homogeneous() *
            na::Matrix4::new_scaling(ezconf_float!(CONFIG: "planet.scale", 6.0) as f32);


        let camera_position = na::Point3::new(3.0, s.camera_height, 2.0);
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
            view_matrix: Into::<[[f32; 4]; 4]>::into(view),
            model_matrix: Into::<[[f32; 4]; 4]>::into(
                model * na::Similarity3::from_scaling(1.01).to_homogeneous()),
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
