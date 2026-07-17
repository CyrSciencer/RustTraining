use std::option::Option;

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
    pub fn new() -> Self {
        return World {
            transforms: Vec::new(),
            motions: Vec::new(),
            renders: Vec::new(),
            next_id: 0,
        };
    }
    pub fn spawn_actor(&mut self) -> Entity {
        let current_entity = self.next_id;
        self.transforms.push(None);
        self.motions.push(None);
        self.renders.push(None);
        self.next_id += 1;
        return current_entity;
    }
    pub fn attach_transform(&mut self, entity: Entity, t: Transform2D) {
        self.transforms[entity] = Some(t);
    }
    pub fn attach_motion(&mut self, entity: Entity, m: Motion2D) {
        self.motions[entity] = Some(m);
    }
    pub fn attach_render(&mut self, entity: Entity, r: SvgRender) {
        self.renders[entity] = Some(r);
    }
}

pub fn step_kinematics(world: &mut World) {
    for (transform_op, motion_op) in world.transforms.iter_mut().zip(world.motions.iter_mut()) {
        if let (Some(transform), Some(motion)) = (transform_op, motion_op) {
            transform.x += motion.vx;
            transform.y += motion.vy;
        };
    }
}

pub fn main9() {
    let mut world = World::new();

    // Actor 0: Static SVG Map Boundary (Only has Position & Graphic references)
    let background = world.spawn_actor();
    world.attach_transform(
        background,
        Transform2D {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        },
    );
    world.attach_render(
        background,
        SvgRender {
            asset_guid: String::from("svg_map_grid"),
        },
    );

    // Actor 1: Dynamic Vector Agent (Possesses complete Kinematic components)
    let player = world.spawn_actor();
    world.attach_transform(
        player,
        Transform2D {
            x: 12.0,
            y: -5.0,
            scale: 2.5,
        },
    );
    world.attach_motion(player, Motion2D { vx: 0.5, vy: 1.2 });
    world.attach_render(
        player,
        SvgRender {
            asset_guid: String::from("svg_player_vessel"),
        },
    );

    println!("[ECS] Memory alignment mapped. Run sequence steps...");
    for frame in 1..=3 {
        step_kinematics(&mut world);
        println!("[ECS] Matrix loop step {} finished.", frame);
    }
}
