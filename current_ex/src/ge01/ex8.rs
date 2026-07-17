use std::time;

fn update_physics(dt: f32) {
}

pub fn delta() {
    let mut last_time = time::Instant::now();
    let mut accumulator: f32 = 0.0;
    let mut render: i32 = 0;
    let limit: f32 = 20.0;
    let mut counter: i32 = 1;
    loop {
        let sleeper: u64;
        let current_time = time::Instant::now();
        let delta = current_time.duration_since(last_time).as_secs_f32() * 1000.0;
        last_time = current_time;
        accumulator += delta;
        while accumulator >= limit {
            println!("[PHYSICS] Stepping 2D Physics Matrix: 20ms");
            update_physics(limit);
            accumulator -= limit;
        }
        println!("[RENDER] Rasterizing active SVG shapes to screen");
        render += 1;
        if render >= 10 {
            break;
        }
        if counter % 2 == 0{
            sleeper = 10;
        }
        else{
            sleeper = 55;
        }
        counter += 1;
        std::thread::sleep(time::Duration::from_millis(sleeper));
    }
}
