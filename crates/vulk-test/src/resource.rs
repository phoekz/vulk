use super::*;

//
// Buffer
//

#[derive(Debug)]
pub struct Buffer<T> {
    pub buffer_create_info: vk::BufferCreateInfo,
    pub buffer: vk::Buffer,

    pub memory_requirements: vk::MemoryRequirements,
    pub memory_allocate_info: vk::MemoryAllocateInfo,
    pub device_memory: vk::DeviceMemory,
    pub bind_buffer_memory_info: vk::BindBufferMemoryInfo,

    pub device_address: vk::DeviceAddress,
    pub element_count: usize,
    pub ptr: *mut T,
}

impl<T> Buffer<T> {
    pub unsafe fn create(
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
            buffer_create_info,
            buffer,

            memory_requirements,
            memory_allocate_info,
            device_memory,
            bind_buffer_memory_info,

            device_address,
            element_count,
            ptr,
        })
    }

    pub unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_buffer(self.buffer);
        device.free_memory(self.device_memory);
    }

    pub fn byte_size(&self) -> usize {
        self.element_count * size_of::<T>()
    }
}

//
// Image
//

#[derive(Debug)]
pub struct Image2d {
    pub image_create_info: vk::ImageCreateInfo,
    pub image: vk::Image,

    pub memory_requirements: vk::MemoryRequirements,
    pub memory_allocate_info: vk::MemoryAllocateInfo,
    pub device_memory: vk::DeviceMemory,
    pub bind_image_memory_info: vk::BindImageMemoryInfo,

    pub image_view: vk::ImageView,
    pub image_view_create_info: vk::ImageViewCreateInfo,
}

impl Image2d {
    pub unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        format: vk::Format,
        width: u32,
        height: u32,
        samples: vk::SampleCountFlagBits,
        usage: vk::ImageUsageFlags,
        property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        // Image info.
        let image_create_info = vk::ImageCreateInfo {
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
            samples,
            tiling: vk::ImageTiling::Optimal,
            usage,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
            initial_layout: vk::ImageLayout::Undefined,
        };

        // Image.
        let image = device
            .create_image(&image_create_info)
            .context("Creating image")?;

        // Requirements.
        let device_image_memory_requirements = vk::DeviceImageMemoryRequirements {
            s_type: vk::StructureType::DeviceImageMemoryRequirements,
            p_next: null(),
            p_create_info: &image_create_info,
            plane_aspect: vk::ImageAspectFlagBits::empty(),
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
        let memory_requirements = memory_requirements2.memory_requirements;

        // Allocation.
        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: null(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index(
                &physical_device.memory_properties,
                property_flags,
                &memory_requirements,
            ),
        };
        let device_memory = device
            .allocate_memory(&memory_allocate_info)
            .context("Allocating device memory for image")?;

        // Bind.
        let bind_image_memory_info = vk::BindImageMemoryInfo {
            s_type: vk::StructureType::BindImageMemoryInfo,
            p_next: null(),
            image,
            memory: device_memory,
            memory_offset: 0,
        };
        device
            .bind_image_memory2(1, &bind_image_memory_info)
            .context("Binding device memory to image")?;

        // Image view info.
        let image_view_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type: vk::ImageViewType::Type2d,
            format: image_create_info.format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        // Image view.
        let image_view = device.create_image_view(&image_view_create_info)?;

        Ok(Self {
            image_create_info,
            image,

            memory_requirements,
            memory_allocate_info,
            device_memory,
            bind_image_memory_info,

            image_view,
            image_view_create_info,
        })
    }

    pub unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_image_view(self.image_view);
        device.destroy_image(self.image);
        device.free_memory(self.device_memory);
    }

    pub unsafe fn width(&self) -> u32 {
        self.image_create_info.extent.width
    }

    pub unsafe fn height(&self) -> u32 {
        self.image_create_info.extent.height
    }

    pub unsafe fn extent_2d(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width(),
            height: self.height(),
        }
    }

    pub unsafe fn extent_3d(&self) -> vk::Extent3D {
        vk::Extent3D {
            width: self.width(),
            height: self.height(),
            depth: 1,
        }
    }

    pub unsafe fn rect_2d(&self) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.extent_2d(),
        }
    }

    pub unsafe fn subresource_range(&self) -> vk::ImageSubresourceRange {
        self.image_view_create_info.subresource_range
    }

    pub unsafe fn subresource_layers(&self) -> vk::ImageSubresourceLayers {
        vk::ImageSubresourceLayers {
            aspect_mask: self.image_view_create_info.subresource_range.aspect_mask,
            mip_level: 0,
            base_array_layer: self
                .image_view_create_info
                .subresource_range
                .base_array_layer,
            layer_count: self.image_view_create_info.subresource_range.layer_count,
        }
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
