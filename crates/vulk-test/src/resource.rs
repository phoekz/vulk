use super::*;

//
// Buffer
//

// Todo: Re-design with the new batch allocator.
// Todo: The batch allocator could create the descriptor handle too.

#[derive(Debug)]
pub struct BufferCreateInfo {
    pub size: vk::DeviceSize,
    pub usage: vk::BufferUsageFlags,
    pub property_flags: vk::MemoryPropertyFlags,
}

#[derive(Debug)]
pub struct Buffer {
    buffer: vk::Buffer,
    allocations: vkx::BufferAllocations,
    allocation: vkx::BufferAllocation,
    descriptor: vkx::Descriptor,
}

impl GpuResource for Buffer {
    type CreateInfo<'a> = BufferCreateInfo;

    unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Buffer.
        let (buffer, buffer_create_info) =
            vkx::BufferCreator::new(create_info.size, create_info.usage)
                .create(device)
                .context("Creating buffer object")?;

        // Allocate.
        let allocations = vkx::BufferAllocations::allocate(
            physical_device,
            device,
            &[buffer],
            &[buffer_create_info],
            create_info.property_flags,
        )?;
        let allocation = allocations.allocations()[0];

        // Descriptor.
        let descriptor = vkx::Descriptor::create(
            physical_device,
            device,
            vkx::DescriptorCreateInfo::StorageBuffer {
                address: allocation.device_address(),
                range: create_info.size,
            },
        );

        Ok(Self {
            buffer,
            allocations,
            allocation,
            descriptor,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_buffer(self.buffer);
        self.allocations.free(device);
    }
}

impl Buffer {
    pub fn handle(&self) -> vk::Buffer {
        self.buffer
    }

    pub fn memory(&self) -> &vkx::BufferAllocation {
        &self.allocation
    }

    pub fn memory_mut(&mut self) -> &mut vkx::BufferAllocation {
        &mut self.allocation
    }

    pub fn descriptor(&self) -> vkx::Descriptor {
        self.descriptor
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

    allocations: vkx::ImageAllocations,

    pub image_view: vk::ImageView,
    pub image_view_create_info: vk::ImageViewCreateInfo,

    pub descriptor: vkx::Descriptor,
}

impl GpuResource for Image2d {
    type CreateInfo<'a> = Image2dCreateInfo;

    unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Image.
        let (image, image_create_info) = vkx::ImageCreator::new_2d(
            create_info.width,
            create_info.height,
            create_info.format,
            create_info.usage,
        )
        .samples(create_info.samples)
        .create(device)
        .context("Creating image")?;

        // Allocate.
        let allocations = vkx::ImageAllocations::allocate(
            physical_device,
            device,
            &[image],
            &[image_create_info],
            create_info.property_flags,
        )?;

        // Image view.
        let (image_view, image_view_create_info) =
            vkx::ImageViewCreator::new_2d(image, image_create_info.format)
                .create(device)
                .context("Creating image view")?;

        // Descriptor.
        let descriptor = if create_info
            .usage
            .contains(vk::ImageUsageFlagBits::Storage.into())
        {
            vkx::Descriptor::create(
                physical_device,
                device,
                vkx::DescriptorCreateInfo::StorageImage {
                    image_view,
                    image_layout: vk::ImageLayout::General,
                },
            )
        } else {
            vkx::Descriptor::create(
                physical_device,
                device,
                vkx::DescriptorCreateInfo::SampledImage {
                    image_view,
                    image_layout: vk::ImageLayout::ShaderReadOnlyOptimal,
                },
            )
        };

        Ok(Self {
            image_create_info,
            image,

            allocations,

            image_view,
            image_view_create_info,

            descriptor,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_image_view(self.image_view);
        device.destroy_image(self.image);
        self.allocations.free(device);
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
    pub descriptor: vkx::Descriptor,
}

impl GpuResource for Sampler {
    type CreateInfo<'a> = SamplerCreateInfo;

    unsafe fn create(
        Gpu {
            physical_device,
            device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        // Sampler.
        let (sampler, sampler_create_info) = vkx::SamplerCreator::new()
            .mag_filter(create_info.mag_filter)
            .min_filter(create_info.min_filter)
            .mipmap_mode(create_info.mipmap_mode)
            .address_mode_uvw(create_info.address_mode)
            .create(device)
            .context("Creating sampler")?;

        // Descriptor.
        let descriptor = vkx::Descriptor::create(
            physical_device,
            device,
            vkx::DescriptorCreateInfo::Sampler(sampler),
        );

        Ok(Self {
            sampler,
            sampler_create_info,
            descriptor,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_sampler(self.sampler);
    }
}

//
// Upload
//

pub unsafe fn multi_upload_images(
    gpu @ Gpu { device, .. }: &Gpu,
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
    let byte_size = datas.iter().map(Vec::len).sum::<usize>() as vk::DeviceSize;
    let staging = resource::Buffer::create(
        gpu,
        &BufferCreateInfo {
            size: byte_size,
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
            staging.memory().as_mut_ptr::<u8>().add(dst_offset),
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
        device.queue,
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
                stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
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
