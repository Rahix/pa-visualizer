use ecs;
use glium;
use std::rc;
use na;

use info;
use components;
use entities;

#[derive(Debug, Clone)]
pub struct FreqDropShared {
    program: rc::Rc<glium::Program>,
}

#[derive(Debug, Clone)]
pub struct FreqDrop {
    start_time: f32,
}

impl FreqDrop {
    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        info: &info::Info,
        position: f32,
        width: f32,
    ) -> ecs::Entity {
        // There is an entity which just exists to make sure, the shared data
        // will be kept even if there is no flash currently alive
        let shared = if let Ok(ents) = sys.entities_with::<FreqDropShared>() {
            sys.get::<FreqDropShared>(ents[0]).unwrap()
        } else {
            let shared_ent = sys.new_entity();

            // Create shared data
            let shared = FreqDropShared {
                program: rc::Rc::new(shader_program_ent!(
                    display,
                    "shaders/freq_drop.vert",
                    "shaders/freq_drop.frag"
                )),
            };

            sys.add(shared_ent, shared.clone()).unwrap();

            shared
        };

        let f = FreqDrop { start_time: info.time };
        let ent = entities::Billboard::create(
            sys,
            None,
            display,
            width,
            4.0,
            info,
            Some(shared.program.clone()),
        );

        sys.add(ent, f).unwrap();
        sys.set(
            ent,
            components::Position(na::Point3::new(position - width / 2.0, 0.0, 0.9)),
        ).unwrap();
        sys.add(ent, components::Updateable::new(FreqDrop::update))
            .unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        if (info.time - sys.borrow::<FreqDrop>(ent).unwrap().start_time) > 10.0 {
            sys.remove_entity(ent).unwrap();
        }
    }
}
