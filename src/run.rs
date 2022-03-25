use vulkano::device::{Device, Queue};

use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::window::Window;

use std::sync::Arc;


/// Runs the [`EventLoop`] and handles all given [`Event`]s.
/// Rendering operations first occur when a [`Event::RedrawRequested`] is polled from the [`EventLoop`].
pub fn run(
    event_loop: EventLoop<()>,
    window: Window,
    device: Arc<Device>,
    queue: Arc<Queue>
) {
    event_loop.run(move |event, _, control_flow| {
        // Continuously run the event loop, even if the OS hasn't dispatched any.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },

            Event::MainEventsCleared => {
                window.request_redraw();
            },

            Event::RedrawRequested(id) => {
                // Rendering Goes Here
            },

            Event::LoopDestroyed => {
                // Do On Quit, Cleanup Code
            },

            _ => {  },
        }
    });
}