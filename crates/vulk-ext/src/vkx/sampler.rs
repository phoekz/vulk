use super::*;

#[derive(Clone, Copy, Debug)]
pub struct SamplerCreator(vk::SamplerCreateInfo);

impl SamplerCreator {
    #[must_use]
    pub fn new() -> Self {
        Self(vk::SamplerCreateInfo {
            s_type: vk::StructureType::SamplerCreateInfo,
            p_next: null(),
            flags: vk::SamplerCreateFlags::empty(),
            mag_filter: vk::Filter::Nearest,
            min_filter: vk::Filter::Nearest,
            mipmap_mode: vk::SamplerMipmapMode::Nearest,
            address_mode_u: vk::SamplerAddressMode::ClampToEdge,
            address_mode_v: vk::SamplerAddressMode::ClampToEdge,
            address_mode_w: vk::SamplerAddressMode::ClampToEdge,
            mip_lod_bias: 0.0,
            anisotropy_enable: vk::FALSE,
            max_anisotropy: 0.0,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::Always,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: vk::BorderColor::FloatTransparentBlack,
            unnormalized_coordinates: vk::FALSE,
        })
    }

    #[must_use]
    pub fn mag_filter(self, mag_filter: vk::Filter) -> Self {
        Self(vk::SamplerCreateInfo {
            mag_filter,
            ..self.0
        })
    }

    #[must_use]
    pub fn min_filter(self, min_filter: vk::Filter) -> Self {
        Self(vk::SamplerCreateInfo {
            min_filter,
            ..self.0
        })
    }

    #[must_use]
    pub fn mipmap_mode(self, mipmap_mode: vk::SamplerMipmapMode) -> Self {
        Self(vk::SamplerCreateInfo {
            mipmap_mode,
            ..self.0
        })
    }

    #[must_use]
    pub fn address_mode_uvw(self, address_mode: vk::SamplerAddressMode) -> Self {
        Self(vk::SamplerCreateInfo {
            address_mode_u: address_mode,
            address_mode_v: address_mode,
            address_mode_w: address_mode,
            ..self.0
        })
    }

    pub unsafe fn create(self, device: &Device) -> Result<(vk::Sampler, vk::SamplerCreateInfo)> {
        let sampler_create_info = self.0;
        let sampler = device.create_sampler(&sampler_create_info)?;
        Ok((sampler, sampler_create_info))
    }
}

impl Default for SamplerCreator {
    fn default() -> Self {
        Self::new()
    }
}

/// [`SamplerResource`] is meant to be used by a shader.
#[derive(Debug)]
pub struct SamplerResource {
    sampler: vk::Sampler,
    sampler_create_info: vk::SamplerCreateInfo,
    descriptor: Descriptor,
}

impl SamplerResource {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        samplers: &[vk::Sampler],
        sampler_create_infos: &[vk::SamplerCreateInfo],
    ) -> Result<Vec<Self>> {
        // Validation.
        ensure!(!samplers.is_empty());
        ensure!(!sampler_create_infos.is_empty());
        ensure!(samplers.len() == sampler_create_infos.len());

        // Sampler resources.
        let mut sampler_resources = vec![];
        for i in 0..samplers.len() {
            let sampler = samplers[i];
            let sampler_create_info = sampler_create_infos[i];
            let descriptor = Descriptor::create(
                physical_device,
                device,
                DescriptorCreateInfo::Sampler(sampler),
            );
            sampler_resources.push(Self {
                sampler,
                sampler_create_info,
                descriptor,
            });
        }
        Ok(sampler_resources)
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_sampler(self.sampler);
    }

    #[must_use]
    pub fn handle(&self) -> vk::Sampler {
        self.sampler
    }

    #[must_use]
    pub fn create_info(&self) -> &vk::SamplerCreateInfo {
        &self.sampler_create_info
    }

    #[must_use]
    pub fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}
