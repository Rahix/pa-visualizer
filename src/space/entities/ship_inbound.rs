use ecs;
use glium;
use rand;
use na;

use entities;
use components;
use info;

#[derive(Debug, Clone)]
pub struct ShipInbound {
    start_time: f32,
}

impl ShipInbound {
    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        info: &info::Info,
    ) -> ecs::Entity {
        let ent = entities::Ship::create(sys, display);
        let position = na::Point3::new(
            rand::random::<f32>() * 1.0 + 0.5,
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 2.0 - 1.0,
        );

        sys.set(ent, components::Position(position)).unwrap();

        let s = ShipInbound { start_time: info.time };

        sys.add(ent, s).unwrap();
        sys.add(ent, components::Updateable::new(ShipInbound::update))
            .unwrap();

        // Create fsd flash
        let flash = entities::FsdFlash::create(sys, display, info);
        sys.set(flash, components::Position(position)).unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, info: &info::Info) {
        let new_vel = {
            let pos = sys.borrow::<components::Position>(ent).unwrap().0;
            // Delete this ship if we are inside the station
            if pos.x < -1.5 {
                sys.set(ent, components::Position(na::Point3::new(-2.0, 0.0, 0.0)))
                    .unwrap();
                sys.set(ent, components::Velocity(na::Vector3::new(0.005, 0.0, 0.0)))
                    .unwrap();
                sys.set(
                    ent,
                    components::Acceleration(na::Vector3::new(1.0, 0.0, 0.0)),
                ).unwrap();
                sys.set(
                    ent,
                    components::Updateable::new(entities::ShipOutbound::update),
                ).unwrap();
                return;
            }
            let s = sys.borrow::<ShipInbound>(ent).unwrap();
            (&na::Vector3::new(-1.0, -pos.y * 5.0, -pos.z * 5.0).normalize()) *
                (1.0 / (1.0 + info.time - s.start_time) + 0.4)
        };

        sys.set(ent, components::Velocity(new_vel)).unwrap();
    }
}
