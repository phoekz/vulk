use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "triangle";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands = command::Commands::create(gpu)?;
        let queries = query::Queries::create(gpu)?;
        let shaders = create_shaders(gpu)?;
        let format = vk::Format::R8g8b8a8Unorm;
        let width = 256;
        let height = 256;
        let render_targets = create_render_targets(gpu, format, width, height)?;
        let output = create_output(gpu, format, width, height)?;
        Ok(Self {
            commands,
            queries,
            shaders,
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
            queries,
            shaders,
            render_targets,
            output,
        } = state;
        destroy_output(gpu, &output);
        destroy_render_targets(gpu, &render_targets);
        destroy_shaders(gpu, &shaders);
        queries.destroy(gpu);
        commands.destroy(gpu);
        Ok(())
    }
}

//
// Shaders
//

struct Shaders {
    task: vk::ShaderEXT,
    mesh: vk::ShaderEXT,
    fragment: vk::ShaderEXT,
}

unsafe fn create_shaders(Gpu { device, .. }: &Gpu) -> Result<Shaders> {
    // Shader compiler
    let mut compiler = shader::Compiler::new()?;

    // Includes.
    compiler.include(
        "common.glsl",
        r#"
            #extension GL_EXT_mesh_shader : require

            struct MeshTask {
                float scale;
            };

            struct MeshVertex {
                vec3 color;
            };

            struct MeshPrimitive {
                float alpha;
            };
        "#,
    );

    // Shaders.
    let task_spirv = compiler.compile(
        r#"
            #version 460 core
            #include "common.glsl"

            taskPayloadSharedEXT MeshTask mesh_task;

            void main() {
                mesh_task.scale = 1.5;
                EmitMeshTasksEXT(1, 1, 1);
            }
        "#,
        shader::ShaderType::Task,
    )?;
    let mesh_spirv = compiler.compile(
        r#"
            #version 460 core
            #include "common.glsl"

            taskPayloadSharedEXT MeshTask mesh_task;

            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            layout(triangles, max_vertices = 3, max_primitives = 1) out;
            layout(location = 0) out MeshVertex mesh_vertices[];
            layout(location = 1) perprimitiveEXT flat out MeshPrimitive mesh_primitives[];

            void main() {
                const vec2 positions[3] = {
                    vec2(-0.5, -0.5),
                    vec2(0.0, 0.5),
                    vec2(0.5, -0.5),
                };
                const vec3 colors[3] = {
                    vec3(1.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                };
                const uvec3 triangles[1] = {
                    uvec3(0, 1, 2),
                };
                SetMeshOutputsEXT(3, 1);
                gl_MeshVerticesEXT[0].gl_Position = vec4(mesh_task.scale * positions[0], 0.0, 1.0);
                gl_MeshVerticesEXT[1].gl_Position = vec4(mesh_task.scale * positions[1], 0.0, 1.0);
                gl_MeshVerticesEXT[2].gl_Position = vec4(mesh_task.scale * positions[2], 0.0, 1.0);
                mesh_vertices[0].color = colors[0];
                mesh_vertices[1].color = colors[1];
                mesh_vertices[2].color = colors[2];
                gl_PrimitiveTriangleIndicesEXT[0] = triangles[0];
                mesh_primitives[0].alpha = 1.0;
            }
        "#,
        shader::ShaderType::Mesh,
    )?;
    let fragment_spirv = compiler.compile(
        r#"
            #version 460 core
            #include "common.glsl"

            layout(location = 0) in MeshVertex mesh_vertex;
            layout(location = 1) perprimitiveEXT flat in MeshPrimitive mesh_primitive;
            layout(location = 0) out vec4 fragment_color;

            void main() {
                fragment_color = vec4(mesh_vertex.color, mesh_primitive.alpha);
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
            stage: vk::ShaderStageFlagBits::TASK_EXT,
            next_stage: vk::ShaderStageFlags::MESH_EXT,
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: task_spirv.len(),
            p_code: task_spirv.as_ptr().cast(),
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
            stage: vk::ShaderStageFlagBits::MESH_EXT,
            next_stage: vk::ShaderStageFlags::FRAGMENT,
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: mesh_spirv.len(),
            p_code: mesh_spirv.as_ptr().cast(),
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
    let mut shaders: [vk::ShaderEXT; 3] = zeroed();
    device.create_shaders_ext(
        shader_create_infos.len() as _,
        shader_create_infos.as_ptr(),
        shaders.as_mut_ptr(),
    )?;

    Ok(Shaders {
        task: shaders[0],
        mesh: shaders[1],
        fragment: shaders[2],
    })
}

unsafe fn destroy_shaders(Gpu { device, .. }: &Gpu, shaders: &Shaders) {
    device.destroy_shader_ext(shaders.task);
    device.destroy_shader_ext(shaders.mesh);
    device.destroy_shader_ext(shaders.fragment);
}

//
// Render targets
//

struct RenderTargets {
    color: resource::Image2d,
}

unsafe fn create_render_targets(
    gpu: &Gpu,
    format: vk::Format,
    width: u32,
    height: u32,
) -> Result<RenderTargets> {
    let color = resource::Image2d::create(
        gpu,
        format,
        width,
        height,
        vk::SampleCountFlagBits::NUM_1,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;
    Ok(RenderTargets { color })
}

unsafe fn destroy_render_targets(gpu: &Gpu, rt: &RenderTargets) {
    rt.color.destroy(gpu);
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
    gpu @ Gpu { device, queue, .. }: &Gpu,
    Demo {
        commands,
        queries,
        shaders,
        render_targets,
        output,
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
    queries.begin(gpu, cmd);

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
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
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
            render_area: render_targets.color.rect_2d(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: render_targets.color.image_view,
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

    // Set rasterizer state.
    {
        let width = render_targets.color.width() as f32;
        let height = render_targets.color.height() as f32;

        device.cmd_set_cull_mode(cmd, vk::CullModeFlags::BACK);
        device.cmd_set_front_face(cmd, vk::FrontFace::Clockwise);
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
    }

    // Bind shaders.
    {
        let stages = [
            vk::ShaderStageFlagBits::TASK_EXT,
            vk::ShaderStageFlagBits::MESH_EXT,
            vk::ShaderStageFlagBits::FRAGMENT,
        ];
        let shaders = [shaders.task, shaders.mesh, shaders.fragment];
        device.cmd_bind_shaders_ext(cmd, stages.len() as _, stages.as_ptr(), shaders.as_ptr());
    }

    // Draw.
    device.cmd_draw_mesh_tasks_ext(cmd, 1, 1, 1);

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
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_targets.color.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_targets.color.width(),
                buffer_image_height: render_targets.color.height(),
                image_subresource: render_targets.color.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_targets.color.extent_3d(),
            }),
        }),
    );

    // End queries.
    queries.end(gpu, cmd);

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

    // Query results.
    {
        info!("Rendering took {:?}", queries.elapsed(gpu)?);
        info!("Rendering statistics: {:?}", queries.statistics(gpu)?);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.color.width();
        let height = render_targets.color.height();
        let pixels_byte_size = render_targets.color.byte_size();
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
