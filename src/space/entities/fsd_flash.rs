use ecs;
use glium;
use std::rc;

use info;
use components;
use entities;

#[derive(Debug, Clone)]
struct FsdFlashShared {
    program: rc::Rc<glium::Program>,
}

#[derive(Debug, Clone)]
pub struct FsdFlash {
    start_time: f32,
}

impl FsdFlash {
    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        info: &info::Info,
    ) -> ecs::Entity {
        // There is an entity which just exists to make sure, the shared data
        // will be kept even if there is no flash currently alive
        let shared = if let Ok(ents) = sys.entities_with::<FsdFlashShared>() {
            sys.get::<FsdFlashShared>(ents[0]).unwrap()
        } else {
            let shared_ent = sys.new_entity();

            // Create shared data
            let shared = FsdFlashShared {
                program: rc::Rc::new(shader_program_ent!(
                    display,
                    "shaders/billboard.vert",
                    "shaders/fsd_flash.frag"
                )),
            };

            sys.add(shared_ent, shared.clone()).unwrap();

            shared
        };

        let f = FsdFlash { start_time: info.time };
        let ent = entities::Billboard::create(
            sys,
            None,
            display,
            1.0,
            1.0,
            info,
            Some(shared.program.clone()),
        );

        sys.add(ent, f).unwrap();
        sys.add(ent, components::Updateable::new(FsdFlash::update))
            .unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        if (info.time - sys.borrow::<FsdFlash>(ent).unwrap().start_time) > 0.5 {
            sys.remove_entity(ent).unwrap();
        }
    }
}
