use std::mem::MaybeUninit;

use super::*;

pub struct CommandBuffer {
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    //
    // General
    //

    pub unsafe fn create(device: &Device) -> Result<Self> {
        // Command pool.
        let command_pool = device.create_command_pool(&vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: null(),
            flags: vk::CommandPoolCreateFlags::empty(),
            queue_family_index: device.queue_family_index,
        })?;

        // Command buffer.
        let command_buffer = {
            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::CommandBufferAllocateInfo,
                p_next: null(),
                command_pool,
                level: vk::CommandBufferLevel::Primary,
                command_buffer_count: 1,
            };
            let mut command_buffer = MaybeUninit::uninit();
            device.allocate_command_buffers(
                &command_buffer_allocate_info,
                command_buffer.as_mut_ptr(),
            )?;
            command_buffer.assume_init()
        };

        Ok(Self {
            command_pool,
            command_buffer,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.free_command_buffers(self.command_pool, 1, &self.command_buffer);
        device.destroy_command_pool(self.command_pool);
    }

    pub unsafe fn begin(&self, device: &Device) -> Result<()> {
        // Reset command buffers by resetting the pool.
        device.reset_command_pool(self.command_pool, vk::CommandPoolResetFlags::empty())?;

        // Begin command buffer.
        device.begin_command_buffer(
            self.command_buffer,
            &vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::CommandBufferBeginInfo,
                p_next: null(),
                flags: vk::CommandBufferUsageFlagBits::OneTimeSubmit.into(),
                p_inheritance_info: null(),
            },
        )?;

        Ok(())
    }

    pub unsafe fn end(&self, device: &Device) -> Result<()> {
        device.end_command_buffer(self.command_buffer)?;
        Ok(())
    }

    #[must_use]
    pub unsafe fn handle(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    //
    // Dynamic rendering
    //

    pub unsafe fn begin_rendering<Image>(
        &self,
        device: &Device,
        color_attachment: (&Image, [f32; 4]),
        depth_attachment: Option<(&Image, f32)>,
        resolve_attachment: Option<&Image>,
    ) where
        Image: ImageOps,
    {
        // Color attachment.
        let mut color_attachment_info = vk::RenderingAttachmentInfo {
            s_type: vk::StructureType::RenderingAttachmentInfo,
            p_next: null(),
            image_view: color_attachment.0.image_view_handle(),
            image_layout: vk::ImageLayout::AttachmentOptimal,
            resolve_mode: vk::ResolveModeFlagBits::None,
            resolve_image_view: vk::ImageView::null(),
            resolve_image_layout: vk::ImageLayout::Undefined,
            load_op: vk::AttachmentLoadOp::Clear,
            store_op: vk::AttachmentStoreOp::Store,
            clear_value: vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: color_attachment.1,
                },
            },
        };

        // Optional: resolve attachment.
        if let Some(resolve_attachment) = resolve_attachment {
            color_attachment_info.resolve_mode = vk::ResolveModeFlagBits::Average;
            color_attachment_info.resolve_image_view = resolve_attachment.image_view_handle();
            color_attachment_info.resolve_image_layout = vk::ImageLayout::AttachmentOptimal;
        }

        // Optional: depth attachment.
        let mut depth_attachment_info = vk::RenderingAttachmentInfo {
            s_type: vk::StructureType::RenderingAttachmentInfo,
            p_next: null(),
            image_view: vk::ImageView::null(),
            image_layout: vk::ImageLayout::AttachmentOptimal,
            resolve_mode: vk::ResolveModeFlagBits::None,
            resolve_image_view: vk::ImageView::null(),
            resolve_image_layout: vk::ImageLayout::Undefined,
            load_op: vk::AttachmentLoadOp::Clear,
            store_op: vk::AttachmentStoreOp::Store,
            clear_value: vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        };
        if let Some((depth_attachment, depth)) = depth_attachment {
            depth_attachment_info.image_view = depth_attachment.image_view_handle();
            depth_attachment_info.clear_value.depth_stencil.depth = depth;
        }

        // Rendering info.
        let mut rendering_info = vk::RenderingInfo {
            s_type: vk::StructureType::RenderingInfo,
            p_next: null(),
            flags: vk::RenderingFlags::empty(),
            render_area: color_attachment.0.rect_2d(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_info,
            p_depth_attachment: null(),
            p_stencil_attachment: null(),
        };
        if depth_attachment.is_some() {
            rendering_info.p_depth_attachment = &depth_attachment_info;
        }

        // Begin rendering.
        device.cmd_begin_rendering(self.command_buffer, &rendering_info);
    }

    pub unsafe fn end_rendering(&self, device: &Device) {
        device.cmd_end_rendering(self.command_buffer);
    }

    //
    // Barriers
    //

    pub unsafe fn barrier(
        &self,
        device: &Device,
        src_stage_mask: impl Into<vk::PipelineStageFlags2>,
        src_access_mask: impl Into<vk::AccessFlags2>,
        dst_stage_mask: impl Into<vk::PipelineStageFlags2>,
        dst_access_mask: impl Into<vk::AccessFlags2>,
    ) {
        device.cmd_pipeline_barrier2(
            self.command_buffer,
            &vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 1,
                p_memory_barriers: &vk::MemoryBarrier2 {
                    s_type: vk::StructureType::MemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: src_stage_mask.into(),
                    src_access_mask: src_access_mask.into(),
                    dst_stage_mask: dst_stage_mask.into(),
                    dst_access_mask: dst_access_mask.into(),
                },
                buffer_memory_barrier_count: 0,
                p_buffer_memory_barriers: null(),
                image_memory_barrier_count: 0,
                p_image_memory_barriers: null(),
            },
        );
    }

    pub unsafe fn buffer_barrier(
        &self,
        device: &Device,
        buffer: &impl BufferOps,
        src_stage_mask: impl Into<vk::PipelineStageFlags2>,
        src_access_mask: impl Into<vk::AccessFlags2>,
        dst_stage_mask: impl Into<vk::PipelineStageFlags2>,
        dst_access_mask: impl Into<vk::AccessFlags2>,
    ) {
        device.cmd_pipeline_barrier2(
            self.command_buffer,
            &vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 0,
                p_memory_barriers: null(),
                buffer_memory_barrier_count: 1,
                p_buffer_memory_barriers: &vk::BufferMemoryBarrier2 {
                    s_type: vk::StructureType::BufferMemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: src_stage_mask.into(),
                    src_access_mask: src_access_mask.into(),
                    dst_stage_mask: dst_stage_mask.into(),
                    dst_access_mask: dst_access_mask.into(),
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    buffer: buffer.buffer_handle(),
                    offset: 0,
                    size: vk::WHOLE_SIZE,
                },
                image_memory_barrier_count: 0,
                p_image_memory_barriers: null(),
            },
        );
    }

    pub unsafe fn image_barrier(
        &self,
        device: &Device,
        image: &impl ImageOps,
        src_stage_mask: impl Into<vk::PipelineStageFlags2>,
        src_access_mask: impl Into<vk::AccessFlags2>,
        dst_stage_mask: impl Into<vk::PipelineStageFlags2>,
        dst_access_mask: impl Into<vk::AccessFlags2>,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
    ) {
        device.cmd_pipeline_barrier2(
            self.command_buffer,
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
                    src_stage_mask: src_stage_mask.into(),
                    src_access_mask: src_access_mask.into(),
                    dst_stage_mask: dst_stage_mask.into(),
                    dst_access_mask: dst_access_mask.into(),
                    old_layout,
                    new_layout,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: image.image_handle(),
                    subresource_range: image.subresource_range(),
                },
            },
        );
    }

    //
    // Draws/dispatches
    //

    pub unsafe fn draw_mesh_tasks(
        &self,
        device: &Device,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        device.cmd_draw_mesh_tasks_ext(
            self.command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }

    pub unsafe fn draw_mesh_tasks_indirect(
        &self,
        device: &Device,
        buffer: &impl BufferOps,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        device.cmd_draw_mesh_tasks_indirect_ext(
            self.command_buffer,
            buffer.buffer_handle(),
            offset,
            draw_count,
            stride,
        );
    }

    pub unsafe fn draw_mesh_tasks_indirect_count(
        &self,
        device: &Device,
        buffer: &impl BufferOps,
        offset: vk::DeviceSize,
        count_buffer: &impl BufferOps,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        device.cmd_draw_mesh_tasks_indirect_count_ext(
            self.command_buffer,
            buffer.buffer_handle(),
            offset,
            count_buffer.buffer_handle(),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    pub unsafe fn dispatch(
        &self,
        device: &Device,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        device.cmd_dispatch(
            self.command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }

    pub unsafe fn dispatch_indirect(
        &self,
        device: &Device,
        buffer: &impl BufferOps,
        offset: vk::DeviceSize,
    ) {
        device.cmd_dispatch_indirect(self.command_buffer, buffer.buffer_handle(), offset);
    }

    //
    // Copies
    //

    pub unsafe fn copy_buffer(
        &self,
        device: &Device,
        src_buffer: (&impl BufferOps, vk::DeviceSize),
        dst_buffer: (&impl BufferOps, vk::DeviceSize),
        size: vk::DeviceSize,
    ) {
        device.cmd_copy_buffer2(
            self.command_buffer,
            &vk::CopyBufferInfo2 {
                s_type: vk::StructureType::CopyBufferInfo2,
                p_next: null(),
                src_buffer: src_buffer.0.buffer_handle(),
                dst_buffer: dst_buffer.0.buffer_handle(),
                region_count: 1,
                p_regions: &vk::BufferCopy2 {
                    s_type: vk::StructureType::BufferCopy2,
                    p_next: null(),
                    src_offset: src_buffer.1,
                    dst_offset: dst_buffer.1,
                    size,
                },
            },
        );
    }

    pub unsafe fn copy_image(
        &self,
        device: &Device,
        src_image: (&impl ImageOps, vk::ImageLayout),
        dst_image: (&impl ImageOps, vk::ImageLayout),
        extent: vk::Extent3D,
    ) {
        device.cmd_copy_image2(
            self.command_buffer,
            &vk::CopyImageInfo2 {
                s_type: vk::StructureType::CopyImageInfo2,
                p_next: null(),
                src_image: src_image.0.image_handle(),
                src_image_layout: src_image.1,
                dst_image: dst_image.0.image_handle(),
                dst_image_layout: dst_image.1,
                region_count: 1,
                p_regions: &vk::ImageCopy2 {
                    s_type: vk::StructureType::ImageCopy2,
                    p_next: null(),
                    src_subresource: src_image.0.subresource_layers(),
                    src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    dst_subresource: dst_image.0.subresource_layers(),
                    dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    extent,
                },
            },
        );
    }

    pub unsafe fn copy_buffer_to_image(
        &self,
        device: &Device,
        src_buffer: (&impl BufferOps, vk::DeviceSize),
        dst_image: &impl ImageOps,
    ) {
        device.cmd_copy_buffer_to_image2(
            self.command_buffer,
            &vk::CopyBufferToImageInfo2 {
                s_type: vk::StructureType::CopyBufferToImageInfo2,
                p_next: null(),
                src_buffer: src_buffer.0.buffer_handle(),
                dst_image: dst_image.image_handle(),
                dst_image_layout: vk::ImageLayout::TransferDstOptimal,
                region_count: 1,
                p_regions: &vk::BufferImageCopy2 {
                    s_type: vk::StructureType::BufferImageCopy2,
                    p_next: null(),
                    buffer_offset: src_buffer.1,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: dst_image.subresource_layers(),
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: dst_image.extent_3d(),
                },
            },
        );
    }

    pub unsafe fn copy_image_to_buffer(
        &self,
        device: &Device,
        src_image: &impl ImageOps,
        dst_buffer: (&impl BufferOps, vk::DeviceSize),
    ) {
        device.cmd_copy_image_to_buffer2(
            self.command_buffer,
            &vk::CopyImageToBufferInfo2 {
                s_type: vk::StructureType::CopyImageToBufferInfo2,
                p_next: null(),
                src_image: src_image.image_handle(),
                src_image_layout: vk::ImageLayout::TransferSrcOptimal,
                dst_buffer: dst_buffer.0.buffer_handle(),
                region_count: 1,
                p_regions: &vk::BufferImageCopy2 {
                    s_type: vk::StructureType::BufferImageCopy2,
                    p_next: null(),
                    buffer_offset: dst_buffer.1,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: src_image.subresource_layers(),
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: src_image.extent_3d(),
                },
            },
        );
    }

    //
    // Descriptors/push constants
    //

    pub unsafe fn bind_descriptor_storage(
        &self,
        device: &Device,
        descriptor_storage: &DescriptorStorage,
        pipeline_bind_point: vk::PipelineBindPoint,
    ) {
        let ds = descriptor_storage;
        device.cmd_bind_descriptor_buffers_ext(
            self.command_buffer,
            1,
            &vk::DescriptorBufferBindingInfoEXT {
                s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
                p_next: null_mut(),
                address: ds.allocation.device_address(),
                usage: ds.usage,
            },
        );
        device.cmd_set_descriptor_buffer_offsets_ext(
            self.command_buffer,
            pipeline_bind_point,
            ds.pipeline_layout,
            0,
            ds.set_count,
            ds.buffer_indices.as_ptr(),
            ds.offsets.as_ptr(),
        );
    }

    pub unsafe fn push_constants<T>(
        &self,
        device: &Device,
        descriptor_storage: &DescriptorStorage,
        data: &T,
    ) -> Result<()> {
        let ds = descriptor_storage;
        let Some(pcr) = ds.push_constant_range else {
            bail!("Missing push constant range");
        };
        ensure!(pcr.size as usize == size_of::<T>());
        device.cmd_push_constants(
            self.command_buffer,
            ds.pipeline_layout,
            pcr.stage_flags,
            pcr.offset,
            pcr.size,
            (data as *const T).cast(),
        );
        Ok(())
    }

    //
    // Shaders
    //

    pub unsafe fn bind_shader(&self, device: &Device, shader: &Shader) {
        device.cmd_bind_shaders_ext(
            self.command_buffer,
            shader.stages.len() as _,
            shader.stages.as_ptr(),
            shader.shaders.as_ptr(),
        );
    }

    //
    // Rasterizer
    //

    pub unsafe fn set_viewport(&self, device: &Device, viewport: &vk::Viewport) {
        device.cmd_set_viewport_with_count(self.command_buffer, 1, viewport);
    }

    pub unsafe fn set_viewport_flip_y(&self, device: &Device, viewport: &vk::Viewport) {
        // VK_KHR_maintenance1: Allow negative height to be specified in the
        // VkViewport::height field to perform y-inversion of the clip-space to
        // framebuffer-space transform. This allows apps to avoid having to use
        // gl_Position.y = -gl_Position.y in shaders also targeting other APIs.
        self.set_viewport(
            device,
            &vk::Viewport {
                x: viewport.x,
                y: viewport.y + viewport.height,
                width: viewport.width,
                height: -viewport.height,
                min_depth: viewport.min_depth,
                max_depth: viewport.max_depth,
            },
        );
    }

    pub unsafe fn set_scissor(&self, device: &Device, scissor: &vk::Rect2D) {
        device.cmd_set_scissor_with_count(self.command_buffer, 1, scissor);
    }

    pub unsafe fn set_samples(&self, device: &Device, samples: vk::SampleCountFlagBits) {
        device.cmd_set_rasterization_samples_ext(self.command_buffer, samples);
    }

    pub unsafe fn set_front_face(&self, device: &Device, front_face: vk::FrontFace) {
        device.cmd_set_front_face(self.command_buffer, front_face);
    }

    pub unsafe fn set_cull_mode(&self, device: &Device, cull_mode: impl Into<vk::CullModeFlags>) {
        device.cmd_set_cull_mode(self.command_buffer, cull_mode.into());
    }

    pub unsafe fn set_depth_test(&self, device: &Device, depth_test: bool) {
        device.cmd_set_depth_test_enable(self.command_buffer, depth_test.into());
    }

    pub unsafe fn set_depth_compare_op(&self, device: &Device, depth_compare_op: vk::CompareOp) {
        device.cmd_set_depth_compare_op(self.command_buffer, depth_compare_op);
    }

    pub unsafe fn set_depth_write(&self, device: &Device, depth_write: bool) {
        device.cmd_set_depth_write_enable(self.command_buffer, depth_write.into());
    }

    pub unsafe fn set_color_blend(&self, device: &Device, attachment: u32, color_blend: bool) {
        device.cmd_set_color_blend_enable_ext(
            self.command_buffer,
            attachment,
            1,
            [color_blend.into()].as_ptr(),
        );
    }

    pub unsafe fn set_color_blend_equation(
        &self,
        device: &Device,
        attachment: u32,
        color_blend_equation: &vk::ColorBlendEquationEXT,
    ) {
        device.cmd_set_color_blend_equation_ext(
            self.command_buffer,
            attachment,
            1,
            color_blend_equation,
        );
    }

    pub unsafe fn set_color_write_mask(
        &self,
        device: &Device,
        attachment: u32,
        mask: vk::ColorComponentFlags,
    ) {
        device.cmd_set_color_write_mask_ext(self.command_buffer, attachment, 1, [mask].as_ptr());
    }

    pub unsafe fn set_color_write_mask_rgba(&self, device: &Device, attachment: u32) {
        self.set_color_write_mask(
            device,
            attachment,
            vk::ColorComponentFlagBits::R
                | vk::ColorComponentFlagBits::G
                | vk::ColorComponentFlagBits::B
                | vk::ColorComponentFlagBits::A,
        );
    }

    //
    // Queries
    //

    pub unsafe fn write_timestamp(&self, device: &Device, timestamps: &TimestampQuery, index: u32) {
        device.cmd_write_timestamp2(
            self.command_buffer,
            vk::PipelineStageFlagBits2::AllCommands.into(),
            timestamps.query_pool,
            index,
        );
    }

    pub unsafe fn begin_statistics(&self, device: &Device, statistics: &StatisticsQuery) {
        device.cmd_begin_query(
            self.command_buffer,
            statistics.pipeline_statistic_query_pool,
            0,
            vk::QueryControlFlags::empty(),
        );
        device.cmd_begin_query(
            self.command_buffer,
            statistics.mesh_primitives_generated_query_pool,
            0,
            vk::QueryControlFlags::empty(),
        );
    }

    pub unsafe fn end_statistics(&self, device: &Device, statistics: &StatisticsQuery) {
        device.cmd_end_query(
            self.command_buffer,
            statistics.pipeline_statistic_query_pool,
            0,
        );
        device.cmd_end_query(
            self.command_buffer,
            statistics.mesh_primitives_generated_query_pool,
            0,
        );
    }

    //
    // Ray tracing
    //

    pub unsafe fn build_acceleration_structures(
        &self,
        device: &Device,
        ty: vk::AccelerationStructureTypeKHR,
        dst_acceleration_structure: vk::AccelerationStructureKHR,
        geometry_count: u32,
        geometries: &vk::AccelerationStructureGeometryKHR,
        primitive_count: u32,
        scratch_data: &impl BufferOps,
    ) {
        let acceleration_structure_build_geometry_info_khr =
            vk::AccelerationStructureBuildGeometryInfoKHR {
                s_type: vk::StructureType::AccelerationStructureBuildGeometryInfoKHR,
                p_next: null(),
                ty,
                flags: vk::BuildAccelerationStructureFlagBitsKHR::PreferFastTraceKHR.into(),
                mode: vk::BuildAccelerationStructureModeKHR::BuildKHR,
                src_acceleration_structure: vk::AccelerationStructureKHR::null(),
                dst_acceleration_structure,
                geometry_count,
                p_geometries: geometries as *const _,
                pp_geometries: null(),
                scratch_data: vk::DeviceOrHostAddressKHR {
                    device_address: scratch_data.memory().device_address(),
                },
            };
        let acceleration_structure_build_range_info_khr =
            vk::AccelerationStructureBuildRangeInfoKHR {
                primitive_count,
                primitive_offset: 0,
                first_vertex: 0,
                transform_offset: 0,
            };

        device.cmd_build_acceleration_structures_khr(
            self.command_buffer,
            1,
            &acceleration_structure_build_geometry_info_khr,
            [addr_of!(acceleration_structure_build_range_info_khr)].as_ptr(),
        );
    }

    pub unsafe fn bind_ray_tracing_pipeline(&self, device: &Device, pipeline: vk::Pipeline) {
        device.cmd_bind_pipeline(
            self.command_buffer,
            vk::PipelineBindPoint::RayTracingKHR,
            pipeline,
        );
    }

    pub unsafe fn trace_rays<Sbt>(
        &self,
        physical_device: &PhysicalDevice,
        device: &Device,
        raygen_sbt: Option<&Sbt>,
        miss_sbt: Option<&Sbt>,
        hit_sbt: Option<&Sbt>,
        callable_sbt: Option<&Sbt>,
        width: u32,
        height: u32,
    ) where
        Sbt: BufferOps,
    {
        let rt_props = &physical_device.raytracing_pipeline_properties;
        let shader_group_handle_size = u64::from(rt_props.shader_group_handle_size);

        let mut raygen_sdar: vk::StridedDeviceAddressRegionKHR = zeroed();
        if let Some(sbt) = raygen_sbt {
            raygen_sdar.device_address = sbt.memory().device_address();
            raygen_sdar.stride = shader_group_handle_size;
            raygen_sdar.size = shader_group_handle_size;
        }

        let mut miss_sdar: vk::StridedDeviceAddressRegionKHR = zeroed();
        if let Some(sbt) = miss_sbt {
            miss_sdar.device_address = sbt.memory().device_address();
            miss_sdar.stride = shader_group_handle_size;
            miss_sdar.size = shader_group_handle_size;
        }

        let mut hit_sdar: vk::StridedDeviceAddressRegionKHR = zeroed();
        if let Some(sbt) = hit_sbt {
            hit_sdar.device_address = sbt.memory().device_address();
            hit_sdar.stride = shader_group_handle_size;
            hit_sdar.size = shader_group_handle_size;
        }

        let mut callable_sdar: vk::StridedDeviceAddressRegionKHR = zeroed();
        if let Some(sbt) = callable_sbt {
            callable_sdar.device_address = sbt.memory().device_address();
            callable_sdar.stride = shader_group_handle_size;
            callable_sdar.size = shader_group_handle_size;
        }

        device.cmd_trace_rays_khr(
            self.command_buffer,
            &raygen_sdar,
            &miss_sdar,
            &hit_sdar,
            &callable_sdar,
            width,
            height,
            1,
        );
    }
}
