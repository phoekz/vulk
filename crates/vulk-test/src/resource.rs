use super::*;

//
// Upload
//

pub unsafe fn multi_upload_images(
    gpu @ Gpu {
        physical_device,
        device,
        ..
    }: &Gpu,
    images: &[vkx::ImageResource],
    datas: &[&[u8]],
) -> Result<()> {
    // Validation.
    ensure!(!images.is_empty());
    ensure!(!datas.is_empty());
    ensure!(images.len() == datas.len());
    ensure!(images.iter().all(|img| img.byte_size() > 0));
    ensure!(datas.iter().all(|p| !p.is_empty()));
    ensure!(images
        .iter()
        .zip(datas)
        .all(|(img, p)| img.byte_size() as usize == p.len()));

    // Staging buffer.
    let mut staging_buffer = vkx::BufferDedicatedTransfer::create(
        physical_device,
        device,
        vkx::BufferCreator::new(
            datas.iter().map(|data| data.len()).sum::<usize>() as vk::DeviceSize,
            vk::BufferUsageFlagBits::TransferSrc.into(),
        ),
        vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
    )?;

    // Stage datas.
    let mut dst_offset = 0;
    for image_data in datas {
        // Copy.
        std::ptr::copy_nonoverlapping(
            image_data.as_ptr(),
            staging_buffer
                .memory_mut()
                .as_mut_ptr::<u8>()
                .add(dst_offset),
            image_data.len(),
        );

        // Advance.
        dst_offset += image_data.len();
    }

    // Begin staging.
    let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
    let cmd = commands.begin(gpu)?;

    // Transfer commands.
    let mut src_offset = 0;
    for (image, image_data) in images.iter().zip(datas) {
        // Transition Undefined -> TransferDstOptimal.
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
                    src_stage_mask: vk::PipelineStageFlagBits2::Host.into(),
                    src_access_mask: vk::AccessFlags2::empty(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    dst_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    old_layout: vk::ImageLayout::Undefined,
                    new_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: 0,
                    dst_queue_family_index: 0,
                    image: image.image_handle(),
                    subresource_range: image.subresource_range(),
                },
            },
        );
        device.cmd_copy_buffer_to_image2(
            cmd,
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
                    src_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    src_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::FragmentShader.into(),
                    dst_access_mask: vk::AccessFlagBits2::ShaderRead.into(),
                    new_layout: vk::ImageLayout::ReadOnlyOptimal,
                    old_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: 0,
                    dst_queue_family_index: 0,
                    image: image.image_handle(),
                    subresource_range: image.subresource_range(),
                },
            },
        );

        // Advance.
        src_offset += image_data.len();
    }

    // End staging.
    commands.end(gpu)?;
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
                command_buffer: cmd,
                device_mask: 0,
            },
            signal_semaphore_info_count: 1,
            p_signal_semaphore_infos: &vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: commands.semaphore.handle(),
                value: 1,
                stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
                device_index: 0,
            },
        },
        vk::Fence::null(),
    )?;
    commands.semaphore.wait(device, 1, u64::MAX)?;

    // Cleanup.
    commands.destroy(gpu);
    staging_buffer.destroy(device);

    Ok(())
}
