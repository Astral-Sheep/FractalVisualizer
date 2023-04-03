#![allow(unused)]

use std::sync::Arc;
use bytemuck::{ Pod, Zeroable };
use vulkano::{
    device::{ Device, Queue },
    pipeline::{
        graphics::{
            vertex_input::BuffersDefinition,
            viewport::{ Viewport, ViewportState },
            input_assembly::InputAssemblyState,
        },
        layout::PipelineLayoutCreateInfo,
        GraphicsPipeline, PipelineBindPoint, Pipeline, PipelineLayout,
    },
    render_pass::{ RenderPass, Subpass, Framebuffer, FramebufferCreateInfo },
    buffer::{ CpuAccessibleBuffer, BufferUsage, TypedBufferAccess, BufferContents },
    command_buffer::{ RenderPassBeginInfo, AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, CommandBufferUsage, SubpassContents },
    image::{
        SwapchainImage,
        view::ImageView,
    },
    swapchain::Swapchain,
    descriptor_set::{ PersistentDescriptorSet, WriteDescriptorSet },
};
use winit::window::Window;
use super::shader;
use super::Vector2;

pub fn create_render_pass(device: &Arc<Device>, swapchain: &Arc<Swapchain<Window>>) -> Arc<RenderPass>
{
    vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).expect("Failed to create render pass.")
}

pub fn create_framebuffers(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: &Arc<RenderPass>
) -> Vec<Arc<Framebuffer>>
{
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            ).unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn create_pipeline(device: &Arc<Device>, render_pass: &Arc<RenderPass>, viewport: &Viewport) -> Arc<GraphicsPipeline>
{
    const ENTRY_POINT: &str = "main";

    let vert_shader = shader::vertex::load(device.clone()).expect("Failed to create vertex shader module.");
    let frag_shader = shader::fragment::load(device.clone()).expect("Failed to create fragment shader module.");

    GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vert_shader.entry_point(ENTRY_POINT).unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport.clone()]))
        .fragment_shader(frag_shader.entry_point(ENTRY_POINT).unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .expect("Failed to create graphics pipeline.")
}

pub fn create_vertex_buffer(device: &Arc<Device>) -> Arc<CpuAccessibleBuffer<[Vertex]>>
{
    CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        false,
        vec![
            Vertex { position: [3.0, -1.0] },
            Vertex { position: [-1.0, -1.0] },
            Vertex { position: [-1.0, 3.0] },
        ].into_iter()
    ).expect("Failed to create vertex buffer.")
}

pub fn create_command_buffers(
    device: &Arc<Device>,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
    image_center: Vector2,
    image_offset: Vector2,
    image_zoom: f32
) -> Vec<Arc<PrimaryAutoCommandBuffer>>
{
    let layout = pipeline.layout().set_layouts().get(0).expect("Failed to get layout set.");
    let set = PersistentDescriptorSet::new(
        layout.clone(),
        [
            create_descriptor_set(&device, 0, image_center),
            create_descriptor_set(&device, 1, image_offset),
            create_descriptor_set(&device, 2, image_zoom)
        ]
    ).unwrap();

    framebuffers
        .iter()
        .map(|framebuffer| {
            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.1, 0.1, 0.1, 1.0].into())],
                        ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                    },
                    SubpassContents::Inline,
                )
                .unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    set.clone()
                )
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .expect("Failed to draw render pass")
                .end_render_pass()
                .unwrap();

            Arc::new(builder.build().unwrap())
        })
        .collect()
}

fn create_descriptor_set<T>(device: &Arc<Device>, binding: u32, data: T) -> WriteDescriptorSet
where T : BufferContents
{
    WriteDescriptorSet::buffer(
        binding,
        CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage {
                uniform_buffer: true,
                ..Default::default()
            },
            false,
            data
        ).expect("Failed to create descriptor set")
    )
}

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub struct Vertex
{
    position: [f32; 2]
}

vulkano::impl_vertex!(Vertex, position);
