use winit::application::ApplicationHandler;
use winit::window::{Window, WindowId};

pub struct AppShell {
    window_surface: Option<Window>,
}

impl AppShell {
    pub fn new() -> Self {
        return AppShell {
            window_surface: None,
        };
    }
}
impl ApplicationHandler for AppShell {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window_surface.is_none() {
            let loop_event: Result<Window, winit::error::OsError> = event_loop
                .create_window(Window::default_attributes().with_title("My First Window!"));
            self.window_surface = match loop_event {
                Ok(window) => Some(window),
                Err(e) => {
                    println!("Error creating window: {:?}", e);
                    None
                }
            };
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: WindowId,
        event: winit::event::WindowEvent,
    ) -> () {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("Window close requested, exiting event loop.");
                event_loop.exit();
            }
            _ => (),
        }
    }
}

pub fn main1() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut app_shell = AppShell {
        window_surface: None,
    };
    if let Ok(event_loop) = event_loop {
        event_loop
            .run_app(&mut app_shell)
            .expect("failed to run event loop");
    } else {
        println!("Error creating event loop: {:?}", event_loop.err().unwrap());
    }
}
