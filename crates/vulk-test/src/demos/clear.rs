use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    timestamp_queries: TimestampQueries,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "clear";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands: command::Commands = command::create(gpu)?;
        let timestamp_queries = create_timestamp_queries(gpu)?;
        let format = vk::Format::R8g8b8a8Unorm;
        let width = 256;
        let height = 256;
        let render_targets = create_render_targets(gpu, format, width, height)?;
        let output = create_output(gpu, format, width, height)?;
        Ok(Self {
            commands,
            timestamp_queries,
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
            render_targets,
            output,
            timestamp_queries,
        } = state;
        destroy_output(gpu, &output);
        destroy_render_targets(gpu, &render_targets);
        destroy_timestamp_queries(gpu, &timestamp_queries);
        command::destroy(gpu, &commands);
        Ok(())
    }
}

//
// Timestamp queries
//

struct TimestampQueries {
    query_pool: vk::QueryPool,
    query_pool_create_info: vk::QueryPoolCreateInfo,
}

unsafe fn create_timestamp_queries(Gpu { device, .. }: &Gpu) -> Result<TimestampQueries> {
    let query_count = 2;
    let query_pool_create_info = vk::QueryPoolCreateInfo {
        s_type: vk::StructureType::QueryPoolCreateInfo,
        p_next: null(),
        flags: vk::QueryPoolCreateFlags::empty(),
        query_type: vk::QueryType::Timestamp,
        query_count,
        pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
    };
    let query_pool = device.create_query_pool(&query_pool_create_info)?;
    device.reset_query_pool(query_pool, 0, query_count);
    Ok(TimestampQueries {
        query_pool,
        query_pool_create_info,
    })
}

unsafe fn destroy_timestamp_queries(
    Gpu { device, .. }: &Gpu,
    timestamp_queries: &TimestampQueries,
) {
    device.destroy_query_pool(timestamp_queries.query_pool);
}

//
// Render targets
//

struct RenderTargets {
    color_image: resource::Image2d,
}

unsafe fn create_render_targets(
    gpu: &Gpu,
    format: vk::Format,
    width: u32,
    height: u32,
) -> Result<RenderTargets> {
    let color_image = resource::Image2d::create(
        gpu,
        format,
        width,
        height,
        vk::SampleCountFlagBits::NUM_1,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;
    Ok(RenderTargets { color_image })
}

unsafe fn destroy_render_targets(gpu: &Gpu, rt: &RenderTargets) {
    rt.color_image.destroy(gpu);
}

//
// Output
//

type OutputBuffer = resource::Buffer<u32>;

struct Output {
    buffer: OutputBuffer,
}

unsafe fn create_output(gpu: &Gpu, format: vk::Format, width: u32, height: u32) -> Result<Output> {
    let element_count = format.block_size() * width * height;
    let buffer = OutputBuffer::create(
        gpu,
        element_count as _,
        vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::HOST_VISIBLE,
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
    Gpu {
        device,
        queue,
        physical_device,
        ..
    }: &Gpu,
    Demo {
        commands,
        render_targets,
        output,
        timestamp_queries,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.command_buffer;
    device.begin_command_buffer(
        cmd,
        &(vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: null(),
        }),
    )?;

    // Begin queries.
    device.cmd_write_timestamp2(
        cmd,
        vk::PipelineStageFlags2::TOP_OF_PIPE,
        timestamp_queries.query_pool,
        0,
    );

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
                src_stage_mask: vk::PipelineStageFlags2::TOP_OF_PIPE,
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color_image.image,
                subresource_range: render_targets.color_image.subresource_range(),
            },
        },
    );

    // Begin rendering.
    let rendering_attachment_info = vk::RenderingAttachmentInfo {
        s_type: vk::StructureType::RenderingAttachmentInfo,
        p_next: null(),
        image_view: render_targets.color_image.image_view,
        image_layout: vk::ImageLayout::AttachmentOptimal,
        resolve_mode: vk::ResolveModeFlagBits::NONE,
        resolve_image_view: vk::ImageView::null(),
        resolve_image_layout: vk::ImageLayout::Undefined,
        load_op: vk::AttachmentLoadOp::Clear,
        store_op: vk::AttachmentStoreOp::Store,
        clear_value: vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [1.0, 0.5, 0.0, 1.0],
            },
        },
    };
    let rendering_info = vk::RenderingInfo {
        s_type: vk::StructureType::RenderingInfo,
        p_next: null(),
        flags: vk::RenderingFlags::empty(),
        render_area: render_targets.color_image.rect_2d(),
        layer_count: 1,
        view_mask: 0,
        color_attachment_count: 1,
        p_color_attachments: &rendering_attachment_info,
        p_depth_attachment: null(),
        p_stencil_attachment: null(),
    };
    device.cmd_begin_rendering(cmd, &rendering_info);

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
                src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                src_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags2::COPY,
                dst_access_mask: vk::AccessFlags2::TRANSFER_READ,
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color_image.image,
                subresource_range: render_targets.color_image.subresource_range(),
            },
        },
    );

    // Copy to output.
    let buffer_image_copy2 = vk::BufferImageCopy2 {
        s_type: vk::StructureType::BufferImageCopy2,
        p_next: null(),
        buffer_offset: 0,
        buffer_row_length: render_targets.color_image.width(),
        buffer_image_height: render_targets.color_image.height(),
        image_subresource: render_targets.color_image.subresource_layers(),
        image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
        image_extent: render_targets.color_image.extent_3d(),
    };
    let copy_image_to_buffer_info2 = vk::CopyImageToBufferInfo2 {
        s_type: vk::StructureType::CopyImageToBufferInfo2,
        p_next: null(),
        src_image: render_targets.color_image.image,
        src_image_layout: vk::ImageLayout::TransferSrcOptimal,
        dst_buffer: output.buffer.buffer,
        region_count: 1,
        p_regions: &buffer_image_copy2,
    };
    device.cmd_copy_image_to_buffer2(cmd, &copy_image_to_buffer_info2);

    // End queries.
    device.cmd_write_timestamp2(
        cmd,
        vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
        timestamp_queries.query_pool,
        1,
    );

    // End command buffer.
    device.end_command_buffer(cmd)?;

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
                stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                device_index: 0,
            }),
        }),
        vk::Fence::null(),
    )?;

    // Wait for semaphore.
    {
        let semaphores = [commands.semaphore];
        let values = [1];
        let semaphore_wait_info = vk::SemaphoreWaitInfo {
            s_type: vk::StructureType::SemaphoreWaitInfo,
            p_next: null(),
            flags: vk::SemaphoreWaitFlags::ANY,
            semaphore_count: semaphores.len() as _,
            p_semaphores: semaphores.as_ptr(),
            p_values: values.as_ptr(),
        };
        device.wait_semaphores(&semaphore_wait_info, u64::MAX)?;
    }

    // Read timestamp.
    {
        let query_count = timestamp_queries.query_pool_create_info.query_count;
        let mut timestamps = vec![0_u64; query_count as _];
        device.get_query_pool_results(
            timestamp_queries.query_pool,
            0,
            query_count,
            size_of::<u64>() * query_count as usize,
            timestamps.as_mut_ptr().cast(),
            size_of::<u64>() as _,
            vk::QueryResultFlags::NUM_64 | vk::QueryResultFlags::WAIT,
        )?;

        let elapsed = timestamps[1]
            .checked_sub(timestamps[0])
            .expect("Later timestamp is larger than earlier timestamp");
        let elapsed_ns = elapsed as f32 * physical_device.properties.limits.timestamp_period;
        let elapsed_ms = elapsed_ns / 1e6;
        info!("Clear took {elapsed_ms} ms, raw={:?}", timestamps);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.color_image.width();
        let height = render_targets.color_image.height();
        let pixels_byte_size = render_targets.color_image.byte_size();
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
