use super::*;

//
// Demo
//

pub struct Demo {
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "triangle";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let shaders = Shaders::create(gpu, &())?;
        let render_targets = RenderTargets::create(gpu, &())?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;
        Ok(Self {
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
            shaders,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(&gpu.device);
        state.render_targets.destroy(gpu);
        state.shaders.destroy(gpu);
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
        Ok(())
    }
}

//
// Shaders
//

struct Shaders {
    shader: vkx::Shader,
}

impl GpuResource for Shaders {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Shader compiler
        let mut compiler = vkx::ShaderCompiler::new()?;

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
            vkx::ShaderType::Task,
            "task_shader",
            "main",
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
            vkx::ShaderType::Mesh,
            "mesh_shader",
            "main",
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
            vkx::ShaderType::Fragment,
            "fragment_shader",
            "main",
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

        let shader = vkx::Shader::create(
            &gpu.device,
            &vkx::ShaderCreateInfo {
                shader_binaries: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: &[],
                push_constant_ranges: &[],
                specialization_info: None,
            },
        )?;

        Ok(Self { shader })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.shader.destroy(&gpu.device);
    }
}

//
// Render targets
//

struct RenderTargets {
    color: vkx::ImageDedicatedResource,
}

impl GpuResource for RenderTargets {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let color = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                vk::ImageUsageFlagBits::InputAttachment
                    | vk::ImageUsageFlagBits::ColorAttachment
                    | vk::ImageUsageFlagBits::TransferSrc,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        Ok(Self { color })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.color.destroy(&gpu.device);
    }
}

//
// Output
//

struct Output {
    buffer: vkx::BufferDedicatedTransfer,
}

impl GpuResource for Output {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = vkx::BufferDedicatedTransfer::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE,
                vk::BufferUsageFlagBits::TransferDst,
            ),
            vk::MemoryPropertyFlagBits::HostVisible,
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.buffer.destroy(&gpu.device);
    }
}

//
// Draw
//

unsafe fn draw(
    Gpu { device, .. }: &Gpu,
    Demo {
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        shaders,
        render_targets,
        output,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Record commands.
    command_buffer.begin(device)?;
    command_buffer.write_timestamp(device, timestamps, 0);
    command_buffer.begin_statistics(device, statistics);
    command_buffer.image_barrier(
        device,
        &render_targets.color,
        vk::PipelineStageFlagBits2::None,
        vk::AccessFlags2::empty(),
        vk::PipelineStageFlagBits2::ColorAttachmentOutput,
        vk::AccessFlagBits2::ColorAttachmentWrite,
        vk::ImageLayout::Undefined,
        vk::ImageLayout::AttachmentOptimal,
    );

    command_buffer.begin_rendering(
        device,
        (&render_targets.color, DEFAULT_RENDER_TARGET_CLEAR_COLOR),
        None,
        None,
    );
    command_buffer.set_cull_mode(device, vk::CullModeFlagBits::Back);
    command_buffer.set_front_face(device, vk::FrontFace::Clockwise);
    command_buffer.set_viewport_flip_y(
        device,
        &vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: render_targets.color.width() as f32,
            height: render_targets.color.height() as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        },
    );
    command_buffer.bind_shader(device, &shaders.shader);
    command_buffer.draw_mesh_tasks(device, 1, 1, 1);
    command_buffer.end_rendering(device);

    command_buffer.image_barrier(
        device,
        &render_targets.color,
        vk::PipelineStageFlagBits2::ColorAttachmentOutput,
        vk::AccessFlagBits2::ColorAttachmentWrite,
        vk::PipelineStageFlagBits2::Copy,
        vk::AccessFlagBits2::TransferRead,
        vk::ImageLayout::AttachmentOptimal,
        vk::ImageLayout::TransferSrcOptimal,
    );
    command_buffer.copy_image_to_buffer(device, &render_targets.color, (&output.buffer, 0));
    command_buffer.end_statistics(device, statistics);
    command_buffer.write_timestamp(device, timestamps, 1);
    command_buffer.end(device)?;

    // Submit & wait.
    vkx::queue_submit(
        device,
        command_buffer,
        &[],
        &[command_buffer_done.submit_info(1, vk::PipelineStageFlagBits2::AllCommands)],
    )?;
    command_buffer_done.wait(device, 1, u64::MAX)?;

    // Query results.
    {
        let timestamp_differences = timestamps.get_differences(device)?[0];
        let statistics = statistics.get_statistics(device)?;
        println!("Rendering took: {timestamp_differences:?}");
        println!("Rendering statistics: {statistics:?}");
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
