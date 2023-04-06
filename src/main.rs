mod utils;

use std::sync::Arc;
use vulkano::{
    swapchain::{ self, SwapchainCreateInfo, SwapchainCreationError, PresentInfo, AcquireError, Surface },
    sync::{ self, GpuFuture, FenceSignalFuture, FlushError },
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{ Window, WindowBuilder },
    dpi::PhysicalPosition,
    event_loop::{ EventLoop, ControlFlow },
    event::{ Event, WindowEvent, MouseButton, ElementState, MouseScrollDelta }
};
use utils::Vector2_32;
use utils::Vector2_64;

const ZOOM_MULTIPLIER_32: f32 = 1.25;
const FRACTAL_SPEED: f32 = 0.005;
const LEFT_KEY: u32 = 57419;
const RIGHT_KEY: u32 = 57421;
const UP_KEY: u32 = 57416;
const DOWN_KEY: u32 = 57424;

fn main()
{
    let instance = utils::init::create_instance();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_title("Fractals: Julia | Vulkano")
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to create surface.");

    let (physical_device, device, mut queues) = utils::init::create_device(&instance, &surface);
    let queue = queues.next().unwrap();

    let (mut swapchain, images) = utils::init::create_swapchain(
        physical_device.clone(),
        device.clone(),
        surface.clone()
    );

    // Rendering
    let render_pass = utils::rendering::create_render_pass(&device, &swapchain);
    let framebuffers = utils::rendering::create_framebuffers(&images, &render_pass);
    let vertex_buffer = utils::rendering::create_vertex_buffer(&device);
    let mut viewport = utils::init::get_viewport(&surface);

    let mut image_zoom = 1f32;
    let mut image_center = Vector2_32::new(0.0, 0.0);
    set_center(&mut image_center, &surface);
    let mut image_offset = Vector2_64::new(0.0, 0.0);
    let mut fractal_coordinates = Vector2_32::new(0.0, 0.0);

    let pipeline = utils::rendering::create_pipeline(&device, &render_pass, &viewport);
    let mut command_buffers = utils::rendering::create_command_buffers(
        &device,
        &queue,
        &pipeline,
        &framebuffers,
        &vertex_buffer,
        image_center,
        image_offset,
        image_zoom,
        fractal_coordinates
    );

    // Main Loop ----------------------------------------------------

    let mut redraw = false;
    let mut recreate_swapchain = false;
    let mut mouse_down = false;
    let mut last_mouse_position = PhysicalPosition::new(0.0, 0.0);

    let frames_in_flight = images.len();
    let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
    let mut previous_fence_i = 0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            redraw = true;
        }
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => {
            if input.state == ElementState::Pressed
            {
                match input.scancode
                {
                    UP_KEY => {
                        fractal_coordinates.y += FRACTAL_SPEED;
                    }
                    LEFT_KEY => {
                        fractal_coordinates.x -= FRACTAL_SPEED;
                    }
                    RIGHT_KEY => {
                        fractal_coordinates.x += FRACTAL_SPEED;
                    }
                    DOWN_KEY => {
                        fractal_coordinates.y -= FRACTAL_SPEED;
                    }
                    _ => ()
                }

                redraw = true;
            }
        }
        Event::WindowEvent {
            event: WindowEvent::MouseWheel { delta, .. },
            ..
        } => {
            zoom_32(
                &mut image_zoom,
                match delta {
                    MouseScrollDelta::LineDelta(_, vertical) => {
                        vertical
                    }
                    MouseScrollDelta::PixelDelta(position) => {
                        position.y as f32
                    }
                }
            );

            redraw = true;
        }
        Event::WindowEvent {
            event: WindowEvent::MouseInput { state, button, .. },
            ..
        } => {
            if button == MouseButton::Left || button == MouseButton::Right
            {
                mouse_down = state == ElementState::Pressed;
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            if mouse_down
            {
                image_offset += utils::Vector2_64::new(
                    (position.x - last_mouse_position.x) / (image_zoom as f64),
                    (position.y - last_mouse_position.y) / (image_zoom as f64)
                );
                redraw = true;
            }

            last_mouse_position = position;
        }
        Event::MainEventsCleared => {
            if redraw || recreate_swapchain
            {
                recreate_swapchain = false;

                let new_dimensions = surface.window().inner_size();
                let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo{
                    image_extent: new_dimensions.into(),
                    ..swapchain.create_info()
                }) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::ImageExtentNotSupported{..}) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e)
                };
                swapchain = new_swapchain;
                let new_framebuffers = utils::rendering::create_framebuffers(&new_images, &render_pass);

                if redraw
                {
                    redraw = false;
                    viewport.dimensions = new_dimensions.into();
                    set_center(&mut image_center, &surface);

                    let new_pipeline = utils::rendering::create_pipeline(&device, &render_pass, &viewport);
                    command_buffers = utils::rendering::create_command_buffers(
                        &device,
                        &queue,
                        &new_pipeline,
                        &new_framebuffers,
                        &vertex_buffer,
                        image_center,
                        image_offset,
                        image_zoom,
                        fractal_coordinates
                    );
                }
            }

            let (image_i, suboptimal, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire next image {:?}", e),
            };

            if suboptimal
            {
                recreate_swapchain = true;
            }

            // wait for the fence to this image to finish (normally this would be the oldest fence)
            if let Some(image_fence) = &fences[image_i]
            {
                image_fence.wait(None).unwrap();
            }

            let previous_future = match fences[previous_fence_i].clone()
            {
                // Create a NowFuture
                None => {
                    let mut now = sync::now(device.clone());
                    now.cleanup_finished();
                    now.boxed()
                }
                // Use the existing FenceSignalFuture
                Some(fence) => fence.boxed(),
            };

            let future = previous_future
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffers[image_i].clone())
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    PresentInfo {
                        index: image_i,
                        ..PresentInfo::swapchain(swapchain.clone())
                    },
                )
                .then_signal_fence_and_flush();

            fences[image_i] = match future
            {
                Ok(value) => Some(Arc::new(value)),
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    None
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    None
                }
            };

            previous_fence_i = image_i;
        }
        _ => ()
    });
}

fn set_center(center: &mut Vector2_32, surface: &Arc<Surface<Window>>)
{
    *center = Vector2_32::new(
        surface.window().inner_size().width as f32 / 2.0,
        surface.window().inner_size().height as f32 / 2.0
    );
}

fn zoom_32(zoom_ref: &mut f32, scroll: f32)
{
    *zoom_ref *= if scroll < 0.0 { 1.0 / ZOOM_MULTIPLIER_32 } else { ZOOM_MULTIPLIER_32 };
}
