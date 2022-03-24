use vulkano::device::{Device, Queue};

use winit::event_loop::EventLoop;
use winit::window::Window;

use super::MainError;
use std::sync::Arc;


pub fn run(
    event_loop: EventLoop<()>,
    window: Window,
    device: Arc<Device>,
    queue: Arc<Queue>
) -> Result<(), MainError> {
    return Ok(());
}