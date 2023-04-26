use super::*;

//
// Buffers
//

#[derive(Clone, Copy, Debug)]
pub struct BufferAllocation {
    address: vk::DeviceAddress,
    size: vk::DeviceSize,
    ptr: Option<*mut c_void>,
}

impl BufferAllocation {
    #[must_use]
    pub fn device_address(&self) -> vk::DeviceAddress {
        self.address
    }

    #[must_use]
    pub fn size(&self) -> vk::DeviceSize {
        self.size
    }

    #[must_use]
    pub fn as_ptr<T>(&self) -> *const T {
        if let Some(ptr) = self.ptr {
            ptr.cast()
        } else {
            null()
        }
    }

    #[must_use]
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        if let Some(ptr) = self.ptr {
            ptr.cast()
        } else {
            null_mut()
        }
    }

    #[must_use]
    pub unsafe fn as_slice<T>(&self, len: usize) -> &[T] {
        if let Some(ptr) = self.ptr {
            std::slice::from_raw_parts(ptr.cast(), len)
        } else {
            &[]
        }
    }

    #[must_use]
    pub unsafe fn as_mut_slice<T>(&mut self, len: usize) -> &mut [T] {
        if let Some(ptr) = self.ptr {
            std::slice::from_raw_parts_mut(ptr.cast(), len)
        } else {
            &mut []
        }
    }
}

#[derive(Debug)]
pub struct BufferAllocations {
    memory: vk::DeviceMemory,
    allocations: Vec<BufferAllocation>,
}

impl BufferAllocations {
    pub unsafe fn allocate(
        physical_device: &vkx::PhysicalDevice,
        device: &vkx::Device,
        buffers: &[vk::Buffer],
        create_infos: &[vk::BufferCreateInfo],
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Validation.
        ensure!(!buffers.is_empty());
        ensure!(!create_infos.is_empty());
        ensure!(buffers.len() == create_infos.len());
        ensure!(create_infos
            .iter()
            .all(|info| info.s_type == vk::StructureType::BufferCreateInfo));
        ensure!(create_infos.iter().all(|info| info.p_next.is_null()));
        ensure!(create_infos
            .iter()
            .all(|info| info.flags == vk::BufferCreateFlags::empty()));
        ensure!(create_infos.iter().all(|info| info.size > 0));
        ensure!(create_infos.iter().all(|info| info
            .usage
            .contains(vk::BufferUsageFlagBits::ShaderDeviceAddress.into())));
        ensure!(create_infos
            .iter()
            .all(|info| info.sharing_mode == vk::SharingMode::Exclusive));
        ensure!(create_infos.iter().all(
            |info| info.queue_family_index_count == 0 && info.p_queue_family_indices.is_null()
        ));
        ensure!(property_flags != vk::MemoryPropertyFlags::empty());

        // Requirements.
        let mut memory_requirements = vec![];
        for &create_info in create_infos {
            let device_buffer_memory_requirements = vk::DeviceBufferMemoryRequirements {
                s_type: vk::StructureType::DeviceBufferMemoryRequirements,
                p_next: null(),
                p_create_info: addr_of!(create_info).cast(),
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
            memory_requirements.push(memory_requirements2.memory_requirements);
        }

        // Buffers must be compatible with the allocation we make.
        ensure!(memory_requirements
            .iter()
            .all(|req| req.alignment == memory_requirements[0].alignment));
        ensure!(memory_requirements
            .iter()
            .all(|req| req.memory_type_bits == memory_requirements[0].memory_type_bits));
        let alignment = memory_requirements[0].alignment;
        let memory_type_bits = memory_requirements[0].memory_type_bits;

        // Memory type index.
        let memory_type_index = memory_type_index(
            &physical_device.memory_properties,
            property_flags,
            memory_type_bits,
        );

        // Allocation size.
        let allocation_size = memory_requirements
            .iter()
            .map(|req| aligned_size(req.size, alignment))
            .sum::<vk::DeviceSize>();

        // Allocation.
        let device_memory = {
            let memory_allocate_flags_info = vk::MemoryAllocateFlagsInfo {
                s_type: vk::StructureType::MemoryAllocateFlagsInfo,
                p_next: null(),
                flags: vk::MemoryAllocateFlagBits::DeviceAddress.into(),
                device_mask: 0,
            };
            device
                .allocate_memory(&vk::MemoryAllocateInfo {
                    s_type: vk::StructureType::MemoryAllocateInfo,
                    p_next: addr_of!(memory_allocate_flags_info).cast(),
                    allocation_size,
                    memory_type_index,
                })
                .with_context(|| {
                    format!("Allocating device memory for {} buffers", buffers.len())
                })?
        };

        // Sub-allocations.
        let mut allocations = vec![];
        let mut memory_offset = 0;
        for (buffer_index, (buffer, requirements)) in
            buffers.iter().zip(memory_requirements).enumerate()
        {
            // Aligned size.
            let aligned_size = aligned_size(requirements.size, alignment);

            // Bind buffer memory.
            device
                .bind_buffer_memory2(
                    1,
                    &vk::BindBufferMemoryInfo {
                        s_type: vk::StructureType::BindBufferMemoryInfo,
                        p_next: null(),
                        buffer: *buffer,
                        memory: device_memory,
                        memory_offset,
                    },
                )
                .with_context(|| {
                    format!(
                        "Binding buffer {buffer_index} \
                        size={aligned_size} into \
                        device memory offset={memory_offset}"
                    )
                })?;

            // Device address.
            let device_address = device.get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                s_type: vk::StructureType::BufferDeviceAddressInfo,
                p_next: null(),
                buffer: *buffer,
            });

            // Map memory.
            let mut buffer_ptr = None;
            if property_flags.contains(vk::MemoryPropertyFlagBits::HostVisible.into()) {
                let ptr = device
                    .map_memory2_khr(&vk::MemoryMapInfoKHR {
                        s_type: vk::StructureType::MemoryMapInfoKHR,
                        p_next: null(),
                        flags: vk::MemoryMapFlags::empty(),
                        memory: device_memory,
                        offset: memory_offset,
                        size: aligned_size,
                    })
                    .with_context(|| {
                        format!("Mapping buffer {buffer_index} size={aligned_size}")
                    })?;
                buffer_ptr = Some(ptr);
            }

            // Advance.
            memory_offset += aligned_size;

            // Output.
            allocations.push(BufferAllocation {
                address: device_address,
                size: requirements.size,
                ptr: buffer_ptr,
            });
        }

        Ok(Self {
            memory: device_memory,
            allocations,
        })
    }

    pub unsafe fn free(self, device: &vkx::Device) {
        device.free_memory(self.memory);
    }

    #[must_use]
    pub fn allocations(&self) -> &[BufferAllocation] {
        &self.allocations
    }
}

//
// Images
//

#[derive(Debug)]
pub struct ImageAllocations {
    memory: vk::DeviceMemory,
}

impl ImageAllocations {
    pub unsafe fn allocate(
        physical_device: &vkx::PhysicalDevice,
        device: &vkx::Device,
        images: &[vk::Image],
        create_infos: &[vk::ImageCreateInfo],
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Validation.
        ensure!(!images.is_empty());
        ensure!(!create_infos.is_empty());
        ensure!(images.len() == create_infos.len());
        ensure!(create_infos
            .iter()
            .all(|info| info.s_type == vk::StructureType::ImageCreateInfo));
        ensure!(create_infos.iter().all(|info| info.p_next.is_null()));
        ensure!(create_infos
            .iter()
            .all(|info| info.flags == vk::ImageCreateFlags::empty()));
        ensure!(create_infos
            .iter()
            .all(|info| info.extent.width > 0 && info.extent.height > 0 && info.extent.depth > 0));
        ensure!(create_infos.iter().all(|info| info.mip_levels > 0));
        ensure!(create_infos.iter().all(|info| info.array_layers > 0));
        ensure!(create_infos
            .iter()
            .all(|info| info.tiling == vk::ImageTiling::Optimal));
        ensure!(create_infos
            .iter()
            .all(|info| info.sharing_mode == vk::SharingMode::Exclusive));
        ensure!(create_infos.iter().all(
            |info| info.queue_family_index_count == 0 && info.p_queue_family_indices.is_null()
        ));
        ensure!(create_infos
            .iter()
            .all(|info| info.initial_layout == vk::ImageLayout::Undefined));

        // Requirements.
        let mut memory_requirements = vec![];
        for &create_info in create_infos {
            let device_image_memory_requirements = vk::DeviceImageMemoryRequirements {
                s_type: vk::StructureType::DeviceImageMemoryRequirements,
                p_next: null(),
                p_create_info: &create_info,
                plane_aspect: zeroed(),
            };
            let mut memory_requirements2 = vk::MemoryRequirements2 {
                s_type: vk::StructureType::MemoryRequirements2,
                p_next: null_mut(),
                memory_requirements: zeroed(),
            };
            device.get_device_image_memory_requirements(
                &device_image_memory_requirements,
                &mut memory_requirements2,
            );
            memory_requirements.push(memory_requirements2.memory_requirements);
        }

        // Images must be compatible with the allocation we make.
        ensure!(memory_requirements
            .iter()
            .all(|req| req.alignment == memory_requirements[0].alignment));
        ensure!(memory_requirements
            .iter()
            .all(|req| req.memory_type_bits == memory_requirements[0].memory_type_bits));
        let alignment = memory_requirements[0].alignment;
        let memory_type_bits = memory_requirements[0].memory_type_bits;

        // Memory type index.
        let memory_type_index = memory_type_index(
            &physical_device.memory_properties,
            property_flags,
            memory_type_bits,
        );

        // Allocation size.
        let allocation_size = memory_requirements
            .iter()
            .map(|req| aligned_size(req.size, alignment))
            .sum::<vk::DeviceSize>();

        // Allocation.
        let device_memory = device
            .allocate_memory(&vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MemoryAllocateInfo,
                p_next: null(),
                allocation_size,
                memory_type_index,
            })
            .with_context(|| {
                format!(
                    "\
                    Allocating device memory for {} images",
                    images.len()
                )
            })?;

        // Sub-allocations.
        let mut memory_offset = 0;
        for (image_index, (image, requirements)) in
            images.iter().zip(memory_requirements).enumerate()
        {
            // Aligned size.
            let aligned_size = aligned_size(requirements.size, alignment);

            // Bind image memory.
            device
                .bind_image_memory2(
                    1,
                    &vk::BindImageMemoryInfo {
                        s_type: vk::StructureType::BindImageMemoryInfo,
                        p_next: null(),
                        image: *image,
                        memory: device_memory,
                        memory_offset,
                    },
                )
                .with_context(|| {
                    format!(
                        "\
                        Binding image {image_index} \
                        size={aligned_size} into \
                        device memory offset={memory_offset}"
                    )
                })?;

            // Advance.
            memory_offset += aligned_size;
        }

        Ok(Self {
            memory: device_memory,
        })
    }

    pub unsafe fn free(self, device: &vkx::Device) {
        device.free_memory(self.memory);
    }
}

//
// Utilities
//

fn memory_type_index(
    memory: &vk::PhysicalDeviceMemoryProperties,
    property_flags: vk::MemoryPropertyFlags,
    memory_type_bits: u32,
) -> u32 {
    for memory_type_index in 0..memory.memory_type_count {
        let memory_type = memory.memory_types[memory_type_index as usize];
        let type_matches = (1 << memory_type_index) & memory_type_bits != 0;
        let property_matches = memory_type.property_flags.contains(property_flags);
        if type_matches && property_matches {
            debug!(
                "index={}, type={:?}, heap={:?}",
                memory_type_index,
                &memory.memory_types[memory_type_index as usize].property_flags,
                &memory.memory_heaps[memory_type.heap_index as usize].flags
            );
            return memory_type_index;
        }
    }
    panic!("Unable to find suitable memory type for the buffer, memory_type_bits=0b{memory_type_bits:b}");
}

fn aligned_size(size: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    (size + alignment - 1) & !(alignment - 1)
}
