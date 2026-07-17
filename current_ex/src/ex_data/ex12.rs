use crate::ex11::{SceneStackController, TestSceneA};
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct VectorEngineRuntime {
    window_surface: Option<Window>,
    scene_controller: SceneStackController,
    last_frame_time: Instant,
    accumulator: f32,
}

impl ApplicationHandler for VectorEngineRuntime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_surface.is_none() {
            let attribs: winit::window::WindowAttributes = Window::default_attributes()
                .with_title("2D SVG Vector Engine")
                .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
            let win: Result<Window, winit::error::OsError> = event_loop.create_window(attribs);
            if let Ok(window) = win {
                self.window_surface = Some(window)
            }
            self.last_frame_time = Instant::now();
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("[TEARDOWN] Tearing down engine runtime vectors...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(_window) = &self.window_surface {
                    println!("[RENDER PATH] Rasterizing active SVG shapes to screen");
                }
            }
            _ => (),
        }
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window_surface {
            let current_time = Instant::now();
            let delta_time = current_time
                .duration_since(self.last_frame_time)
                .as_secs_f32() * 1000.0;
            self.last_frame_time = current_time;
            self.accumulator += delta_time;
            const FIXED_TIMESTEP: f32 = 20.0;
            while self.accumulator >= FIXED_TIMESTEP {
                if !self.scene_controller.process_tick() {
                    event_loop.exit();
                    return;
                }
                self.accumulator -= FIXED_TIMESTEP;
            }
            window.request_redraw();
        }
    }
}

pub fn main12() {
    println!("[BOOT] Bootstrapping structural engine runtime systems...");
    let loop_event: EventLoop<()> = EventLoop::new().unwrap();
    loop_event.set_control_flow(ControlFlow::Wait);

    let mut runtime: VectorEngineRuntime = VectorEngineRuntime {
        window_surface: None,
        scene_controller: SceneStackController::new( ),
        last_frame_time: Instant::now(),
        accumulator: 0.0,
    };
    runtime.scene_controller.push_context(Box::new(TestSceneA { tick_count: 0 }));
    loop_event.run_app(&mut runtime).unwrap();
}
