use super::*;

pub unsafe fn queue_submit(
    device: &Device,
    command_buffer: &CommandBuffer,
    wait_semaphores: &[vk::SemaphoreSubmitInfo],
    signal_semaphores: &[vk::SemaphoreSubmitInfo],
) -> Result<()> {
    device.queue_submit2(
        device.queue,
        1,
        &vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: wait_semaphores.len() as _,
            p_wait_semaphore_infos: wait_semaphores.as_ptr(),
            command_buffer_info_count: 1,
            p_command_buffer_infos: &vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer: command_buffer.handle(),
                device_mask: 0,
            },
            signal_semaphore_info_count: signal_semaphores.len() as _,
            p_signal_semaphore_infos: signal_semaphores.as_ptr(),
        },
        vk::Fence::null(),
    )?;
    Ok(())
}
