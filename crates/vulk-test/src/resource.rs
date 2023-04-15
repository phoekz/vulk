use super::*;

#[derive(Debug)]
pub struct Buffer<T> {
    pub handle: vk::Buffer,
    pub memory_requirements: vk::MemoryRequirements,
    pub device_memory: vk::DeviceMemory,
    pub device_address: vk::DeviceAddress,
    pub element_count: usize,
    pub ptr: *mut T,
}

impl<T> Buffer<T> {
    pub(crate) unsafe fn create(
        device_fn: &DeviceFunctions,
        device: vk::Device,
        physical_device: &PhysicalDevice,
        element_count: usize,
        usage: vk::BufferUsageFlags,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Size.
        let size = (element_count * size_of::<T>()) as vk::DeviceSize;

        // Force SHADER_DEVICE_ADDRESS flag.
        let usage = usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;

        // Buffer.
        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
        };
        let buffer = device_fn
            .create_buffer(device, &buffer_create_info, null())
            .context("Creating buffer object")?;

        // Requirements.
        let device_buffer_memory_requirements = vk::DeviceBufferMemoryRequirements {
            s_type: vk::StructureType::DeviceBufferMemoryRequirements,
            p_next: null(),
            p_create_info: addr_of!(buffer_create_info).cast(),
        };
        let mut memory_requirements2 = vk::MemoryRequirements2 {
            s_type: vk::StructureType::MemoryRequirements2,
            p_next: null_mut(),
            memory_requirements: zeroed(),
        };
        device_fn.get_device_buffer_memory_requirements(
            device,
            &device_buffer_memory_requirements,
            &mut memory_requirements2,
        );
        let memory_requirements = memory_requirements2.memory_requirements;

        // Allocation.
        let memory_allocate_flags_info = vk::MemoryAllocateFlagsInfo {
            s_type: vk::StructureType::MemoryAllocateFlagsInfo,
            p_next: null(),
            flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            device_mask: 0,
        };
        let allocation_size = memory_requirements.size;
        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: addr_of!(memory_allocate_flags_info).cast(),
            allocation_size,
            memory_type_index: memory_type_index(
                &physical_device.memory_properties,
                flags,
                &memory_requirements,
            ),
        };
        let device_memory = device_fn
            .allocate_memory(device, &memory_allocate_info, null())
            .context("Allocating device memory for buffer")?;

        // Bind.
        let bind_buffer_memory_info = vk::BindBufferMemoryInfo {
            s_type: vk::StructureType::BindBufferMemoryInfo,
            p_next: null(),
            buffer,
            memory: device_memory,
            memory_offset: 0,
        };
        device_fn
            .bind_buffer_memory2(device, 1, &bind_buffer_memory_info)
            .context("Binding device memory to buffer")?;

        // Device address.
        let buffer_device_address_info = vk::BufferDeviceAddressInfo {
            s_type: vk::StructureType::BufferDeviceAddressInfo,
            p_next: null(),
            buffer,
        };
        let device_address =
            device_fn.get_buffer_device_address(device, &buffer_device_address_info);

        // Memory map.
        let memory_map_info_khr = vk::MemoryMapInfoKHR {
            s_type: vk::StructureType::MemoryMapInfoKHR,
            p_next: null(),
            flags: vk::MemoryMapFlags::empty(),
            memory: device_memory,
            offset: 0,
            size: size as _,
        };
        let mut ptr = MaybeUninit::uninit();
        device_fn
            .map_memory2_khr(device, &memory_map_info_khr, ptr.as_mut_ptr())
            .context("Mapping device memory")?;
        let ptr = ptr.assume_init().cast::<T>();

        Ok(Buffer {
            handle: buffer,
            memory_requirements,
            device_memory,
            device_address,
            element_count,
            ptr,
        })
    }

    pub(crate) unsafe fn destroy(&self, device_fn: &DeviceFunctions, device: vk::Device) {
        device_fn.destroy_buffer(device, self.handle, null());
        device_fn.free_memory(device, self.device_memory, null());
    }

    pub(crate) fn byte_size(&self) -> usize {
        self.element_count * size_of::<T>()
    }
}
