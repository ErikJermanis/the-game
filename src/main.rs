use winit::{
    dpi::LogicalSize, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_title("The Game")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    }).unwrap();
}
