use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    geometry: Geometry,
    shaders: Shaders,
    timestamp_queries: TimestampQueries,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "triangle";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands = command::create(gpu)?;
        let geometry = create_geometry(gpu)?;
        let shaders = create_shaders(gpu)?;
        let timestamp_queries = create_timestamp_queries(gpu)?;
        let format = vk::Format::R8g8b8a8Unorm;
        let width = 256;
        let height = 256;
        let render_targets = create_render_targets(gpu, format, width, height)?;
        let output = create_output(gpu, format, width, height)?;
        Ok(Self {
            commands,
            geometry,
            shaders,
            timestamp_queries,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        let Self {
            commands,
            geometry,
            shaders,
            render_targets,
            output,
            timestamp_queries,
        } = state;
        destroy_output(gpu, &output);
        destroy_render_targets(gpu, &render_targets);
        destroy_timestamp_queries(gpu, &timestamp_queries);
        destroy_shaders(gpu, &shaders);
        destroy_geometry(gpu, &geometry);
        command::destroy(gpu, &commands);
        Ok(())
    }
}

//
// Geometry
//

struct Geometry {
    buffer: resource::Buffer<u8>,
    vertex_stride: u32,
    index_count: u32,
    instance_count: u32,
    index_offset: u64,
    index_type: vk::IndexType,
}

unsafe fn create_geometry(gpu: &Gpu) -> Result<Geometry> {
    struct Vertex {
        position: Vec2,
        color: Vec3,
    }

    let vertex_count = 3;
    let index_count = 3;
    let instance_count = 1;
    let vertex_stride = size_of::<Vertex>();
    let vertex_buffer_size = vertex_count * vertex_stride;
    let index_buffer_size = index_count * size_of::<u32>();
    let element_count = vertex_buffer_size + index_buffer_size;
    let usage = vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = resource::Buffer::create(gpu, element_count, usage, flags)?;
    let vertices: [Vertex; 3] = [
        Vertex {
            position: Vec2::new(-0.5, -0.5),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec2::new(0.0, 0.5),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec2::new(0.5, -0.5),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
    ];
    let indices: [u32; 3] = [0, 1, 2];
    std::ptr::copy_nonoverlapping(
        vertices.as_ptr().cast::<u8>(),
        buffer.ptr,
        vertex_buffer_size,
    );
    std::ptr::copy_nonoverlapping(
        indices.as_ptr().cast::<u8>(),
        buffer.ptr.add(vertex_buffer_size),
        index_buffer_size,
    );
    Ok(Geometry {
        buffer,
        vertex_stride: vertex_stride as _,
        index_count: index_count as _,
        instance_count,
        index_offset: vertex_buffer_size as _,
        index_type: vk::IndexType::Uint32,
    })
}

unsafe fn destroy_geometry(gpu: &Gpu, geometry: &Geometry) {
    geometry.buffer.destroy(gpu);
}

//
// Shaders
//

struct Shaders {
    vertex: vk::ShaderEXT,
    fragment: vk::ShaderEXT,
}

unsafe fn create_shaders(Gpu { device, .. }: &Gpu) -> Result<Shaders> {
    // Shader compiler
    let compiler = shader::Compiler::new()?;

    // Shaders.
    let vertex_spirv = compiler.compile(
        r#"
            #version 460 core

            layout(location = 0) in vec2 input_position;
            layout(location = 1) in vec3 input_color;
            layout(location = 0) out vec3 output_color;

            void main() {
                gl_Position = vec4(input_position, 0.0, 1.0);
                output_color = input_color;
            }
        "#,
        shader::ShaderType::Vertex,
    )?;
    let fragment_spirv = compiler.compile(
        r#"
            #version 460 core

            layout(location = 0) in vec3 input_color;
            layout(location = 0) out vec4 output_color;

            void main() {
                output_color = vec4(input_color, 1.0);
            }
        "#,
        shader::ShaderType::Fragment,
    )?;

    // Shader objects.
    let shader_create_infos = [
        vk::ShaderCreateInfoEXT {
            s_type: vk::StructureType::ShaderCreateInfoEXT,
            p_next: null(),
            flags: vk::ShaderCreateFlagsEXT::LINK_STAGE_EXT,
            stage: vk::ShaderStageFlagBits::VERTEX,
            next_stage: vk::ShaderStageFlags::FRAGMENT,
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: vertex_spirv.len(),
            p_code: vertex_spirv.as_ptr().cast(),
            p_name: b"main\0".as_ptr().cast(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
            p_specialization_info: null(),
        },
        vk::ShaderCreateInfoEXT {
            s_type: vk::StructureType::ShaderCreateInfoEXT,
            p_next: null(),
            flags: vk::ShaderCreateFlagsEXT::LINK_STAGE_EXT,
            stage: vk::ShaderStageFlagBits::FRAGMENT,
            next_stage: vk::ShaderStageFlags::empty(),
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: fragment_spirv.len(),
            p_code: fragment_spirv.as_ptr().cast(),
            p_name: b"main\0".as_ptr().cast(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
            p_specialization_info: null(),
        },
    ];
    let mut shaders: [vk::ShaderEXT; 2] = zeroed();
    device.create_shaders_ext(
        shader_create_infos.len() as _,
        shader_create_infos.as_ptr(),
        shaders.as_mut_ptr(),
    )?;

    Ok(Shaders {
        vertex: shaders[0],
        fragment: shaders[1],
    })
}

unsafe fn destroy_shaders(Gpu { device, .. }: &Gpu, shaders: &Shaders) {
    device.destroy_shader_ext(shaders.vertex);
    device.destroy_shader_ext(shaders.fragment);
}

//
// Timestamp queries
//

struct TimestampQueries {
    query_pool: vk::QueryPool,
    query_pool_create_info: vk::QueryPoolCreateInfo,
}

unsafe fn create_timestamp_queries(Gpu { device, .. }: &Gpu) -> Result<TimestampQueries> {
    let query_count = 2;
    let query_pool_create_info = vk::QueryPoolCreateInfo {
        s_type: vk::StructureType::QueryPoolCreateInfo,
        p_next: null(),
        flags: vk::QueryPoolCreateFlags::empty(),
        query_type: vk::QueryType::Timestamp,
        query_count,
        pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
    };
    let query_pool = device.create_query_pool(&query_pool_create_info)?;
    device.reset_query_pool(query_pool, 0, query_count);
    Ok(TimestampQueries {
        query_pool,
        query_pool_create_info,
    })
}

unsafe fn destroy_timestamp_queries(
    Gpu { device, .. }: &Gpu,
    timestamp_queries: &TimestampQueries,
) {
    device.destroy_query_pool(timestamp_queries.query_pool);
}

//
// Render targets
//

struct RenderTargets {
    color_image: resource::Image2d,
}

unsafe fn create_render_targets(
    gpu: &Gpu,
    format: vk::Format,
    width: u32,
    height: u32,
) -> Result<RenderTargets> {
    let color_image = resource::Image2d::create(
        gpu,
        format,
        width,
        height,
        vk::SampleCountFlagBits::NUM_1,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;
    Ok(RenderTargets { color_image })
}

unsafe fn destroy_render_targets(gpu: &Gpu, rt: &RenderTargets) {
    rt.color_image.destroy(gpu);
}

//
// Output
//

type OutputBuffer = resource::Buffer<u32>;

struct Output {
    buffer: OutputBuffer,
}

unsafe fn create_output(gpu: &Gpu, format: vk::Format, width: u32, height: u32) -> Result<Output> {
    let element_count = format.block_size() * width * height;
    let buffer = OutputBuffer::create(
        gpu,
        element_count as _,
        vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;
    Ok(Output { buffer })
}

unsafe fn destroy_output(gpu: &Gpu, output: &Output) {
    output.buffer.destroy(gpu);
}

//
// Draw
//

unsafe fn draw(
    Gpu {
        device,
        queue,
        physical_device,
        ..
    }: &Gpu,
    Demo {
        commands,
        render_targets,
        output,
        timestamp_queries,
        geometry,
        shaders,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.command_buffer;
    device.begin_command_buffer(
        cmd,
        &(vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: null(),
        }),
    )?;

    // Begin queries.
    device.cmd_write_timestamp2(
        cmd,
        vk::PipelineStageFlags2::TOP_OF_PIPE,
        timestamp_queries.query_pool,
        0,
    );

    // Transition render target.
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
                src_stage_mask: vk::PipelineStageFlags2::TOP_OF_PIPE,
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color_image.image,
                subresource_range: render_targets.color_image.subresource_range(),
            },
        },
    );

    // Begin rendering.
    device.cmd_begin_rendering(
        cmd,
        &(vk::RenderingInfo {
            s_type: vk::StructureType::RenderingInfo,
            p_next: null(),
            flags: vk::RenderingFlags::empty(),
            render_area: render_targets.color_image.rect_2d(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: render_targets.color_image.image_view,
                image_layout: vk::ImageLayout::AttachmentOptimal,
                resolve_mode: vk::ResolveModeFlagBits::NONE,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.2, 0.2, 0.2, 1.0],
                    },
                },
            }),
            p_depth_attachment: null(),
            p_stencil_attachment: null(),
        }),
    );

    // Bind shaders.
    {
        let stages = [
            vk::ShaderStageFlagBits::VERTEX,
            vk::ShaderStageFlagBits::FRAGMENT,
        ];
        let shaders = [shaders.vertex, shaders.fragment];
        device.cmd_bind_shaders_ext(cmd, 2, stages.as_ptr(), shaders.as_ptr());
    }

    // Bind geometry.
    {
        device.cmd_set_vertex_input_ext(
            cmd,
            1,
            &(vk::VertexInputBindingDescription2EXT {
                s_type: vk::StructureType::VertexInputBindingDescription2EXT,
                p_next: null_mut(),
                binding: 0,
                stride: geometry.vertex_stride,
                input_rate: vk::VertexInputRate::Vertex,
                divisor: 1,
            }),
            2,
            [
                vk::VertexInputAttributeDescription2EXT {
                    s_type: vk::StructureType::VertexInputAttributeDescription2EXT,
                    p_next: null_mut(),
                    location: 0,
                    binding: 0,
                    format: vk::Format::R32g32Sfloat,
                    offset: 0,
                },
                vk::VertexInputAttributeDescription2EXT {
                    s_type: vk::StructureType::VertexInputAttributeDescription2EXT,
                    p_next: null_mut(),
                    location: 1,
                    binding: 0,
                    format: vk::Format::R32g32b32Sfloat,
                    offset: 8,
                },
            ]
            .as_ptr(),
        );

        let buffers = [geometry.buffer.buffer];
        let offsets = [0];
        device.cmd_bind_vertex_buffers2(
            cmd,
            0,
            1,
            buffers.as_ptr(),
            offsets.as_ptr(),
            null(),
            null(),
        );
        device.cmd_bind_index_buffer(
            cmd,
            geometry.buffer.buffer,
            geometry.index_offset,
            geometry.index_type,
        );
    }

    // Set rasterizer state.
    {
        let width = render_targets.color_image.width() as f32;
        let height = render_targets.color_image.height() as f32;

        device.cmd_set_primitive_topology(cmd, vk::PrimitiveTopology::TriangleList);
        device.cmd_set_viewport_with_count(
            cmd,
            1,
            &vk::Viewport {
                x: 0.0,
                y: height,
                width,
                height: -height,
                min_depth: 0.0,
                max_depth: 1.0,
            },
        );
        device.cmd_set_scissor_with_count(cmd, 1, &render_targets.color_image.rect_2d());
        device.cmd_set_front_face(cmd, vk::FrontFace::Clockwise);
        device.cmd_set_cull_mode(cmd, vk::CullModeFlags::BACK);
    }

    // Draw.
    device.cmd_draw_indexed(cmd, geometry.index_count, geometry.instance_count, 0, 0, 0);

    // End rendering.
    device.cmd_end_rendering(cmd);

    // Transition render target.
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
                src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                src_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags2::COPY,
                dst_access_mask: vk::AccessFlags2::TRANSFER_READ,
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color_image.image,
                subresource_range: render_targets.color_image.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_targets.color_image.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_targets.color_image.width(),
                buffer_image_height: render_targets.color_image.height(),
                image_subresource: render_targets.color_image.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_targets.color_image.extent_3d(),
            }),
        }),
    );

    // End queries.
    device.cmd_write_timestamp2(
        cmd,
        vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
        timestamp_queries.query_pool,
        1,
    );

    // End command buffer.
    device.end_command_buffer(cmd)?;

    // Queue submit.
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
                stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                device_index: 0,
            }),
        }),
        vk::Fence::null(),
    )?;

    // Wait for semaphore.
    {
        let semaphores = [commands.semaphore];
        let values = [1];
        device.wait_semaphores(
            &(vk::SemaphoreWaitInfo {
                s_type: vk::StructureType::SemaphoreWaitInfo,
                p_next: null(),
                flags: vk::SemaphoreWaitFlags::ANY,
                semaphore_count: semaphores.len() as _,
                p_semaphores: semaphores.as_ptr(),
                p_values: values.as_ptr(),
            }),
            u64::MAX,
        )?;
    }

    // Read timestamp.
    {
        let query_count = timestamp_queries.query_pool_create_info.query_count;
        let mut timestamps = vec![0_u64; query_count as _];
        device.get_query_pool_results(
            timestamp_queries.query_pool,
            0,
            query_count,
            size_of::<u64>() * query_count as usize,
            timestamps.as_mut_ptr().cast(),
            size_of::<u64>() as _,
            vk::QueryResultFlags::NUM_64 | vk::QueryResultFlags::WAIT,
        )?;

        let elapsed = timestamps[1]
            .checked_sub(timestamps[0])
            .expect("Later timestamp is larger than earlier timestamp");
        let elapsed_ns = elapsed as f32 * physical_device.properties.limits.timestamp_period;
        let elapsed_ms = elapsed_ns / 1e6;
        info!("Rendering took {elapsed_ms} ms, raw={:?}", timestamps);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.color_image.width();
        let height = render_targets.color_image.height();
        let pixels_byte_size = render_targets.color_image.byte_size();
        let mut pixels = vec![0_u8; pixels_byte_size as _];
        std::ptr::copy_nonoverlapping(
            output.buffer.ptr.cast::<u8>(),
            pixels.as_mut_ptr(),
            pixels_byte_size as _,
        );
        let image = RgbaImage::from_raw(width, height, pixels)
            .context("Creating image from output buffer")?;
        let image_path = work_dir_or_create()?.join(format!("{demo_name}.png"));
        image.save_with_format(&image_path, ImageFormat::Png)?;
        info!("Wrote image to {}", image_path.display());
    }

    Ok(())
}
