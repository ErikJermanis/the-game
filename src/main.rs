use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes().with_title("The Game");
    ActiveEventLoop::create_window(&self, window_attributes);
}
