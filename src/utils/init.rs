use std::sync::Arc;
use vulkano::{
    VulkanLibrary,
    instance::{ Instance, InstanceCreateInfo },
    device::{
        Device,
        DeviceCreateInfo,
        Queue,
        QueueCreateInfo,
        DeviceExtensions,
        physical::{ PhysicalDevice, PhysicalDeviceType }
    },
    pipeline::graphics::viewport::Viewport,
    swapchain::{ Surface, Swapchain, SwapchainCreateInfo },
    image::{ ImageUsage, SwapchainImage },
};
use winit::window::Window;

pub fn select_physical_device
(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
    device_extensions: &DeviceExtensions
) -> (Arc<PhysicalDevice>, u32)
{
    instance
        .enumerate_physical_devices()
        .expect("Could not enumerate devices.")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            _ => 4,
        })
        .expect("No physical device available.")
}

pub fn create_instance() -> Arc<Instance>
{
    let library = VulkanLibrary::new().expect("No local Vulkan library/DLL.");
    let required_extensions = vulkano_win::required_extensions(&library);

    Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        }
    ).expect("Failed to create instance.")
}

pub fn create_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>
) -> (
    Arc<PhysicalDevice>,
    Arc<Device>,
    impl ExactSizeIterator + Iterator <Item = Arc<Queue>>
)
{
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);

    let (device, queues) = Device::new(
        physical.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        }
    ).expect("Failed to create device.");

    (physical, device, queues)
}

pub fn create_swapchain(
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    surface: Arc<Surface<Window>>
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>)
{
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("Failed to get surface capabilities.");

    let image_format = Some(
        physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0
    );

    let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();

    Swapchain::new(
        device,
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: surface.window().inner_size().into(),
            image_usage: ImageUsage {
                color_attachment: true,
                ..Default::default()
            },
            composite_alpha,
            ..Default::default()
        }
    ).expect("Failed to create swapchain.")
}

pub fn get_viewport(surface: &Arc<Surface<Window>>) -> Viewport
{
    Viewport {
        origin: [0.0, 0.0],
        dimensions: surface.window().inner_size().into(),
        depth_range: 0.0..1.0
    }
}
