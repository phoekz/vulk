use super::*;

/// **Example**:
///
/// ```no_run
/// # use vulk::vk as vk;
/// # use vulk_ext::vkx as vkx;
/// # unsafe {
/// # let device = todo!();
/// # let width = todo!();
/// # let height = todo!();
/// # let format = todo!();
/// # let usage = todo!();
/// let (image, image_create_info) =
///     vkx::ImageCreator::new_2d(width, height, format, usage)
///         .create(device)
///         .unwrap();
/// # }
/// ```
pub struct ImageCreator(vk::ImageCreateInfo);

impl ImageCreator {
    #[must_use]
    pub fn new_2d(width: u32, height: u32, format: vk::Format, usage: vk::ImageUsageFlags) -> Self {
        Self(vk::ImageCreateInfo {
            s_type: vk::StructureType::ImageCreateInfo,
            p_next: null(),
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::Type2d,
            format,
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlagBits::Count1,
            tiling: vk::ImageTiling::Optimal,
            usage,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
            initial_layout: vk::ImageLayout::Undefined,
        })
    }

    #[must_use]
    pub fn samples(self, samples: vk::SampleCountFlagBits) -> Self {
        Self(vk::ImageCreateInfo { samples, ..self.0 })
    }

    pub unsafe fn create(self, device: &Device) -> Result<(vk::Image, vk::ImageCreateInfo)> {
        let image_create_info = self.0;
        let image = device.create_image(&image_create_info)?;
        Ok((image, image_create_info))
    }
}

/// **Example**:
///
/// ```no_run
/// # use vulk::vk as vk;
/// # use vulk_ext::vkx as vkx;
/// # unsafe {
/// # let device = todo!();
/// # let image = todo!();
/// # let format = todo!();
/// let (image_view, image_view_create_info) =
///     vkx::ImageViewCreator::new_2d(image, format)
///         .create(device)
///         .unwrap();
/// # }
/// ```
pub struct ImageViewCreator(vk::ImageViewCreateInfo);

impl ImageViewCreator {
    #[must_use]
    pub fn new_2d(image: vk::Image, format: vk::Format) -> Self {
        Self(vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type: vk::ImageViewType::Type2d,
            format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::Identity,
                g: vk::ComponentSwizzle::Identity,
                b: vk::ComponentSwizzle::Identity,
                a: vk::ComponentSwizzle::Identity,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: format.aspect_mask(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        })
    }

    pub unsafe fn create(
        self,
        device: &Device,
    ) -> Result<(vk::ImageView, vk::ImageViewCreateInfo)> {
        let image_view_create_info = self.0;
        let image_view = device.create_image_view(&image_view_create_info)?;
        Ok((image_view, image_view_create_info))
    }
}
