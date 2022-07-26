use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const WINDOW_TITLE: &'static str = "ash_playground";
const WINDOW_WDITH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct VulkanApp;

impl VulkanApp {
    fn init_window(event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(LogicalSize::new(WINDOW_WDITH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window...")
    }

    fn main_loop(event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            dbg!();
                            *control_flow = ControlFlow::Exit
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            _ => (),
        })
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let _window = VulkanApp::init_window(&event_loop);

    VulkanApp::main_loop(event_loop);
}
