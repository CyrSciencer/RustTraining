use tiny_skia::{Pixmap, PathBuilder, Paint, Transform};
use crate::ge02::ex1::AppShell;

pub fn render_vector_canvas(pixmap: &mut Pixmap) -> Result<(), &'static str> {
    // TODO: Clear background with Deep Salmon Pink (#FF8C94)
    // TODO: Construct path geometry using PathBuilder (Gold color #FFD700)
    // TODO: Fill path into pixmap safely
}

pub fn main2(){
        let event_loop = winit::event_loop::EventLoop::new();
    let mut app_shell = AppShell::new();
    if let Ok(event_loop) = event_loop {
        event_loop
            .run_app(&mut app_shell)
            .expect("failed to run event loop");
    } else {
        println!("Error creating event loop: {:?}", event_loop.err().unwrap());
    }
}