use super::*;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub semaphore: vk::Semaphore,
}

pub unsafe fn create(
    Gpu {
        device,
        queue_family,
        ..
    }: &Gpu,
) -> Result<Commands> {
    // Command pool.
    let command_pool = {
        let command_pool = device.create_command_pool(
            &(vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::CommandPoolCreateInfo,
                p_next: null(),
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: queue_family.index,
            }),
        )?;
        device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;
        command_pool
    };

    // Command buffer.
    let command_buffer = {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: null(),
            command_pool,
            level: vk::CommandBufferLevel::Primary,
            command_buffer_count: 1,
        };
        let mut command_buffer = MaybeUninit::uninit();
        device
            .allocate_command_buffers(&command_buffer_allocate_info, command_buffer.as_mut_ptr())?;
        command_buffer.assume_init()
    };

    // Semaphore
    let semaphore = {
        let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
            s_type: vk::StructureType::SemaphoreTypeCreateInfo,
            p_next: null(),
            semaphore_type: vk::SemaphoreType::Timeline,
            initial_value: 0,
        };
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: addr_of!(semaphore_type_create_info).cast(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };
        device.create_semaphore(addr_of!(semaphore_create_info).cast())?
    };

    Ok(Commands {
        command_pool,
        command_buffer,
        semaphore,
    })
}

pub unsafe fn destroy(Gpu { device, .. }: &Gpu, commands: &Commands) {
    device.destroy_semaphore(commands.semaphore);
    device.free_command_buffers(
        commands.command_pool,
        1,
        addr_of!(commands.command_buffer).cast(),
    );
    device.destroy_command_pool(commands.command_pool);
}