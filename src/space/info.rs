use na;

pub struct Info {
    pub time: f32,
    pub delta: f32,
    pub perspective: na::Matrix4<f32>,
    pub view: na::Matrix4<f32>,
    pub beat: f32,
    pub beat2: bool,
    pub volume: f32,
    pub is_beat: Vec<bool>,
    pub is_beat_previous: Vec<bool>,
}
