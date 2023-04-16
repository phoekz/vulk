use super::*;

//
// Buffer
//

#[derive(Debug)]
pub struct Buffer<T> {
    pub handle: vk::Buffer,
    pub buffer_create_info: vk::BufferCreateInfo,
    pub memory_requirements: vk::MemoryRequirements,
    pub memory_allocate_info: vk::MemoryAllocateInfo,
    pub device_memory: vk::DeviceMemory,
    pub bind_buffer_memory_info: vk::BindBufferMemoryInfo,
    pub device_address: vk::DeviceAddress,
    pub element_count: usize,
    pub ptr: *mut T,
}

impl<T> Buffer<T> {
    pub(crate) unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        element_count: usize,
        usage: vk::BufferUsageFlags,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Size.
        let size = (element_count * size_of::<T>()) as vk::DeviceSize;

        // Force SHADER_DEVICE_ADDRESS flag.
        let usage = usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;

        // Buffer info.
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

        // Buffer.
        let buffer = device
            .create_buffer(&buffer_create_info)
            .context("Creating buffer object")?;

        // Requirements.
        let memory_requirements = {
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
            device.get_device_buffer_memory_requirements(
                &device_buffer_memory_requirements,
                &mut memory_requirements2,
            );
            memory_requirements2.memory_requirements
        };

        // Allocation.
        let memory_allocate_flags_info = vk::MemoryAllocateFlagsInfo {
            s_type: vk::StructureType::MemoryAllocateFlagsInfo,
            p_next: null(),
            flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            device_mask: 0,
        };
        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: addr_of!(memory_allocate_flags_info).cast(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index(
                &physical_device.memory_properties,
                flags,
                &memory_requirements,
            ),
        };
        let device_memory = device
            .allocate_memory(&memory_allocate_info)
            .context("Allocating device memory for buffer")?;

        // Bind.
        let bind_buffer_memory_info = vk::BindBufferMemoryInfo {
            s_type: vk::StructureType::BindBufferMemoryInfo,
            p_next: null(),
            buffer,
            memory: device_memory,
            memory_offset: 0,
        };
        device
            .bind_buffer_memory2(1, &bind_buffer_memory_info)
            .context("Binding device memory to buffer")?;

        // Device address.
        let device_address = device.get_buffer_device_address(
            &(vk::BufferDeviceAddressInfo {
                s_type: vk::StructureType::BufferDeviceAddressInfo,
                p_next: null(),
                buffer,
            }),
        );

        // Memory map.
        let ptr = device
            .map_memory2_khr(
                &(vk::MemoryMapInfoKHR {
                    s_type: vk::StructureType::MemoryMapInfoKHR,
                    p_next: null(),
                    flags: vk::MemoryMapFlags::empty(),
                    memory: device_memory,
                    offset: 0,
                    size: size as _,
                }),
            )
            .context("Mapping device memory")?
            .cast();

        Ok(Buffer {
            handle: buffer,
            buffer_create_info,
            memory_requirements,
            memory_allocate_info,
            device_memory,
            bind_buffer_memory_info,
            device_address,
            element_count,
            ptr,
        })
    }

    pub(crate) unsafe fn destroy(&self, device: &vulk::loader::Device) {
        device.destroy_buffer(self.handle);
        device.free_memory(self.device_memory);
    }

    pub(crate) fn byte_size(&self) -> usize {
        self.element_count * size_of::<T>()
    }
}

//
// Utilities
//

fn memory_type_index(
    memory: &vk::PhysicalDeviceMemoryProperties,
    property_flags: vk::MemoryPropertyFlags,
    requirements: &vk::MemoryRequirements,
) -> u32 {
    let memory_type_bits = requirements.memory_type_bits;
    for memory_type_index in 0..memory.memory_type_count {
        let memory_type = memory.memory_types[memory_type_index as usize];
        let type_matches = (1 << memory_type_index) & memory_type_bits != 0;
        let property_matches = memory_type.property_flags & property_flags == property_flags;
        if type_matches && property_matches {
            return memory_type_index;
        }
    }
    panic!("Unable to find suitable memory type for the buffer, memory_type_bits=0b{memory_type_bits:b}");
}
