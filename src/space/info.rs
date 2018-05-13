use na;
use ecs;

pub struct Info<'a> {
    pub time: f32,
    pub delta: f32,
    pub perspective: na::Matrix4<f32>,
    pub view: na::Matrix4<f32>,
    pub beat: f32,
    pub beat2: bool,
    pub volume: f32,
    pub is_beat: Vec<bool>,
    pub is_beat_previous: Vec<bool>,

    pub station: ecs::Entity,
    pub planet: ecs::Entity,

    pub spectrum: Option<(&'a [f32], &'a [f32])>,
}
