use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "clear";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
        let render_targets = create_render_targets(gpu)?;
        let output = create_output(gpu)?;
        Ok(Self {
            commands,
            queries,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        let Self {
            commands,
            queries,
            render_targets,
            output,
        } = state;
        destroy_output(gpu, &output);
        destroy_render_targets(gpu, &render_targets);
        queries.destroy(gpu);
        commands.destroy(gpu);
        Ok(())
    }
}

//
// Render targets
//

struct RenderTargets {
    color: resource::Image2d,
}

unsafe fn create_render_targets(gpu: &Gpu) -> Result<RenderTargets> {
    let color = resource::Image2d::create(
        gpu,
        &resource::Image2dCreateInfo {
            format: DEFAULT_RENDER_TARGET_COLOR_FORMAT,
            width: DEFAULT_RENDER_TARGET_WIDTH,
            height: DEFAULT_RENDER_TARGET_HEIGHT,
            samples: vk::SampleCountFlagBits::Count1,
            usage: vk::ImageUsageFlagBits::ColorAttachment | vk::ImageUsageFlagBits::TransferSrc,
            property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
        },
    )?;
    Ok(RenderTargets { color })
}

unsafe fn destroy_render_targets(gpu: &Gpu, rt: &RenderTargets) {
    rt.color.destroy(gpu);
}

//
// Output
//

type OutputBuffer = resource::Buffer<u32>;

struct Output {
    buffer: OutputBuffer,
}

unsafe fn create_output(gpu: &Gpu) -> Result<Output> {
    let buffer = OutputBuffer::create(
        gpu,
        &resource::BufferCreateInfo {
            element_count: DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE as _,
            usage: vk::BufferUsageFlagBits::TransferDst.into(),
            property_flags: vk::MemoryPropertyFlagBits::HostVisible.into(),
        },
    )?;
    Ok(Output { buffer })
}

unsafe fn destroy_output(gpu: &Gpu, output: &Output) {
    output.buffer.destroy(gpu);
}

//
// Draw
//

unsafe fn draw(
    gpu @ Gpu { device, queue, .. }: &Gpu,
    Demo {
        commands,
        queries,
        render_targets,
        output,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd);

    // Transition render target.
    device.cmd_pipeline_barrier2(
        cmd,
        &vk::DependencyInfo {
            s_type: vk::StructureType::DependencyInfo,
            p_next: null(),
            dependency_flags: vk::DependencyFlags::empty(),
            memory_barrier_count: 0,
            p_memory_barriers: null(),
            buffer_memory_barrier_count: 0,
            p_buffer_memory_barriers: null(),
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                s_type: vk::StructureType::ImageMemoryBarrier2,
                p_next: null(),
                src_stage_mask: vk::PipelineStageFlagBits2::TopOfPipe.into(),
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                dst_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
            },
        },
    );

    // Begin rendering.
    device.cmd_begin_rendering(
        cmd,
        &(vk::RenderingInfo {
            s_type: vk::StructureType::RenderingInfo,
            p_next: null(),
            flags: vk::RenderingFlags::empty(),
            render_area: render_targets.color.rect_2d(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: render_targets.color.image_view,
                image_layout: vk::ImageLayout::AttachmentOptimal,
                resolve_mode: vk::ResolveModeFlagBits::None,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: DEFAULT_RENDER_TARGET_CLEAR_COLOR,
                    },
                },
            }),
            p_depth_attachment: null(),
            p_stencil_attachment: null(),
        }),
    );

    // End rendering.
    device.cmd_end_rendering(cmd);

    // Transition render target.
    device.cmd_pipeline_barrier2(
        cmd,
        &vk::DependencyInfo {
            s_type: vk::StructureType::DependencyInfo,
            p_next: null(),
            dependency_flags: vk::DependencyFlags::empty(),
            memory_barrier_count: 0,
            p_memory_barriers: null(),
            buffer_memory_barrier_count: 0,
            p_buffer_memory_barriers: null(),
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                s_type: vk::StructureType::ImageMemoryBarrier2,
                p_next: null(),
                src_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                src_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::Copy.into(),
                dst_access_mask: vk::AccessFlagBits2::TransferRead.into(),
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_targets.color.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_targets.color.width(),
                buffer_image_height: render_targets.color.height(),
                image_subresource: render_targets.color.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_targets.color.extent_3d(),
            }),
        }),
    );

    // End queries.
    queries.end(gpu, cmd);

    // End command buffer.
    commands.end(gpu)?;

    // Queue submit.
    device.queue_submit2(
        *queue,
        1,
        &(vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: 0,
            p_wait_semaphore_infos: null(),
            command_buffer_info_count: 1,
            p_command_buffer_infos: &(vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer: cmd,
                device_mask: 0,
            }),
            signal_semaphore_info_count: 1,
            p_signal_semaphore_infos: &(vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: commands.semaphore,
                value: 1,
                stage_mask: vk::PipelineStageFlagBits2::BottomOfPipe.into(),
                device_index: 0,
            }),
        }),
        vk::Fence::null(),
    )?;

    // Wait for semaphore.
    {
        let semaphores = [commands.semaphore];
        let values = [1];
        device.wait_semaphores(
            &(vk::SemaphoreWaitInfo {
                s_type: vk::StructureType::SemaphoreWaitInfo,
                p_next: null(),
                flags: vk::SemaphoreWaitFlagBits::Any.into(),
                semaphore_count: semaphores.len() as _,
                p_semaphores: semaphores.as_ptr(),
                p_values: values.as_ptr(),
            }),
            u64::MAX,
        )?;
    }

    // Query results.
    {
        info!("Rendering took {:?}", queries.elapsed(gpu)?);
        info!("Rendering statistics: {:?}", queries.statistics(gpu)?);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.color.width();
        let height = render_targets.color.height();
        let pixels_byte_size = render_targets.color.byte_size();
        let mut pixels = vec![0_u8; pixels_byte_size as _];
        std::ptr::copy_nonoverlapping(
            output.buffer.ptr.cast::<u8>(),
            pixels.as_mut_ptr(),
            pixels_byte_size as _,
        );
        let image = RgbaImage::from_raw(width, height, pixels)
            .context("Creating image from output buffer")?;
        let image_path = work_dir_or_create()?.join(format!("{demo_name}.png"));
        image.save_with_format(&image_path, ImageFormat::Png)?;
        info!("Wrote image to {}", image_path.display());
    }

    Ok(())
}
