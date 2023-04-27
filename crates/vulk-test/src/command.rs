use super::*;

pub struct CommandsCreateInfo;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub semaphore: vkx::TimelineSemaphore,
}

impl GpuResource for Commands {
    type CreateInfo<'a> = CommandsCreateInfo;

    unsafe fn create(Gpu { device, .. }: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self> {
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
            let mut command_buffer = MaybeUninit::uninit();
            device.allocate_command_buffers(
                &command_buffer_allocate_info,
                command_buffer.as_mut_ptr(),
            )?;
            command_buffer.assume_init()
        };

        // Semaphore.
        let semaphore = vkx::TimelineSemaphore::create(device, 0)?;

        Ok(Self {
            command_pool,
            command_buffer,
            semaphore,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        self.semaphore.destroy(device);
        device.free_command_buffers(self.command_pool, 1, &self.command_buffer);
        device.destroy_command_pool(self.command_pool);
    }
}

impl Commands {
    pub unsafe fn begin(&self, Gpu { device, .. }: &Gpu) -> Result<vk::CommandBuffer> {
        device.reset_command_pool(self.command_pool, vk::CommandPoolResetFlags::empty())?;
        device.begin_command_buffer(
            self.command_buffer,
            &vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::CommandBufferBeginInfo,
                p_next: null(),
                flags: vk::CommandBufferUsageFlagBits::OneTimeSubmit.into(),
                p_inheritance_info: null(),
            },
        )?;
        Ok(self.command_buffer)
    }

    pub unsafe fn end(&self, Gpu { device, .. }: &Gpu) -> Result<()> {
        device.end_command_buffer(self.command_buffer)?;
        Ok(())
    }

    pub unsafe fn submit_and_wait(
        &self,
        Gpu { device, .. }: &Gpu,
        value: u64,
        signal_stage_mask: vk::PipelineStageFlags2,
    ) -> Result<()> {
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
                    command_buffer: self.command_buffer,
                    device_mask: 0,
                },
                signal_semaphore_info_count: 1,
                p_signal_semaphore_infos: &vk::SemaphoreSubmitInfo {
                    s_type: vk::StructureType::SemaphoreSubmitInfo,
                    p_next: null(),
                    semaphore: self.semaphore.handle(),
                    value,
                    stage_mask: signal_stage_mask,
                    device_index: 0,
                },
            },
            vk::Fence::null(),
        )?;
        self.semaphore.wait(device, value, u64::MAX)?;

        Ok(())
    }
}
