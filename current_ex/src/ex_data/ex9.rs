pub type Entity = usize;

pub struct Transform2D {
    x: f32,
    y: f32,
    scale: f32,
}

pub struct Motion2D {
    vx: f32,
    vy: f32,
}

pub struct SvgRender {
    asset_guid: String,
}

pub struct World {
    transforms: Vec<Option<Transform2D>>,
    motions: Vec<Option<Motion2D>>,
    renders: Vec<Option<SvgRender>>,
    next_id: usize,
}

impl World {
    pub fn new() -> Self;
    pub fn spawn_actor(&mut self) -> Entity;
    pub fn attach_transform(&mut self, entity: Entity, t: Transform2D);
    pub fn attach_motion(&mut self, entity: Entity, m: Motion2D);
    pub fn attach_render(&mut self, entity: Entity, r: SvgRender);
}
