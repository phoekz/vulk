use super::*;

pub const MAX_DESCRIPTOR_SIZE: usize = 16;

pub struct Descriptor {
    data: [u8; MAX_DESCRIPTOR_SIZE],
    size: usize,
}

impl std::fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = &self.data[0..self.size];
        f.debug_struct("Descriptor")
            .field("data", &slice)
            .field("size", &self.size)
            .finish()
    }
}

impl Descriptor {
    pub unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        ty: vk::DescriptorType,
        data: &vk::DescriptorDataEXT,
    ) -> Self {
        let descriptor_size =
            descriptor_size(&physical_device.descriptor_buffer_properties_ext, ty);
        let mut descriptor: Self = Self {
            data: Default::default(),
            size: Default::default(),
        };
        device.get_descriptor_ext(
            &(vk::DescriptorGetInfoEXT {
                s_type: vk::StructureType::DescriptorGetInfoEXT,
                p_next: null(),
                ty,
                data: *data,
            }),
            descriptor_size,
            descriptor.data.as_mut_ptr().cast(),
        );
        descriptor.size = descriptor_size;
        descriptor
    }

    pub unsafe fn create_storage_buffer(
        gpu: &Gpu,
        address: vk::DeviceAddress,
        range: vk::DeviceSize,
    ) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::StorageBuffer,
            &vk::DescriptorDataEXT {
                p_storage_buffer: &(vk::DescriptorAddressInfoEXT {
                    s_type: vk::StructureType::DescriptorAddressInfoEXT,
                    p_next: null_mut(),
                    address,
                    range,
                    format: vk::Format::Undefined,
                }),
            },
        )
    }

    pub(crate) unsafe fn create_sampled_image(
        gpu: &Gpu,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::SampledImage,
            &vk::DescriptorDataEXT {
                p_sampled_image: &vk::DescriptorImageInfo {
                    sampler: vk::Sampler::null(),
                    image_view,
                    image_layout,
                },
            },
        )
    }

    pub(crate) unsafe fn create_sampler(gpu: &Gpu, sampler: vk::Sampler) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::Sampler,
            &vk::DescriptorDataEXT {
                p_sampler: &sampler,
            },
        )
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn byte_size(&self) -> usize {
        self.size
    }
}

pub fn assert_descriptor_sizes(properties: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT) {
    assert!(MAX_DESCRIPTOR_SIZE >= properties.buffer_capture_replay_descriptor_data_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.image_capture_replay_descriptor_data_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.image_view_capture_replay_descriptor_data_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.sampler_capture_replay_descriptor_data_size);
    assert!(
        MAX_DESCRIPTOR_SIZE
            >= properties.acceleration_structure_capture_replay_descriptor_data_size
    );
    assert!(MAX_DESCRIPTOR_SIZE >= properties.sampler_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.combined_image_sampler_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.sampled_image_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.storage_image_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.uniform_texel_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.robust_uniform_texel_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.storage_texel_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.robust_storage_texel_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.uniform_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.robust_uniform_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.storage_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.robust_storage_buffer_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.input_attachment_descriptor_size);
    assert!(MAX_DESCRIPTOR_SIZE >= properties.acceleration_structure_descriptor_size);
}

fn descriptor_size(
    properties: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
    ty: vk::DescriptorType,
) -> usize {
    match ty {
        vk::DescriptorType::Sampler => properties.sampler_descriptor_size,
        vk::DescriptorType::SampledImage => properties.sampled_image_descriptor_size,
        vk::DescriptorType::StorageBuffer => properties.storage_buffer_descriptor_size,
        _ => {
            panic!("Unsupported descriptor type={ty:?}");
        }
    }
}
