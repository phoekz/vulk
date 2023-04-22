use super::*;

//
// Descriptor
//

pub const MAX_DESCRIPTOR_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub struct Descriptor {
    data: [u8; MAX_DESCRIPTOR_SIZE],
    size: usize,
    ty: vk::DescriptorType,
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
            ty,
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

    pub unsafe fn create_sampled_image(
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

    pub unsafe fn create_storage_image(
        gpu: &Gpu,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::StorageImage,
            &vk::DescriptorDataEXT {
                p_storage_image: &vk::DescriptorImageInfo {
                    sampler: vk::Sampler::null(),
                    image_view,
                    image_layout,
                },
            },
        )
    }

    pub unsafe fn create_sampler(gpu: &Gpu, sampler: vk::Sampler) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::Sampler,
            &vk::DescriptorDataEXT {
                p_sampler: &sampler,
            },
        )
    }

    pub unsafe fn create_acceleration_structure(
        gpu: &Gpu,
        acceleration_structure: vk::DeviceAddress,
    ) -> Self {
        Self::create(
            gpu,
            vk::DescriptorType::AccelerationStructureKHR,
            &vk::DescriptorDataEXT {
                acceleration_structure,
            },
        )
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn byte_size(&self) -> usize {
        self.size
    }

    pub fn ty(&self) -> vk::DescriptorType {
        self.ty
    }
}

//
// Descriptor storage
//

pub struct DescriptorStorageBinding<'a> {
    pub descriptor_type: vk::DescriptorType,
    pub stage_flags: vk::ShaderStageFlags,
    pub descriptors: &'a [Descriptor],
}

pub struct DescriptorStorageCreateInfo<'a> {
    pub bindings: &'a [DescriptorStorageBinding<'a>],
}

pub type DescriptorBuffer = resource::Buffer<u8>;

#[derive(Debug)]
pub struct DescriptorStorage {
    set_layout: vk::DescriptorSetLayout,
    buffer: DescriptorBuffer,
}

impl GpuResource for DescriptorStorage {
    type CreateInfo<'a> = DescriptorStorageCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self> {
        // Validate.
        ensure!(
            !create_info.bindings.is_empty(),
            "Expected 1 or more bindings"
        );
        for (binding_index, binding) in create_info.bindings.iter().enumerate() {
            ensure!(
                !binding.descriptors.is_empty(),
                "Binding {} expected 1 or more descriptors",
                binding_index
            );
            for descriptor in binding.descriptors {
                ensure!(
                    binding.descriptor_type == descriptor.ty(),
                    "Binding {} expected descriptor type to be equal to {:?}, got {:?} instead",
                    binding_index,
                    binding.descriptor_type,
                    descriptor.ty()
                );
            }
        }

        // Descriptor set layout.
        let bindings = create_info
            .bindings
            .iter()
            .enumerate()
            .map(|(binding_index, binding)| vk::DescriptorSetLayoutBinding {
                binding: binding_index as _,
                descriptor_type: binding.descriptor_type,
                descriptor_count: binding.descriptors.len() as _,
                stage_flags: binding.stage_flags,
                p_immutable_samplers: null(),
            })
            .collect::<Vec<_>>();
        let set_layout = gpu.device.create_descriptor_set_layout(
            &(vk::DescriptorSetLayoutCreateInfo {
                s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
                p_next: null(),
                flags: vk::DescriptorSetLayoutCreateFlagBits::DescriptorBufferEXT.into(),
                binding_count: bindings.len() as _,
                p_bindings: bindings.as_ptr(),
            }),
        )?;

        // Buffer.
        let buffer = DescriptorBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: gpu.device.get_descriptor_set_layout_size_ext(set_layout) as _,
                usage: vk::BufferUsageFlagBits::ResourceDescriptorBufferEXT
                    | vk::BufferUsageFlagBits::SamplerDescriptorBufferEXT,
                property_flags: vk::MemoryPropertyFlagBits::HostVisible
                    | vk::MemoryPropertyFlagBits::HostCoherent,
            },
        )?;
        debug!("set_layout={}", buffer.size);

        // Write.
        for (binding_index, binding) in create_info.bindings.iter().enumerate() {
            let descriptor_offset = gpu
                .device
                .get_descriptor_set_layout_binding_offset_ext(set_layout, binding_index as _);
            debug!(
                "index={binding_index}, offset={descriptor_offset}, type={:?}",
                binding.descriptor_type
            );
            for (array_index, descriptor) in binding.descriptors.iter().enumerate() {
                let dst_offset = descriptor_offset as usize + array_index * descriptor.byte_size();
                debug!(
                    "  index={array_index}, size={}, dst={dst_offset}",
                    descriptor.size
                );
                std::ptr::copy_nonoverlapping(
                    descriptor.as_ptr(),
                    buffer.ptr.add(dst_offset),
                    descriptor.byte_size(),
                );
            }
        }

        Ok(Self { set_layout, buffer })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        gpu.device.destroy_descriptor_set_layout(self.set_layout);
        self.buffer.destroy(gpu);
    }
}

impl DescriptorStorage {
    pub fn set_layout(&self) -> vk::DescriptorSetLayout {
        self.set_layout
    }

    pub unsafe fn bind(&self, gpu: &Gpu, cmd: vk::CommandBuffer) {
        gpu.device.cmd_bind_descriptor_buffers_ext(
            cmd,
            1,
            &vk::DescriptorBufferBindingInfoEXT {
                s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
                p_next: null_mut(),
                address: self.buffer.device_address,
                usage: self.buffer.buffer_create_info.usage,
            },
        );
    }

    #[allow(clippy::unused_self)]
    pub unsafe fn set_offsets(
        &self,
        gpu: &Gpu,
        cmd: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
    ) {
        gpu.device.cmd_set_descriptor_buffer_offsets_ext(
            cmd,
            pipeline_bind_point,
            layout,
            0,
            1,
            [0].as_ptr(),
            [0].as_ptr(),
        );
    }
}

//
// Utilities
//

pub fn validate_descriptor_sizes(
    properties: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
) -> Result<()> {
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.buffer_capture_replay_descriptor_data_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.image_capture_replay_descriptor_data_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.image_view_capture_replay_descriptor_data_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.sampler_capture_replay_descriptor_data_size);
    ensure!(
        MAX_DESCRIPTOR_SIZE
            >= properties.acceleration_structure_capture_replay_descriptor_data_size
    );
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.sampler_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.combined_image_sampler_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.sampled_image_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.storage_image_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.uniform_texel_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.robust_uniform_texel_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.storage_texel_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.robust_storage_texel_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.uniform_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.robust_uniform_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.storage_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.robust_storage_buffer_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.input_attachment_descriptor_size);
    ensure!(MAX_DESCRIPTOR_SIZE >= properties.acceleration_structure_descriptor_size);
    Ok(())
}

fn descriptor_size(
    properties: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
    ty: vk::DescriptorType,
) -> usize {
    match ty {
        vk::DescriptorType::Sampler => properties.sampler_descriptor_size,
        vk::DescriptorType::SampledImage => properties.sampled_image_descriptor_size,
        vk::DescriptorType::StorageImage => properties.storage_image_descriptor_size,
        vk::DescriptorType::StorageBuffer => properties.storage_buffer_descriptor_size,
        vk::DescriptorType::AccelerationStructureKHR => {
            properties.acceleration_structure_descriptor_size
        }
        _ => {
            panic!("Unsupported descriptor type={ty:?}");
        }
    }
}
