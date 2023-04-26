use super::*;

/// **Example**:
///
/// ```no_run
/// # use vulk::vk as vk;
/// # use vulk_ext::vkx as vkx;
/// # unsafe {
/// # let device = todo!();
/// # let mag_filter = todo!();
/// # let min_filter = todo!();
/// # let mipmap_mode = todo!();
/// # let address_mode = todo!();
/// let (sampler, sampler_create_info) =
///     vkx::SamplerCreator::new()
///         .mag_filter(mag_filter)
///         .min_filter(min_filter)
///         .mipmap_mode(mipmap_mode)
///         .address_mode_uvw(address_mode)
///         .create(device)
///         .unwrap();
/// # }
/// ```
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
