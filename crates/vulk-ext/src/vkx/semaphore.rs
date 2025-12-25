use super::*;

pub trait SemaphoreOps {
    fn handle(&self) -> vk::Semaphore;

    fn submit_info(
        &self,
        value: u64,
        stage_mask: impl Into<vk::PipelineStageFlags2> + Copy,
    ) -> vk::SemaphoreSubmitInfo {
        vk::SemaphoreSubmitInfo {
            s_type: vk::StructureType::SemaphoreSubmitInfo,
            p_next: null(),
            semaphore: self.handle(),
            value,
            stage_mask: stage_mask.into(),
            device_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct TimelineSemaphore {
    semaphore: vk::Semaphore,
}

impl TimelineSemaphore {
    pub unsafe fn create(device: &Device, initial_value: u64) -> Result<Self> {
        let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
            s_type: vk::StructureType::SemaphoreTypeCreateInfo,
            p_next: null(),
            semaphore_type: vk::SemaphoreType::Timeline,
            initial_value,
        };
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: addr_of!(semaphore_type_create_info).cast(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };
        let semaphore = device.create_semaphore(&raw const semaphore_create_info)?;
        Ok(Self { semaphore })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_semaphore(self.semaphore);
    }

    pub unsafe fn wait(&self, device: &Device, value: u64, timeout: u64) -> Result<()> {
        device.wait_semaphores(
            &vk::SemaphoreWaitInfo {
                s_type: vk::StructureType::SemaphoreWaitInfo,
                p_next: null(),
                flags: vk::SemaphoreWaitFlagBits::Any.into(),
                semaphore_count: 1,
                p_semaphores: &raw const self.semaphore,
                p_values: [value].as_ptr(),
            },
            timeout,
        )?;
        Ok(())
    }
}

impl SemaphoreOps for TimelineSemaphore {
    fn handle(&self) -> vk::Semaphore {
        self.semaphore
    }
}

/// [`BinarySemaphore`] is only intended to be used with WSI. Prefer
/// [`TimelineSemaphore`] for everything else.
#[derive(Debug)]
pub struct BinarySemaphore {
    semaphore: vk::Semaphore,
}

impl BinarySemaphore {
    pub unsafe fn create(device: &Device) -> Result<Self> {
        let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
            s_type: vk::StructureType::SemaphoreTypeCreateInfo,
            p_next: null(),
            semaphore_type: vk::SemaphoreType::Binary,
            initial_value: 0,
        };
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: addr_of!(semaphore_type_create_info).cast(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };
        let semaphore = device.create_semaphore(&raw const semaphore_create_info)?;
        Ok(Self { semaphore })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_semaphore(self.semaphore);
    }
}

impl SemaphoreOps for BinarySemaphore {
    fn handle(&self) -> vk::Semaphore {
        self.semaphore
    }
}
