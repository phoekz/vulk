use super::*;

//
// Gpu
//

pub struct Gpu {
    pub instance: vkx::Instance,
    pub physical_device: vkx::PhysicalDevice,
    pub device: vkx::Device,
}

impl Gpu {
    pub unsafe fn create() -> Result<Self> {
        let instance = vkx::Instance::create(&vkx::InstanceCreateInfo {
            validation_layers: true,
            ..Default::default()
        })
        .context("Creating instance")?;
        let physical_device =
            vkx::PhysicalDevice::create(&instance).context("Creating physical device")?;
        let device =
            vkx::Device::create(&instance, &physical_device, None).context("Creating device")?;

        Ok(Self {
            instance,
            physical_device,
            device,
        })
    }

    pub unsafe fn destroy(self) {
        self.device.destroy();
        self.instance.destroy();
    }
}
