use crate::ge02::ex1;
use winit::window::Window;
use std::rc::Rc;

struct PaletteApp {
    window: Option<Rc<Window>>,
    context: Option<softbuffer::Context<Rc<Window>>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}