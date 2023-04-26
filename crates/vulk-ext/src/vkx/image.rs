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

pub trait ImageOps {
    fn image_handle(&self) -> vk::Image;

    fn image_view_handle(&self) -> vk::ImageView;

    fn image_create_info(&self) -> &vk::ImageCreateInfo;

    fn image_view_create_info(&self) -> &vk::ImageViewCreateInfo;

    fn descriptor(&self) -> Descriptor;

    fn format(&self) -> vk::Format {
        self.image_create_info().format
    }

    fn extent_2d(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.extent_3d().width,
            height: self.extent_3d().height,
        }
    }

    fn extent_3d(&self) -> vk::Extent3D {
        self.image_create_info().extent
    }

    fn width(&self) -> u32 {
        self.extent_3d().width
    }

    fn height(&self) -> u32 {
        self.extent_3d().height
    }

    fn depth(&self) -> u32 {
        self.extent_3d().depth
    }

    fn byte_size(&self) -> u32 {
        self.format().block_size() * self.width() * self.height() * self.depth()
    }

    fn rect_2d(&self) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.extent_2d(),
        }
    }

    fn subresource_range(&self) -> vk::ImageSubresourceRange {
        self.image_view_create_info().subresource_range
    }

    fn subresource_layers(&self) -> vk::ImageSubresourceLayers {
        let subresource_range = self.subresource_range();
        vk::ImageSubresourceLayers {
            aspect_mask: subresource_range.aspect_mask,
            mip_level: 0,
            base_array_layer: subresource_range.base_array_layer,
            layer_count: subresource_range.layer_count,
        }
    }
}

/// [`ImageResource`] is meant to be used by a shader.
#[derive(Debug)]
pub struct ImageResource {
    image: vk::Image,
    image_view: vk::ImageView,
    image_create_info: vk::ImageCreateInfo,
    image_view_create_info: vk::ImageViewCreateInfo,
    descriptor: Descriptor,
}

impl ImageResource {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        images: &[vk::Image],
        image_views: &[vk::ImageView],
        image_create_infos: &[vk::ImageCreateInfo],
        image_view_create_infos: &[vk::ImageViewCreateInfo],
    ) -> Result<Vec<Self>> {
        // Constants.
        const SAMPLED_IMAGE: vk::ImageUsageFlagBits = vk::ImageUsageFlagBits::Sampled;
        const STORAGE_IMAGE: vk::ImageUsageFlagBits = vk::ImageUsageFlagBits::Storage;
        const INPUT_ATTACHMENT: vk::ImageUsageFlagBits = vk::ImageUsageFlagBits::InputAttachment;

        // Validation.
        ensure!(!images.is_empty());
        ensure!(!image_views.is_empty());
        ensure!(!image_create_infos.is_empty());
        ensure!(!image_view_create_infos.is_empty());
        ensure!(images.len() == image_views.len());
        ensure!(images.len() == image_create_infos.len());
        ensure!(images.len() == image_view_create_infos.len());

        // Image resources.
        let mut image_resources = vec![];
        for i in 0..images.len() {
            let image = images[i];
            let image_view = image_views[i];
            let image_create_info = image_create_infos[i];
            let image_view_create_info = image_view_create_infos[i];
            let usage = image_create_info.usage;
            let descriptor = if usage.contains(SAMPLED_IMAGE.into()) {
                vkx::Descriptor::create(
                    physical_device,
                    device,
                    vkx::DescriptorCreateInfo::SampledImage {
                        image_view,
                        image_layout: vk::ImageLayout::ReadOnlyOptimal,
                    },
                )
            } else if usage.contains(STORAGE_IMAGE.into()) {
                vkx::Descriptor::create(
                    physical_device,
                    device,
                    vkx::DescriptorCreateInfo::StorageImage {
                        image_view,
                        image_layout: vk::ImageLayout::General,
                    },
                )
            } else if usage.contains(INPUT_ATTACHMENT.into()) {
                vkx::Descriptor::create(
                    physical_device,
                    device,
                    vkx::DescriptorCreateInfo::InputAttachment {
                        image_view,
                        image_layout: vk::ImageLayout::ReadOnlyOptimal,
                    },
                )
            } else {
                bail!(
                    "Image resource must be \
                    {SAMPLED_IMAGE} or \
                    {STORAGE_IMAGE} or \
                    {INPUT_ATTACHMENT}, \
                    got {usage}"
                );
            };
            image_resources.push(Self {
                image,
                image_view,
                image_create_info,
                image_view_create_info,
                descriptor,
            });
        }
        Ok(image_resources)
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_image_view(self.image_view);
        device.destroy_image(self.image);
    }
}

impl ImageOps for ImageResource {
    fn image_handle(&self) -> vk::Image {
        self.image
    }

    fn image_view_handle(&self) -> vk::ImageView {
        self.image_view
    }

    fn image_create_info(&self) -> &vk::ImageCreateInfo {
        &self.image_create_info
    }

    fn image_view_create_info(&self) -> &vk::ImageViewCreateInfo {
        &self.image_view_create_info
    }

    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

/// [`ImageDedicatedResource`] trades off allocation efficiency for ease of use.
#[derive(Debug)]
pub struct ImageDedicatedResource {
    image_resource: ImageResource,
    image_allocations: ImageAllocations,
}

impl ImageDedicatedResource {
    pub unsafe fn create_2d(
        physical_device: &PhysicalDevice,
        device: &Device,
        format: vk::Format,
        width: u32,
        height: u32,
        samples: vk::SampleCountFlagBits,
        usage: vk::ImageUsageFlags,
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Image.
        let (image, image_create_info) = ImageCreator::new_2d(width, height, format, usage)
            .samples(samples)
            .create(device)?;

        // Allocation.
        let image_allocations = ImageAllocations::allocate(
            physical_device,
            device,
            &[image],
            &[image_create_info],
            property_flags,
        )?;

        // Image view.
        let (image_view, image_view_create_info) =
            ImageViewCreator::new_2d(image, image_create_info.format).create(device)?;

        // Image resource.
        let mut image_resources = ImageResource::create(
            physical_device,
            device,
            &[image],
            &[image_view],
            &[image_create_info],
            &[image_view_create_info],
        )?;
        let image_resource = image_resources.swap_remove(0);

        Ok(Self {
            image_resource,
            image_allocations,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        self.image_resource.destroy(device);
        self.image_allocations.free(device);
    }
}

impl ImageOps for ImageDedicatedResource {
    fn image_handle(&self) -> vk::Image {
        self.image_resource.image
    }

    fn image_view_handle(&self) -> vk::ImageView {
        self.image_resource.image_view
    }

    fn image_create_info(&self) -> &vk::ImageCreateInfo {
        &self.image_resource.image_create_info
    }

    fn image_view_create_info(&self) -> &vk::ImageViewCreateInfo {
        &self.image_resource.image_view_create_info
    }

    fn descriptor(&self) -> Descriptor {
        self.image_resource.descriptor
    }
}
