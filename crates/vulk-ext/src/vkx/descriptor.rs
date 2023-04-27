use std::mem::MaybeUninit;

use super::*;

const DESCRIPTOR_MAX_SIZE: usize = 16;

pub(crate) fn validate_descriptor_sizes(
    p: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
) -> Result<()> {
    ensure!(DESCRIPTOR_MAX_SIZE >= p.buffer_capture_replay_descriptor_data_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.image_capture_replay_descriptor_data_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.image_view_capture_replay_descriptor_data_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.sampler_capture_replay_descriptor_data_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.acceleration_structure_capture_replay_descriptor_data_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.sampler_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.combined_image_sampler_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.sampled_image_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.storage_image_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.uniform_texel_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.robust_uniform_texel_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.storage_texel_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.robust_storage_texel_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.uniform_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.robust_uniform_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.storage_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.robust_storage_buffer_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.input_attachment_descriptor_size);
    ensure!(DESCRIPTOR_MAX_SIZE >= p.acceleration_structure_descriptor_size);
    Ok(())
}

pub enum DescriptorCreateInfo {
    UniformBuffer {
        address: vk::DeviceAddress,
        range: vk::DeviceSize,
    },
    StorageBuffer {
        address: vk::DeviceAddress,
        range: vk::DeviceSize,
    },
    SampledImage {
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    },
    StorageImage {
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    },
    InputAttachment {
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    },
    Sampler(vk::Sampler),
    AccelerationStructure(vk::DeviceAddress),
}

impl DescriptorCreateInfo {
    fn size(&self, props: &vk::PhysicalDeviceDescriptorBufferPropertiesEXT) -> usize {
        match self {
            DescriptorCreateInfo::UniformBuffer { .. } => props.uniform_buffer_descriptor_size,
            DescriptorCreateInfo::StorageBuffer { .. } => props.storage_buffer_descriptor_size,
            DescriptorCreateInfo::SampledImage { .. } => props.sampled_image_descriptor_size,
            DescriptorCreateInfo::StorageImage { .. } => props.storage_image_descriptor_size,
            DescriptorCreateInfo::InputAttachment { .. } => props.input_attachment_descriptor_size,
            DescriptorCreateInfo::Sampler(_) => props.sampler_descriptor_size,
            DescriptorCreateInfo::AccelerationStructure(_) => {
                props.acceleration_structure_descriptor_size
            }
        }
    }

    fn ty(&self) -> vk::DescriptorType {
        match self {
            DescriptorCreateInfo::UniformBuffer { .. } => vk::DescriptorType::UniformBuffer,
            DescriptorCreateInfo::StorageBuffer { .. } => vk::DescriptorType::StorageBuffer,
            DescriptorCreateInfo::SampledImage { .. } => vk::DescriptorType::SampledImage,
            DescriptorCreateInfo::StorageImage { .. } => vk::DescriptorType::StorageImage,
            DescriptorCreateInfo::InputAttachment { .. } => vk::DescriptorType::InputAttachment,
            DescriptorCreateInfo::Sampler(_) => vk::DescriptorType::Sampler,
            DescriptorCreateInfo::AccelerationStructure(_) => {
                vk::DescriptorType::AccelerationStructureKHR
            }
        }
    }
}

type DescriptorData = [u8; DESCRIPTOR_MAX_SIZE];

#[derive(Clone, Copy)]
pub struct Descriptor {
    ty: vk::DescriptorType,
    size: usize,
    data: DescriptorData,
}

impl Descriptor {
    #[must_use]
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        create_info: DescriptorCreateInfo,
    ) -> Self {
        // Descriptor info.
        let props = physical_device.descriptor_buffer_properties_ext;
        let size = create_info.size(&props);
        let ty = create_info.ty();

        // Get descriptor data.
        let data = match create_info {
            DescriptorCreateInfo::UniformBuffer { address, range } => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_uniform_buffer: &vk::DescriptorAddressInfoEXT {
                        s_type: vk::StructureType::DescriptorAddressInfoEXT,
                        p_next: null_mut(),
                        address,
                        range,
                        format: vk::Format::Undefined,
                    },
                },
            ),
            DescriptorCreateInfo::StorageBuffer { address, range } => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_storage_buffer: &vk::DescriptorAddressInfoEXT {
                        s_type: vk::StructureType::DescriptorAddressInfoEXT,
                        p_next: null_mut(),
                        address,
                        range,
                        format: vk::Format::Undefined,
                    },
                },
            ),
            DescriptorCreateInfo::SampledImage {
                image_view,
                image_layout,
            } => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_sampled_image: &vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view,
                        image_layout,
                    },
                },
            ),
            DescriptorCreateInfo::StorageImage {
                image_view,
                image_layout,
            } => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_storage_image: &vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view,
                        image_layout,
                    },
                },
            ),
            DescriptorCreateInfo::InputAttachment {
                image_view,
                image_layout,
            } => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_input_attachment_image: &vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view,
                        image_layout,
                    },
                },
            ),
            DescriptorCreateInfo::Sampler(sampler) => Self::get_descriptor_data(
                device,
                ty,
                size,
                vk::DescriptorDataEXT {
                    p_sampler: &sampler,
                },
            ),
            DescriptorCreateInfo::AccelerationStructure(acceleration_structure) => {
                Self::get_descriptor_data(
                    device,
                    ty,
                    size,
                    vk::DescriptorDataEXT {
                        acceleration_structure,
                    },
                )
            }
        };

        Self { ty, size, data }
    }

    unsafe fn get_descriptor_data(
        device: &Device,
        ty: vk::DescriptorType,
        size: usize,
        data: vk::DescriptorDataEXT,
    ) -> DescriptorData {
        let mut descriptor = MaybeUninit::<DescriptorData>::zeroed();
        device.get_descriptor_ext(
            &vk::DescriptorGetInfoEXT {
                s_type: vk::StructureType::DescriptorGetInfoEXT,
                p_next: null(),
                ty,
                data,
            },
            size,
            descriptor.as_mut_ptr().cast(),
        );
        descriptor.assume_init()
    }
}

impl std::fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = &self.data[0..self.size];
        f.debug_struct("Descriptor")
            .field("ty", &self.ty)
            .field("size", &self.size)
            .field("data", &slice)
            .finish()
    }
}

pub struct DescriptorBinding<'a> {
    pub ty: vk::DescriptorType,
    pub stages: vk::ShaderStageFlags,
    pub descriptors: &'a [Descriptor],
}

pub struct DescriptorStorage {
    buffer: vk::Buffer,
    allocations: BufferAllocations,
    allocation: BufferAllocation,
    set_layout: vk::DescriptorSetLayout,
    set_count: u32,
    buffer_indices: Vec<u32>,
    offsets: Vec<vk::DeviceSize>,
}

fn descriptor_buffer_usage() -> vk::BufferUsageFlags {
    vk::BufferUsageFlagBits::ResourceDescriptorBufferEXT
        | vk::BufferUsageFlagBits::SamplerDescriptorBufferEXT
}

impl DescriptorStorage {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        bindings: &[DescriptorBinding],
    ) -> Result<Self> {
        // Validation.
        ensure!(!bindings.is_empty(), "Expected 1 or more bindings");
        for (binding_index, binding) in bindings.iter().enumerate() {
            ensure!(
                !binding.descriptors.is_empty(),
                "Binding {} expected 1 or more descriptors",
                binding_index
            );
            for descriptor in binding.descriptors {
                ensure!(
                    binding.ty == descriptor.ty,
                    "Binding {} expected descriptor type to be equal to {:?}, got {:?} instead",
                    binding_index,
                    binding.ty,
                    descriptor.ty
                );
            }
        }

        // Descriptor set layout.
        let set_layout_bindings = bindings
            .iter()
            .enumerate()
            .map(|(binding_index, binding)| vk::DescriptorSetLayoutBinding {
                binding: binding_index as _,
                descriptor_type: binding.ty,
                descriptor_count: binding.descriptors.len() as _,
                stage_flags: binding.stages,
                p_immutable_samplers: null(),
            })
            .collect::<Vec<_>>();
        let set_layout =
            device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo {
                s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
                p_next: null(),
                flags: vk::DescriptorSetLayoutCreateFlagBits::DescriptorBufferEXT.into(),
                binding_count: set_layout_bindings.len() as _,
                p_bindings: set_layout_bindings.as_ptr(),
            })?;
        let set_count = 1;
        let buffer_indices = vec![0];
        let offsets = vec![0];
        let size = device.get_descriptor_set_layout_size_ext(set_layout);

        // Buffer.
        let (buffer, buffer_create_info) = BufferCreator::new(size, descriptor_buffer_usage())
            .create(device)
            .context("Creating buffer object")?;

        // Allocate.
        let allocations = BufferAllocations::allocate(
            physical_device,
            device,
            &[buffer],
            &[buffer_create_info],
            vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
        )?;
        let allocation = allocations.allocations()[0];

        // Write descriptors.
        for (binding_index, binding) in bindings.iter().enumerate() {
            let binding_index = binding_index as u32;
            let descriptor_offset =
                device.get_descriptor_set_layout_binding_offset_ext(set_layout, binding_index);
            let descriptor_offset = descriptor_offset as usize;
            for (array_index, descriptor) in binding.descriptors.iter().enumerate() {
                let dst_offset = descriptor_offset + array_index * descriptor.size;
                std::ptr::copy_nonoverlapping(
                    descriptor.data.as_ptr(),
                    allocation.as_mut_ptr::<u8>().add(dst_offset),
                    descriptor.size,
                );
            }
        }

        Ok(Self {
            buffer,
            allocations,
            allocation,
            set_layout,
            set_count,
            buffer_indices,
            offsets,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_descriptor_set_layout(self.set_layout);
        device.destroy_buffer(self.buffer);
        self.allocations.free(device);
    }

    #[must_use]
    pub fn set_layout(&self) -> vk::DescriptorSetLayout {
        self.set_layout
    }

    pub unsafe fn bind(&self, device: &Device, cmd: vk::CommandBuffer) {
        device.cmd_bind_descriptor_buffers_ext(
            cmd,
            1,
            &vk::DescriptorBufferBindingInfoEXT {
                s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
                p_next: null_mut(),
                address: self.allocation.device_address(),
                usage: descriptor_buffer_usage(),
            },
        );
    }

    pub unsafe fn set_offsets(
        &self,
        device: &Device,
        cmd: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
    ) {
        device.cmd_set_descriptor_buffer_offsets_ext(
            cmd,
            pipeline_bind_point,
            layout,
            0,
            self.set_count,
            self.buffer_indices.as_ptr(),
            self.offsets.as_ptr(),
        );
    }
}
