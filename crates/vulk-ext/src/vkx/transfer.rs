use super::*;

pub unsafe fn transfer_resources(
    physical_device: &PhysicalDevice,
    device: &Device,
    buffers: &[BufferResource],
    buffers_bytes: &[&[u8]],
    images: &[ImageResource],
    images_bytes: &[&[u8]],
) -> Result<()> {
    // Total size.
    let staging_buffer_byte_size = buffers_bytes
        .iter()
        .chain(images_bytes)
        .map(|bytes| bytes.len() as vk::DeviceSize)
        .sum();
    debug!("Staging buffer size {staging_buffer_byte_size}");

    // Staging buffer.
    let mut staging_buffer = vkx::BufferDedicatedTransfer::create(
        physical_device,
        device,
        vkx::BufferCreator::new(
            staging_buffer_byte_size,
            vk::BufferUsageFlagBits::TransferSrc.into(),
        ),
        vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
    )?;

    // Copy into staging buffer.
    let mut dst_offset = 0;
    for bytes in buffers_bytes.iter().chain(images_bytes) {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            staging_buffer
                .memory_mut()
                .as_mut_ptr::<u8>()
                .add(dst_offset),
            bytes.len(),
        );
        dst_offset += bytes.len();
    }

    // Command pool.
    let command_pool = device.create_command_pool(&vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::CommandPoolCreateInfo,
        p_next: null(),
        flags: vk::CommandPoolCreateFlags::empty(),
        queue_family_index: device.queue_family_index,
    })?;

    // Command buffer.
    let command_buffer = {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: null(),
            command_pool,
            level: vk::CommandBufferLevel::Primary,
            command_buffer_count: 1,
        };
        let mut command_buffer = std::mem::MaybeUninit::uninit();
        device
            .allocate_command_buffers(&command_buffer_allocate_info, command_buffer.as_mut_ptr())?;
        command_buffer.assume_init()
    };

    // Semaphore.
    let semaphore = vkx::TimelineSemaphore::create(device, 0)?;

    // Begin.
    device.begin_command_buffer(
        command_buffer,
        &vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: vk::CommandBufferUsageFlagBits::OneTimeSubmit.into(),
            p_inheritance_info: null(),
        },
    )?;

    // Buffer copy commands.
    let mut src_offset = 0;
    for (buffer, buffer_bytes) in buffers.iter().zip(buffers_bytes) {
        // Copy.
        device.cmd_copy_buffer2(
            command_buffer,
            &vk::CopyBufferInfo2 {
                s_type: vk::StructureType::CopyBufferInfo2,
                p_next: null(),
                src_buffer: staging_buffer.buffer_handle(),
                dst_buffer: buffer.buffer_handle(),
                region_count: 1,
                p_regions: &vk::BufferCopy2 {
                    s_type: vk::StructureType::BufferCopy2,
                    p_next: null(),
                    src_offset,
                    dst_offset: 0,
                    size: buffer.size(),
                },
            },
        );

        // Log.
        debug!(
            "Transfering buffer from offset {} of size {}",
            src_offset,
            buffer_bytes.len()
        );

        // Advance.
        src_offset += buffer_bytes.len() as vk::DeviceSize;
    }

    // Image copy commands.
    for (image, image_bytes) in images.iter().zip(images_bytes) {
        // Transition Undefined -> TransferDstOptimal.
        device.cmd_pipeline_barrier2(
            command_buffer,
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
                    src_stage_mask: vk::PipelineStageFlagBits2::Host.into(),
                    src_access_mask: vk::AccessFlags2::empty(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    dst_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    old_layout: vk::ImageLayout::Undefined,
                    new_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: image.image_handle(),
                    subresource_range: image.subresource_range(),
                },
            },
        );

        // Copy.
        device.cmd_copy_buffer_to_image2(
            command_buffer,
            &vk::CopyBufferToImageInfo2 {
                s_type: vk::StructureType::CopyBufferToImageInfo2,
                p_next: null(),
                src_buffer: staging_buffer.buffer_handle(),
                dst_image: image.image_handle(),
                dst_image_layout: vk::ImageLayout::TransferDstOptimal,
                region_count: 1,
                p_regions: &vk::BufferImageCopy2 {
                    s_type: vk::StructureType::BufferImageCopy2,
                    p_next: null(),
                    buffer_offset: src_offset as _,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: image.subresource_layers(),
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: image.extent_3d(),
                },
            },
        );

        // Transition TransferDstOptimal -> ShaderReadOnly.
        device.cmd_pipeline_barrier2(
            command_buffer,
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
                    src_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    src_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::TaskShaderEXT
                        | vk::PipelineStageFlagBits2::MeshShaderEXT
                        | vk::PipelineStageFlagBits2::FragmentShader
                        | vk::PipelineStageFlagBits2::RayTracingShaderKHR,
                    dst_access_mask: vk::AccessFlagBits2::ShaderRead.into(),
                    new_layout: vk::ImageLayout::ReadOnlyOptimal,
                    old_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: image.image_handle(),
                    subresource_range: image.subresource_range(),
                },
            },
        );

        // Log.
        debug!(
            "Transfering image from offset {} of size {}",
            src_offset,
            image_bytes.len()
        );

        // Advance.
        src_offset += image_bytes.len() as vk::DeviceSize;
    }

    // Submit.
    device.end_command_buffer(command_buffer)?;
    device.queue_submit2(
        device.queue,
        1,
        &vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: 0,
            p_wait_semaphore_infos: null(),
            command_buffer_info_count: 1,
            p_command_buffer_infos: &vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer,
                device_mask: 0,
            },
            signal_semaphore_info_count: 1,
            p_signal_semaphore_infos: &vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: semaphore.handle(),
                value: 1,
                stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
                device_index: 0,
            },
        },
        vk::Fence::null(),
    )?;
    semaphore.wait(device, 1, u64::MAX)?;

    // Cleanup.
    semaphore.destroy(device);
    device.destroy_command_pool(command_pool);
    staging_buffer.destroy(device);

    Ok(())
}