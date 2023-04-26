use super::*;

/// **Example**:
///
/// ```no_run
/// # use vulk::vk as vk;
/// # use vulk_ext::vkx as vkx;
/// # unsafe {
/// # let device = todo!();
/// # let size = todo!();
/// # let usage = todo!();
/// let (buffer, buffer_create_info) =
///     vkx::BufferCreator::new(size, usage)
///         .create(device)
///         .unwrap();
/// # }
/// ```
pub struct BufferCreator(vk::BufferCreateInfo);

impl BufferCreator {
    #[must_use]
    pub fn new(size: vk::DeviceSize, usage: vk::BufferUsageFlags) -> Self {
        Self(vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage: usage | vk::BufferUsageFlagBits::ShaderDeviceAddress,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
        })
    }

    pub unsafe fn create(self, device: &Device) -> Result<(vk::Buffer, vk::BufferCreateInfo)> {
        let buffer_create_info = self.0;
        let buffer = device.create_buffer(&buffer_create_info)?;
        Ok((buffer, buffer_create_info))
    }
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
        buffers: &[vk::Buffer],
        buffer_create_infos: &[vk::BufferCreateInfo],
        buffer_allocations: &[BufferAllocation],
    ) -> Result<Vec<Self>> {
        // Constants.
        const UNIFORM_BUFFER: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::UniformBuffer;
        const STORAGE_BUFFER: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::StorageBuffer;
        const AS_BUFFER: vk::BufferUsageFlagBits =
            vk::BufferUsageFlagBits::AccelerationStructureStorageKHR;

        // Validation.
        ensure!(!buffers.is_empty());
        ensure!(buffers.len() == buffer_create_infos.len());
        ensure!(buffers.len() == buffer_allocations.len());

        // Buffer resources.
        let mut buffer_resources = vec![];
        for i in 0..buffers.len() {
            let buffer = buffers[i];
            let buffer_create_info = buffer_create_infos[i];
            let buffer_allocation = buffer_allocations[i];
            let buffer_usage = buffer_create_info.usage;
            let descriptor_create_info = if buffer_usage.contains(UNIFORM_BUFFER.into()) {
                DescriptorCreateInfo::UniformBuffer {
                    address: buffer_allocation.device_address(),
                    range: buffer_create_info.size,
                }
            } else if buffer_usage.contains(STORAGE_BUFFER.into()) {
                DescriptorCreateInfo::StorageBuffer {
                    address: buffer_allocation.device_address(),
                    range: buffer_create_info.size,
                }
            } else if buffer_usage.contains(AS_BUFFER.into()) {
                DescriptorCreateInfo::AccelerationStructure(buffer_allocation.device_address())
            } else {
                bail!("Buffer resource must be {UNIFORM_BUFFER} or {STORAGE_BUFFER}, got {buffer_usage}");
            };
            let descriptor = Descriptor::create(physical_device, device, descriptor_create_info);
            buffer_resources.push(Self {
                buffer,
                buffer_create_info,
                buffer_allocation,
                descriptor,
            });
        }
        Ok(buffer_resources)
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_buffer(self.buffer);
    }

    #[must_use]
    pub fn handle(&self) -> vk::Buffer {
        self.buffer
    }

    #[must_use]
    pub fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    #[must_use]
    pub fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    #[must_use]
    pub fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_allocation
    }

    #[must_use]
    pub fn descriptor(&self) -> Descriptor {
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
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Buffer.
        let (buffer, buffer_create_info) = BufferCreator::new(size, usage).create(device)?;

        // Allocation.
        let buffer_allocations = BufferAllocations::allocate(
            physical_device,
            device,
            &[buffer],
            &[buffer_create_info],
            property_flags,
        )?;

        // Resource.
        let mut buffer_resources = BufferResource::create(
            physical_device,
            device,
            &[buffer],
            &[buffer_create_info],
            buffer_allocations.allocations(),
        )?;
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

    #[must_use]
    pub fn handle(&self) -> vk::Buffer {
        self.buffer_resource.buffer
    }

    #[must_use]
    pub fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_resource.buffer_create_info
    }

    #[must_use]
    pub fn memory(&self) -> &BufferAllocation {
        &self.buffer_resource.buffer_allocation
    }

    #[must_use]
    pub fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_resource.buffer_allocation
    }

    #[must_use]
    pub fn descriptor(&self) -> Descriptor {
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
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Validation.
        const TRANSFER_SRC: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::TransferSrc;
        const TRANSFER_DST: vk::BufferUsageFlagBits = vk::BufferUsageFlagBits::TransferDst;
        const AS_BUILD_ONLY: vk::BufferUsageFlagBits =
            vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR;
        ensure!(size > 0);
        ensure!(
            usage.contains(TRANSFER_SRC.into())
                || usage.contains(TRANSFER_DST.into())
                || usage.contains(AS_BUILD_ONLY.into()),
            "got {usage}"
        );
        ensure!(property_flags.contains(vk::MemoryPropertyFlagBits::HostVisible.into()));

        // Buffer.
        let (buffer, buffer_create_info) = BufferCreator::new(size, usage).create(device)?;

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

    #[must_use]
    pub fn handle(&self) -> vk::Buffer {
        self.buffer
    }

    #[must_use]
    pub fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    #[must_use]
    pub fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    #[must_use]
    pub fn memory_mut(&mut self) -> &mut BufferAllocation {
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

    #[must_use]
    pub fn handle(&self) -> vk::Buffer {
        self.buffer
    }

    #[must_use]
    pub fn create_info(&self) -> &vk::BufferCreateInfo {
        &self.buffer_create_info
    }

    #[must_use]
    pub fn memory(&self) -> &BufferAllocation {
        &self.buffer_allocation
    }

    #[must_use]
    pub fn memory_mut(&mut self) -> &mut BufferAllocation {
        &mut self.buffer_allocation
    }
}
