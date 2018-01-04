use ecs;
use na;
use std::ops;

use info;

macro_rules! impl_deref {
    ($name:ty, $tp:ty) => (
        impl ops::Deref for $name {
            type Target = $tp;

            fn deref(&self) -> &$tp {
                &self.0
            }
        }

        impl ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $tp {
                &mut self.0
            }
        }
    )
}

#[derive(Debug, Clone)]
pub struct Position(pub na::Point3<f32>);
impl_deref!(Position, na::Point3<f32>);

#[derive(Debug, Clone)]
pub struct Velocity(pub na::Vector3<f32>);
impl_deref!(Velocity, na::Vector3<f32>);

#[derive(Debug, Clone)]
pub struct Acceleration(pub na::Vector3<f32>);
impl_deref!(Acceleration, na::Vector3<f32>);

pub fn update(sys: &mut ecs::System, info: &info::Info) {
    // Don't check result, as we do not care about the ComponentNotFound
    // error, that might happen (No entity uses acceleration, for example)
    let _ = sys.run_mut::<Acceleration, _>(|sys, ent| {
        let new_vel = {
            let acc = sys.borrow::<Acceleration>(ent).unwrap();
            let vel = sys.borrow::<Velocity>(ent).unwrap();

            vel.0 + acc.0 * info.delta
        };

        sys.set(ent, Velocity(new_vel)).unwrap();
    });

    let _ = sys.run_mut::<Velocity, _>(|sys, ent| {
        let new_pos = {
            let vel = sys.borrow::<Velocity>(ent).unwrap();
            let pos = sys.borrow::<Position>(ent).unwrap();

            pos.0 + vel.0 * info.delta
        };

        sys.set(ent, Position(new_pos)).unwrap();
    });
}
