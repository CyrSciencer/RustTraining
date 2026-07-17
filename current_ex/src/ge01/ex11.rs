
pub struct SceneStackController{
    active_layers: Vec<Box<dyn Scene>>,
}
pub trait Scene {
    fn on_scene_enter(&mut self);
    fn on_scene_tick(&mut self) -> Option<SceneTransition>;
    fn on_scene_exit(&mut self);
}

impl SceneStackController{
    pub fn new()-> Self {
        return Self {
            active_layers: Vec::new()
        };
    }
    pub fn push_context(&mut self, mut scene: Box<dyn Scene>){
        scene.on_scene_enter();
        self.active_layers.push(scene);
    }
    pub fn process_tick(&mut self) -> bool{
        let active_scene = self.active_layers.last_mut();
        match active_scene{
            Some(_) => {
                self.active_layers[0].on_scene_exit();
                return true;
            },
            None => return false,
        }
    }
}
enum SceneTransition{
    Push(Box<dyn Scene>),
    Pop,
}
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TestSceneA {
    pub tick_count: u32,
}
impl Scene for TestSceneA {
    fn on_scene_enter(&mut self) { println!("[A] enter"); }
    fn on_scene_exit(&mut self)  { println!("[A] exit"); }
    fn on_scene_tick(&mut self) -> Option<SceneTransition> {
        self.tick_count += 1;
        if self.tick_count == 1 {
            // First tick instructs a Push onto the stack
            return Some(SceneTransition::Push(Box::new(TestSceneB)));
        }
        None
    }
}

struct TestSceneB;
impl Scene for TestSceneB {
    fn on_scene_enter(&mut self) { println!("[B] enter"); }
    fn on_scene_exit(&mut self)  { println!("[B] exit"); }
    fn on_scene_tick(&mut self) -> Option<SceneTransition> {
        // First tick immediately instructs a Pop off the stack
        Some(SceneTransition::Pop)
    }
}

pub fn main11(){
    let mut controller = SceneStackController::new();

    println!("--- Initial Push ---");
    // This must trigger TestSceneA's on_scene_enter immediately
    controller.push_context(Box::new(TestSceneA { tick_count: 0 })); 
    assert_eq!(controller.active_layers.len(), 1);

    println!("\n--- Tick 1 (A pushes B) ---");
    // A ticks, returns Push(B). B's enter must run.
    let keep_running = controller.process_tick();
    assert!(keep_running);
    assert_eq!(controller.active_layers.len(), 2);

    println!("\n--- Tick 2 (B pops itself) ---");
    // B ticks, returns Pop. B's exit must run. Control returns to A.
    let keep_running = controller.process_tick();
    assert!(keep_running);
    assert_eq!(controller.active_layers.len(), 1);

    println!("\n--- Tick 3 (A does nothing) ---");
    // A ticks, returns None. No stack adjustments.
    let keep_running = controller.process_tick();
    assert!(keep_running);
    assert_eq!(controller.active_layers.len(), 1);

    println!("\n--- Manual Pop to clear stack ---");
    // Manually popping the last scene context layer
    if let Some(mut remaining_scene) = controller.active_layers.pop() {
        remaining_scene.on_scene_exit(); // A's exit must run
    }

    println!("\n--- Tick 4 (Empty Stack Check) ---");
    // Stack is empty; process_tick must return false to signal shutdown
    let keep_running = controller.process_tick();
    assert!(!keep_running);

    println!("\n[SUCCESS] Exercise 11 architecture conforms to specification!");
}