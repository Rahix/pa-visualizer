use ecs;
use glium;
use na;

use entities;
use components;
use info;

#[derive(Debug, Clone)]
pub struct ShipOutbound;

impl ShipOutbound {
    pub fn create(
        sys: &mut ecs::System,
        display: &glium::Display,
        _info: &info::Info,
    ) -> ecs::Entity {
        let ent = entities::Ship::create(sys, display);

        sys.set(ent, components::Position(na::Point3::new(-2.0, 0.0, 0.0)))
            .unwrap();
        sys.add(
            ent,
            components::Acceleration(na::Vector3::new(2.0, 0.0, 0.0)),
        ).unwrap();
        sys.add(ent, components::Updateable::new(ShipOutbound::update))
            .unwrap();

        ent
    }

    pub fn update(sys: &mut ecs::System, ent: ecs::Entity, _info: &info::Info) {
        let pos = sys.borrow::<components::Position>(ent).unwrap().0;
        // Delete this ship if we are inside the station
        if pos.x > 5.0 {
            sys.remove_entity(ent).unwrap();
            return;
        }
    }
}
