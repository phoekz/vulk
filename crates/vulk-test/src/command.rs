use super::*;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub semaphore: vk::Semaphore,
}

impl Commands {
    pub unsafe fn create(
        Gpu {
            device,
            queue_family,
            ..
        }: &Gpu,
    ) -> Result<Self> {
        // Command pool.
        let command_pool = device.create_command_pool(
            &(vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::CommandPoolCreateInfo,
                p_next: null(),
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: queue_family.index,
            }),
        )?;

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
            device.allocate_command_buffers(
                &command_buffer_allocate_info,
                command_buffer.as_mut_ptr(),
            )?;
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

        Ok(Self {
            command_pool,
            command_buffer,
            semaphore,
        })
    }

    pub unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_semaphore(self.semaphore);
        device.free_command_buffers(self.command_pool, 1, addr_of!(self.command_buffer).cast());
        device.destroy_command_pool(self.command_pool);
    }

    pub unsafe fn begin(&self, Gpu { device, .. }: &Gpu) -> Result<vk::CommandBuffer> {
        device.reset_command_pool(self.command_pool, vk::CommandPoolResetFlags::empty())?;
        device.begin_command_buffer(
            self.command_buffer,
            &(vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::CommandBufferBeginInfo,
                p_next: null(),
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                p_inheritance_info: null(),
            }),
        )?;
        Ok(self.command_buffer)
    }

    pub unsafe fn end(&self, Gpu { device, .. }: &Gpu) -> Result<()> {
        device.end_command_buffer(self.command_buffer)?;
        Ok(())
    }
}
