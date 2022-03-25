use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::physical::PhysicalDevice;

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use std::sync::Arc;

mod run;


/// Initializes Vulkan through the [`vulkano`] library, creating a Vulkan [`Device`] and [`Queue`].
/// 
/// # Panics
/// - If [`vulkano`] fails to create an [`Instance`].
/// - If there is no [`PhysicalDevice`] availible.
/// - If there is no `QueueFamily` that supports graphics within the [`PhysicalDevice`].
/// - If there is a failure during the creation of a [`Device`].
fn init_vulkan() -> (Arc<Device>, Arc<Queue>) {
    let instance = Instance::new(InstanceCreateInfo::default())
        .expect("Failed to create an Instance.");
    
    let physical_device = PhysicalDevice::enumerate(&instance).next()
        .expect("No PhysicalDevice availible.");
    
    let queue_family = physical_device.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Could not find a Graphical Queue Family from the PhysicalDevice.");
    
    let (device, mut queues) = Device::new(
        physical_device, DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            .. Default::default()
        },
    ).expect("Failed to create a Device.");
    
    return (device, queues.next().unwrap());
}


fn main(){
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    
    window.set_title("Vulkan Template");
    
    let (device, queue) = init_vulkan();

    run::run(event_loop, window, device, queue);
}
