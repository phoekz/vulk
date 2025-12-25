use super::*;

#[derive(Clone, Copy, Debug)]
pub struct BufferCreator(vk::BufferCreateInfo);

impl BufferCreator {
    #[must_use]
    pub fn new(size: vk::DeviceSize, usage: impl Into<vk::BufferUsageFlags> + Copy) -> Self {
        Self(vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage: usage.into() | vk::BufferUsageFlagBits::ShaderDeviceAddress,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
        })
    }

    pub unsafe fn create(self, device: &Device) -> Result<(vk::Buffer, vk::BufferCreateInfo)> {
        let buffer_create_info = self.0;
        let buffer = device.create_buffer(&raw const buffer_create_info)?;
        Ok((buffer, buffer_create_info))
    }
}

pub trait BufferOps {
    fn buffer_handle(&self) -> vk::Buffer;

    fn create_info(&self) -> &vk::BufferCreateInfo;

    fn memory(&self) -> &BufferAllocation;

    fn memory_mut(&mut self) -> &mut BufferAllocation;

    fn size(&self) -> vk::DeviceSize {
        self.create_info().size
    }
}

pub trait BufferResourceOps {
    fn descriptor(&self) -> Descriptor;
}

/// [`BufferResource`] is meant to be used by a shader.
#[derive(Debug)]
pub struct BufferResource {
    buffer: vk::Buffer,
    buffer_create_info: vk::BufferCreateInfo,
    buffer_allocation: BufferAllocation,
    descriptor: Descriptor,
}

impl BufferResource {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        buffer_creators: &[BufferCreator],
        property_flags: impl Into<vk::MemoryPropertyFlags> + Copy,
    ) -> Result<(Vec<Self>, BufferAllocations)> {
        // Constants.
        const UNIFORM_BUFFER: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::UniformBuffer;
        const STORAGE_BUFFER: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::StorageBuffer;
        const AS_BUFFER: vk::BufferUsageFlagBits =
            vk::BufferUsageFlagBits::AccelerationStructureStorageKHR;

        // Buffers.
        let mut buffers = Vec::with_capacity(buffer_creators.len());
        let mut buffer_create_infos = Vec::with_capacity(buffer_creators.len());
        for &buffer_creator in buffer_creators {
            let (buffer, buffer_create_info) = buffer_creator.create(device)?;
            buffers.push(buffer);
            buffer_create_infos.push(buffer_create_info);
        }

        // Buffer allocations.
        let buffer_allocations = BufferAllocations::allocate(
            physical_device,
            device,
            &buffers,
            &buffer_create_infos,
            property_flags,
        )?;

        // Descriptors.
        let mut descriptors = Vec::with_capacity(buffer_creators.len());
        for (buffer_allocation, buffer_create_info) in buffer_allocations
            .allocations()
            .iter()
            .zip(&buffer_create_infos)
        {
            let usage = buffer_create_info.usage;
            let descriptor = if usage.contains(UNIFORM_BUFFER) {
                // assert buffer_allocation == buffer_create_info.size
                Descriptor::create(
                    physical_device,
                    device,
                    DescriptorCreateInfo::UniformBuffer {
                        address: buffer_allocation.device_address(),
                        range: buffer_create_info.size,
                    },
                )
            } else if usage.contains(STORAGE_BUFFER) {
                Descriptor::create(
                    physical_device,
                    device,
                    DescriptorCreateInfo::StorageBuffer {
                        address: buffer_allocation.device_address(),
                        range: buffer_create_info.size,
                    },
                )
            } else if usage.contains(AS_BUFFER) {
                Descriptor::create(
                    physical_device,
                    device,
                    DescriptorCreateInfo::AccelerationStructure(buffer_allocation.device_address()),
                )
            } else {
                bail!(
                    "Buffer resource must be \
                    {UNIFORM_BUFFER} or \
                    {STORAGE_BUFFER} or \
                    {AS_BUFFER}, \
                    got {usage}"
                );
            };
            descriptors.push(descriptor);
        }

        // Resources.
        let mut buffer_resources = Vec::with_capacity(buffer_creators.len());
        for i in 0..buffer_creators.len() {
            let buffer = buffers[i];
            let buffer_create_info = buffer_create_infos[i];
            let buffer_allocation = buffer_allocations.allocations()[i];
            let descriptor = descriptors[i];
            buffer_resources.push(Self {
                buffer,
                buffer_create_info,
                buffer_allocation,
                descriptor,
            });
        }
        Ok((buffer_resources, buffer_allocations))
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_buffer(self.buffer);
    }
}

impl BufferOps for BufferResource {
    fn buffer_handle(&self) -> vk::Buffer {
        self.buffer
    }

    fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_allocation
    }
}

impl BufferResourceOps for BufferResource {
    fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

/// [`BufferDedicatedResource`] trades off allocation efficiency for ease of use.
#[derive(Debug)]
pub struct BufferDedicatedResource {
    buffer_resource: BufferResource,
    buffer_allocations: BufferAllocations,
}

impl BufferDedicatedResource {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        buffer_creator: BufferCreator,
        property_flags: impl Into<vk::MemoryPropertyFlags> + Copy,
    ) -> Result<Self> {
        let (mut buffer_resources, buffer_allocations) =
            BufferResource::create(physical_device, device, &[buffer_creator], property_flags)?;
        let buffer_resource = buffer_resources.swap_remove(0);
        Ok(Self {
            buffer_resource,
            buffer_allocations,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        self.buffer_resource.destroy(device);
        self.buffer_allocations.free(device);
    }
}

impl BufferOps for BufferDedicatedResource {
    fn buffer_handle(&self) -> vk::Buffer {
        self.buffer_resource.buffer
    }

    fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_resource.buffer_create_info
    }

    fn memory(&self) -> &BufferAllocation {
        &self.buffer_resource.buffer_allocation
    }

    fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_resource.buffer_allocation
    }
}

impl BufferResourceOps for BufferDedicatedResource {
    fn descriptor(&self) -> Descriptor {
        self.buffer_resource.descriptor
    }
}

/// [`BufferDedicatedTransfer`] is intended to be used as `TransferSrc` or
/// `TransferDst` and it must also be `HostVisible`.
#[derive(Debug)]
pub struct BufferDedicatedTransfer {
    buffer: vk::Buffer,
    buffer_create_info: vk::BufferCreateInfo,
    buffer_allocations: BufferAllocations,
    buffer_allocation: BufferAllocation,
}

impl BufferDedicatedTransfer {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        buffer_creator: BufferCreator,
        property_flags: impl Into<vk::MemoryPropertyFlags> + Copy,
    ) -> Result<Self> {
        // Validation.
        const TRANSFER_SRC: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::TransferSrc;
        const TRANSFER_DST: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::TransferDst;
        const AS_BUILD_ONLY: vk::BufferUsageFlagBits =
            vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR;

        ensure!(buffer_creator.0.size > 0);
        ensure!(
            buffer_creator.0.usage.contains(TRANSFER_SRC)
                || buffer_creator.0.usage.contains(TRANSFER_DST)
                || buffer_creator.0.usage.contains(AS_BUILD_ONLY),
            "got {}",
            buffer_creator.0.usage
        );
        ensure!(property_flags
            .into()
            .contains(vk::MemoryPropertyFlagBits::HostVisible));

        // Buffer.
        let (buffer, buffer_create_info) = buffer_creator.create(device)?;

        // Allocation.
        let buffer_allocations = BufferAllocations::allocate(
            physical_device,
            device,
            &[buffer],
            &[buffer_create_info],
            property_flags,
        )?;
        let buffer_allocation = buffer_allocations.allocations()[0];

        Ok(Self {
            buffer,
            buffer_create_info,
            buffer_allocations,
            buffer_allocation,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_buffer(self.buffer);
        self.buffer_allocations.free(device);
    }
}

impl BufferOps for BufferDedicatedTransfer {
    fn buffer_handle(&self) -> vk::Buffer {
        self.buffer
    }

    fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_allocation
    }
}

/// [`BufferShaderBindingTable`] is used in ray tracing extensions.
#[derive(Debug)]
pub struct BufferShaderBindingTable {
    buffer: vk::Buffer,
    buffer_create_info: vk::BufferCreateInfo,
    buffer_allocation: BufferAllocation,
}

impl BufferShaderBindingTable {
    pub unsafe fn create(
        buffers: &[vk::Buffer],
        buffer_create_infos: &[vk::BufferCreateInfo],
        buffer_allocations: &[BufferAllocation],
    ) -> Result<Vec<Self>> {
        // Validation.
        ensure!(!buffers.is_empty());
        ensure!(buffers.len() == buffer_create_infos.len());
        ensure!(buffers.len() == buffer_allocations.len());

        // Buffer shader binding tables.
        let mut buffer_sbts = vec![];
        for i in 0..buffers.len() {
            let buffer = buffers[i];
            let buffer_create_info = buffer_create_infos[i];
            let buffer_allocation = buffer_allocations[i];
            buffer_sbts.push(Self {
                buffer,
                buffer_create_info,
                buffer_allocation,
            });
        }
        Ok(buffer_sbts)
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_buffer(self.buffer);
    }
}

impl BufferOps for BufferShaderBindingTable {
    fn buffer_handle(&self) -> vk::Buffer {
        self.buffer
    }

    fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_allocation
    }
}
