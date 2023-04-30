use super::*;

#[derive(Debug)]
pub struct Swapchain {
    swapchain: vk::SwapchainKHR,
    create_info: vk::SwapchainCreateInfoKHR,
    images: Vec<(vk::Image, vk::ImageView)>,
}

impl Swapchain {
    pub unsafe fn create(device: &Device, surface: &Surface) -> Result<Self> {
        // Swapchain create info.
        let create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKHR,
            p_next: null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface.handle(),
            min_image_count: surface.surface_capabilities.min_image_count,
            image_format: surface.surface_format.format,
            image_color_space: surface.surface_format.color_space,
            image_extent: vk::Extent2D {
                width: surface.surface_capabilities.min_image_extent.width,
                height: surface.surface_capabilities.min_image_extent.height,
            },
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlagBits::ColorAttachment.into(),
            image_sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
            pre_transform: vk::SurfaceTransformFlagBitsKHR::IdentityKHR,
            composite_alpha: vk::CompositeAlphaFlagBitsKHR::OpaqueKHR,
            present_mode: surface.present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
        };

        // Swapchain.
        let swapchain = device.create_swapchain_khr(&create_info)?;

        // Swapchain images.
        let mut images = vec![];
        for image in vulk::read_to_vec(
            |count, ptr| device.get_swapchain_images_khr(swapchain, count, ptr),
            None,
        )? {
            let (image_view, _) =
                ImageViewCreator::new_2d(image, surface.surface_format.format).create(device)?;
            images.push((image, image_view));
        }

        Ok(Self {
            swapchain,
            create_info,
            images,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_swapchain_khr(self.swapchain);
        for &(_, image_view) in &self.images {
            device.destroy_image_view(image_view);
        }
    }

    #[must_use]
    pub fn handle(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    #[must_use]
    pub fn image_count(&self) -> u64 {
        self.images.len() as u64
    }

    #[must_use]
    pub fn image(&self, image_index: u32) -> vk::Image {
        self.images[image_index as usize].0
    }

    #[must_use]
    pub fn image_view(&self, image_index: u32) -> vk::ImageView {
        self.images[image_index as usize].1
    }

    #[must_use]
    pub fn image_subresource_range(&self) -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange {
            aspect_mask: self.create_info.image_format.aspect_mask(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        }
    }

    #[must_use]
    pub fn render_area(&self) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.create_info.image_extent,
        }
    }
}
