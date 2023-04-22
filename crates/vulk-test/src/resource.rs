use super::*;

//
// Buffer
//

#[derive(Debug)]
pub struct BufferCreateInfo {
    pub element_count: usize,
    pub usage: vk::BufferUsageFlags,
    pub property_flags: vk::MemoryPropertyFlags,
}

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

    pub descriptor: descriptor::Descriptor,
}

impl<T> GpuResource for Buffer<T> {
    type CreateInfo<'a> = BufferCreateInfo;

    unsafe fn create(
        gpu @ Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Size.
        let size = (create_info.element_count * size_of::<T>()) as vk::DeviceSize;

        // Force SHADER_DEVICE_ADDRESS flag.
        let usage = create_info.usage | vk::BufferUsageFlagBits::ShaderDeviceAddress;

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
            flags: vk::MemoryAllocateFlagBits::DeviceAddress.into(),
            device_mask: 0,
        };
        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: addr_of!(memory_allocate_flags_info).cast(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index(
                &physical_device.memory_properties,
                create_info.property_flags,
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
        let ptr = if create_info
            .property_flags
            .contains(vk::MemoryPropertyFlagBits::HostVisible.into())
        {
            device
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
                .cast()
        } else {
            null_mut()
        };

        // Descriptor.
        let descriptor = descriptor::Descriptor::create_storage_buffer(gpu, device_address, size);

        Ok(Buffer {
            buffer_create_info,
            buffer,

            memory_requirements,
            memory_allocate_info,
            device_memory,
            bind_buffer_memory_info,

            device_address,
            element_count: create_info.element_count,
            ptr,

            descriptor,
        })
    }

    unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_buffer(self.buffer);
        device.free_memory(self.device_memory);
    }
}

//
// Texture
//

#[derive(Debug)]
pub struct Image2dCreateInfo {
    pub format: vk::Format,
    pub width: u32,
    pub height: u32,
    pub samples: vk::SampleCountFlagBits,
    pub usage: vk::ImageUsageFlags,
    pub property_flags: vk::MemoryPropertyFlags,
}

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

    pub descriptor: descriptor::Descriptor,
}

impl GpuResource for Image2d {
    type CreateInfo<'a> = Image2dCreateInfo;

    unsafe fn create(
        gpu @ Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Image info.
        let image_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::ImageCreateInfo,
            p_next: null(),
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::Type2d,
            format: create_info.format,
            extent: vk::Extent3D {
                width: create_info.width,
                height: create_info.height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: create_info.samples,
            tiling: vk::ImageTiling::Optimal,
            usage: create_info.usage,
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
        let memory_requirements = memory_requirements2.memory_requirements;

        // Allocation.
        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: null(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index(
                &physical_device.memory_properties,
                create_info.property_flags,
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
                aspect_mask: create_info
                    .format
                    .aspect_mask()
                    .with_context(|| format!("Invalid image format: {}", create_info.format))?,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        // Image view.
        let image_view = device.create_image_view(&image_view_create_info)?;

        // Descriptor.
        let descriptor = descriptor::Descriptor::create_sampled_image(
            gpu,
            image_view,
            vk::ImageLayout::ShaderReadOnlyOptimal,
        );

        Ok(Self {
            image_create_info,
            image,

            memory_requirements,
            memory_allocate_info,
            device_memory,
            bind_image_memory_info,

            image_view,
            image_view_create_info,

            descriptor,
        })
    }

    unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_image_view(self.image_view);
        device.destroy_image(self.image);
        device.free_memory(self.device_memory);
    }
}

impl Image2d {
    pub unsafe fn format(&self) -> vk::Format {
        self.image_create_info.format
    }

    pub unsafe fn width(&self) -> u32 {
        self.image_create_info.extent.width
    }

    pub unsafe fn height(&self) -> u32 {
        self.image_create_info.extent.height
    }

    pub unsafe fn byte_size(&self) -> u32 {
        self.format().block_size() * self.width() * self.height()
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
// Sampler
//

#[derive(Debug)]
pub struct SamplerCreateInfo {
    pub mag_filter: vk::Filter,
    pub min_filter: vk::Filter,
    pub mipmap_mode: vk::SamplerMipmapMode,
    pub address_mode: vk::SamplerAddressMode,
}

#[derive(Debug)]
pub struct Sampler {
    pub sampler: vk::Sampler,
    pub sampler_create_info: vk::SamplerCreateInfo,
    pub descriptor: descriptor::Descriptor,
}

impl GpuResource for Sampler {
    type CreateInfo<'a> = SamplerCreateInfo;

    unsafe fn create(
        gpu @ Gpu { device, .. }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Sampler info.
        let sampler_create_info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SamplerCreateInfo,
            p_next: null(),
            flags: vk::SamplerCreateFlags::empty(),
            mag_filter: create_info.mag_filter,
            min_filter: create_info.min_filter,
            mipmap_mode: create_info.mipmap_mode,
            address_mode_u: create_info.address_mode,
            address_mode_v: create_info.address_mode,
            address_mode_w: create_info.address_mode,
            mip_lod_bias: 0.0,
            anisotropy_enable: vk::FALSE,
            max_anisotropy: 0.0,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::Always,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: vk::BorderColor::FloatTransparentBlack,
            unnormalized_coordinates: vk::FALSE,
        };

        // Sampler.
        let sampler = device.create_sampler(&sampler_create_info)?;

        // Descriptor.
        let descriptor = descriptor::Descriptor::create_sampler(gpu, sampler);

        Ok(Self {
            sampler,
            sampler_create_info,
            descriptor,
        })
    }

    unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_sampler(self.sampler);
    }
}

//
// Upload
//

pub unsafe fn multi_upload_images(
    gpu @ Gpu { device, queue, .. }: &Gpu,
    images: &[Image2d],
    datas: &[Vec<u8>],
) -> Result<()> {
    // Validation.
    ensure!(!images.is_empty());
    ensure!(!datas.is_empty());
    ensure!(images.len() == datas.len());
    ensure!(images.iter().all(|img| img.byte_size() > 0));
    ensure!(datas.iter().all(|p| !p.is_empty()));
    ensure!(images
        .iter()
        .zip(datas)
        .all(|(img, p)| img.byte_size() as usize == p.len()));

    // Staging buffer.
    let byte_size = datas.iter().map(Vec::len).sum::<usize>();
    let staging = resource::Buffer::<u8>::create(
        gpu,
        &BufferCreateInfo {
            element_count: byte_size,
            usage: vk::BufferUsageFlagBits::TransferSrc.into(),
            property_flags: vk::MemoryPropertyFlagBits::HostVisible
                | vk::MemoryPropertyFlagBits::HostCoherent,
        },
    )?;
    let mut dst_offset = 0;
    for image_data in datas {
        // Copy.
        std::ptr::copy_nonoverlapping(
            image_data.as_ptr(),
            staging.ptr.add(dst_offset),
            image_data.len(),
        );

        // Advance.
        dst_offset += image_data.len();
    }

    // Begin staging.
    let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
    let cmd = commands.begin(gpu)?;

    // Transfer commands.
    let mut src_offset = 0;
    for (image, image_data) in images.iter().zip(datas) {
        // Transition Undefined -> TransferDstOptimal.
        device.cmd_pipeline_barrier2(
            cmd,
            &vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 0,
                p_memory_barriers: null(),
                buffer_memory_barrier_count: 0,
                p_buffer_memory_barriers: null(),
                image_memory_barrier_count: 1,
                p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                    s_type: vk::StructureType::ImageMemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: vk::PipelineStageFlagBits2::Host.into(),
                    src_access_mask: vk::AccessFlags2::empty(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    dst_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    old_layout: vk::ImageLayout::Undefined,
                    new_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: 0,
                    dst_queue_family_index: 0,
                    image: image.image,
                    subresource_range: image.subresource_range(),
                },
            },
        );
        device.cmd_copy_buffer_to_image2(
            cmd,
            &(vk::CopyBufferToImageInfo2 {
                s_type: vk::StructureType::CopyBufferToImageInfo2,
                p_next: null(),
                src_buffer: staging.buffer,
                dst_image: image.image,
                dst_image_layout: vk::ImageLayout::TransferDstOptimal,
                region_count: 1,
                p_regions: &(vk::BufferImageCopy2 {
                    s_type: vk::StructureType::BufferImageCopy2,
                    p_next: null(),
                    buffer_offset: src_offset as _,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: image.subresource_layers(),
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: image.extent_3d(),
                }),
            }),
        );
        // Transition TransferDstOptimal -> ShaderReadOnly.
        device.cmd_pipeline_barrier2(
            cmd,
            &vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 0,
                p_memory_barriers: null(),
                buffer_memory_barrier_count: 0,
                p_buffer_memory_barriers: null(),
                image_memory_barrier_count: 1,
                p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                    s_type: vk::StructureType::ImageMemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: vk::PipelineStageFlagBits2::AllTransfer.into(),
                    src_access_mask: vk::AccessFlagBits2::TransferWrite.into(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::FragmentShader.into(),
                    dst_access_mask: vk::AccessFlagBits2::ShaderRead.into(),
                    new_layout: vk::ImageLayout::ShaderReadOnlyOptimal,
                    old_layout: vk::ImageLayout::TransferDstOptimal,
                    src_queue_family_index: 0,
                    dst_queue_family_index: 0,
                    image: image.image,
                    subresource_range: image.subresource_range(),
                },
            },
        );

        // Advance.
        src_offset += image_data.len();
    }

    // End staging.
    commands.end(gpu)?;
    device.queue_submit2(
        *queue,
        1,
        &(vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: 0,
            p_wait_semaphore_infos: null(),
            command_buffer_info_count: 1,
            p_command_buffer_infos: &(vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer: cmd,
                device_mask: 0,
            }),
            signal_semaphore_info_count: 1,
            p_signal_semaphore_infos: &(vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: commands.semaphore,
                value: 1,
                stage_mask: vk::PipelineStageFlagBits2::BottomOfPipe.into(),
                device_index: 0,
            }),
        }),
        vk::Fence::null(),
    )?;
    device.wait_semaphores(
        &(vk::SemaphoreWaitInfo {
            s_type: vk::StructureType::SemaphoreWaitInfo,
            p_next: null(),
            flags: vk::SemaphoreWaitFlagBits::Any.into(),
            semaphore_count: 1,
            p_semaphores: [commands.semaphore].as_ptr(),
            p_values: [1].as_ptr(),
        }),
        u64::MAX,
    )?;

    // Cleanup.
    commands.destroy(gpu);
    staging.destroy(gpu);

    Ok(())
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
        let property_matches = memory_type.property_flags.contains(property_flags);
        if type_matches && property_matches {
            return memory_type_index;
        }
    }
    panic!("Unable to find suitable memory type for the buffer, memory_type_bits=0b{memory_type_bits:b}");
}
