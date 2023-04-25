use super::*;

pub struct CommandsCreateInfo;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub semaphore: vk::Semaphore,
}

impl GpuResource for Commands {
    type CreateInfo<'a> = CommandsCreateInfo;

    unsafe fn create(
        Gpu {
            device,
            queue_family,
            ..
        }: &Gpu,
        _: &Self::CreateInfo<'_>,
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

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_semaphore(self.semaphore);
        device.free_command_buffers(self.command_pool, 1, addr_of!(self.command_buffer).cast());
        device.destroy_command_pool(self.command_pool);
    }
}

impl Commands {
    pub unsafe fn begin(&self, Gpu { device, .. }: &Gpu) -> Result<vk::CommandBuffer> {
        device.reset_command_pool(self.command_pool, vk::CommandPoolResetFlags::empty())?;
        device.begin_command_buffer(
            self.command_buffer,
            &(vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::CommandBufferBeginInfo,
                p_next: null(),
                flags: vk::CommandBufferUsageFlagBits::OneTimeSubmit.into(),
                p_inheritance_info: null(),
            }),
        )?;
        Ok(self.command_buffer)
    }

    pub unsafe fn end(&self, Gpu { device, .. }: &Gpu) -> Result<()> {
        device.end_command_buffer(self.command_buffer)?;
        Ok(())
    }

    pub unsafe fn submit_and_wait(
        &self,
        Gpu { device, queue, .. }: &Gpu,
        signal_stage_mask: vk::PipelineStageFlags2,
    ) -> Result<()> {
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
                    command_buffer: self.command_buffer,
                    device_mask: 0,
                }),
                signal_semaphore_info_count: 1,
                p_signal_semaphore_infos: &(vk::SemaphoreSubmitInfo {
                    s_type: vk::StructureType::SemaphoreSubmitInfo,
                    p_next: null(),
                    semaphore: self.semaphore,
                    value: 1,
                    stage_mask: signal_stage_mask,
                    device_index: 0,
                }),
            }),
            vk::Fence::null(),
        )?;

        let semaphores = [self.semaphore];
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

        Ok(())
    }
}
