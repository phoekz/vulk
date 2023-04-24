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
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
        let shaders = Shaders::create(gpu, &ShadersCreateInfo {})?;
        let render_targets = RenderTargets::create(gpu, &RenderTargetsCreateInfo {})?;
        let output = Output::create(gpu, &OutputCreateInfo {})?;
        Ok(Self {
            commands,
            queries,
            shaders,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(gpu);
        state.render_targets.destroy(gpu);
        state.shaders.destroy(gpu);
        state.queries.destroy(gpu);
        state.commands.destroy(gpu);
        Ok(())
    }
}

//
// Shaders
//

struct ShadersCreateInfo {}

struct Shaders {
    shader: shader::Shader,
}

impl GpuResource for Shaders {
    type CreateInfo<'a> = ShadersCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
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
            shader::ShaderType::Task,
            r#"
            #version 460 core
            #include "common.glsl"

            taskPayloadSharedEXT MeshTask mesh_task;

            void main() {
                mesh_task.scale = 1.5;
                EmitMeshTasksEXT(1, 1, 1);
            }
        "#,
        )?;
        let mesh_spirv = compiler.compile(
            shader::ShaderType::Mesh,
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
        )?;
        let fragment_spirv = compiler.compile(
            shader::ShaderType::Fragment,
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
        )?;

        let shader = shader::Shader::create(
            gpu,
            &shader::ShaderCreateInfo {
                spirvs: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: &[],
                push_constant_ranges: &[],
                specialization_info: None,
            },
        )?;

        Ok(Self { shader })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.shader.destroy(gpu);
    }
}

//
// Render targets
//

struct RenderTargetsCreateInfo {}

struct RenderTargets {
    color: resource::Image2d,
}

impl GpuResource for RenderTargets {
    type CreateInfo<'a> = RenderTargetsCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let color = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: vk::SampleCountFlagBits::Count1,
                usage: vk::ImageUsageFlagBits::ColorAttachment
                    | vk::ImageUsageFlagBits::TransferSrc,
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;

        Ok(Self { color })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.color.destroy(gpu);
    }
}

//
// Output
//

struct OutputCreateInfo {}

struct Output {
    buffer: resource::Buffer<u32>,
}

impl GpuResource for Output {
    type CreateInfo<'a> = OutputCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = resource::Buffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE as _,
                usage: vk::BufferUsageFlagBits::TransferDst.into(),
                property_flags: vk::MemoryPropertyFlagBits::HostVisible.into(),
            },
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.buffer.destroy(gpu);
    }
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
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd, vk::PipelineStageFlagBits2::None.into());

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
                src_stage_mask: vk::PipelineStageFlagBits2::None.into(),
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                dst_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
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
                resolve_mode: vk::ResolveModeFlagBits::None,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: DEFAULT_RENDER_TARGET_CLEAR_COLOR,
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

        device.cmd_set_cull_mode(cmd, vk::CullModeFlagBits::Back.into());
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
    shaders.shader.bind(gpu, cmd);

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
                src_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                src_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::Copy.into(),
                dst_access_mask: vk::AccessFlagBits2::TransferRead.into(),
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
    queries.end(gpu, cmd, vk::PipelineStageFlagBits2::AllTransfer.into());

    // End command buffer.
    commands.end(gpu)?;

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
                stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
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
                flags: vk::SemaphoreWaitFlagBits::Any.into(),
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
