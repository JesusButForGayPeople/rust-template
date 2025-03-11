use pixels::{Pixels, SurfaceTexture};
use tiny_skia::Pixmap;
use tokio::runtime::Runtime;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use template::{State, render::*};

fn main() {
    // Initialize the Tokio runtime
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        // Create an event loop
        let event_loop = EventLoop::new();

        // Create a window
        let window = WindowBuilder::new()
            .with_title("Moving Dot in Circle")
            .with_inner_size(LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap();

        // Get window size
        let window_size = window.inner_size();

        // Create a pixel buffer
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let mut pixels =
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

        // Initialize the state
        let mut state = State::new(window_size.width, window_size.height);

        // Run the event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            if keycode == VirtualKeyCode::Q && input.state == ElementState::Pressed
                            {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    let mut pixmap = Pixmap::new(window_size.width, window_size.height).unwrap();
                    draw_background(&mut pixmap, &state);
                    draw_dot(&mut pixmap, &state);

                    let frame = pixels.get_frame_mut();
                    frame.copy_from_slice(pixmap.data());

                    if let Err(_err) = pixels.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::MainEventsCleared => {
                    state.update();
                    window.request_redraw();
                }
                _ => {}
            }
        });
    });
}
