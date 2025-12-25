use super::*;

//
// Demo
//

pub struct Demo {
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    indirect_buffer: IndirectBuffer,
    compute_image: ComputeImage,
    descriptors: Descriptors,
    shaders: Shaders,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "compute";

    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let indirect_buffer = IndirectBuffer::create(gpu, &())?;
        let compute_image = ComputeImage::create(gpu, &())?;
        let descriptors = Descriptors::create(
            gpu,
            &DescriptorsCreateInfo {
                indirect_buffer: &indirect_buffer,
                compute_image: &compute_image,
            },
        )?;
        let shaders = Shaders::create(
            gpu,
            &ShadersCreateInfo {
                descriptors: &descriptors,
            },
        )?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;

        Ok(Self {
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
            indirect_buffer,
            compute_image,
            descriptors,
            shaders,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        dispatch(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(&gpu.device);
        state.shaders.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.compute_image.destroy(gpu);
        state.indirect_buffer.destroy(gpu);
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
        Ok(())
    }
}

//
// Indirect buffer
//

struct IndirectBuffer {
    buffer: vkx::BufferDedicatedResource,
}

impl GpuResource for IndirectBuffer {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, (): &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                size_of::<vk::DispatchIndirectCommand>() as _,
                vk::BufferUsageFlagBits::StorageBuffer | vk::BufferUsageFlagBits::IndirectBuffer,
            ),
            vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.buffer.destroy(&gpu.device);
    }
}

//
// Compute image
//

struct ComputeImage {
    image: vkx::ImageDedicatedResource,
}

impl GpuResource for ComputeImage {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, (): &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let image = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                vk::ImageUsageFlagBits::Storage | vk::ImageUsageFlagBits::TransferSrc,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        Ok(Self { image })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.image.destroy(&gpu.device);
    }
}

//
// Descriptors
//

struct DescriptorsCreateInfo<'a> {
    indirect_buffer: &'a IndirectBuffer,
    compute_image: &'a ComputeImage,
}

struct Descriptors {
    storage: vkx::DescriptorStorage,
}

impl GpuResource for Descriptors {
    type CreateInfo<'a> = DescriptorsCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Descriptor storage.
        let stages = vk::ShaderStageFlagBits::Compute.into();
        let storage = vkx::DescriptorStorage::create(
            &gpu.physical_device,
            &gpu.device,
            &[
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageBuffer,
                    stages,
                    descriptors: &[create_info.indirect_buffer.buffer.descriptor()],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageImage,
                    stages,
                    descriptors: &[create_info.compute_image.image.descriptor()],
                },
            ],
            None,
        )?;

        Ok(Self { storage })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.storage.destroy(&gpu.device);
    }
}

//
// Shaders
//

struct ShadersCreateInfo<'a> {
    descriptors: &'a Descriptors,
}

struct Shaders {
    indirect: vkx::Shader,
    compute: vkx::Shader,
}

impl GpuResource for Shaders {
    type CreateInfo<'a> = ShadersCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Shader compiler
        let mut compiler = vkx::ShaderCompiler::new()?;

        // Includes.
        compiler.include(
            "common.glsl",
            r#"
                #extension GL_EXT_scalar_block_layout : require

                struct IndirectCommand {
                    uint x;
                    uint y;
                    uint z;
                };
                layout(scalar, binding = 0) buffer IndirectCommands { IndirectCommand indirect_commands[]; };
                layout(binding = 1, rgba8) uniform image2D compute_image;
            "#,
        );

        // Indirect shader  .
        let indirect_spirv = compiler.compile(
            vkx::ShaderType::Compute,
            "indirect_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(local_size_x = 1) in;

                void main() {
                    IndirectCommand command;
                    command.x = 32;
                    command.y = 32;
                    command.z = 1;
                    indirect_commands[0] = command;
                }
            "#,
        )?;
        let indirect = vkx::Shader::create(
            &gpu.device,
            &vkx::ShaderCreateInfo {
                shader_binaries: &[indirect_spirv],
                set_layouts: create_info.descriptors.storage.set_layouts(),
                push_constant_ranges: &[],
                specialization_info: None,
            },
        )?;

        // Compute shader.
        let compute_spirv = compiler.compile(
            vkx::ShaderType::Compute,
            "compute_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(local_size_x = 8, local_size_y = 8) in;

                void main() {
                    uint x = gl_GlobalInvocationID.x;
                    uint y = gl_GlobalInvocationID.y;
                    vec2 p = vec2(2.0 * ((float(x) + 0.5) / 256.0 - 0.5), 2.0 * ((float(y) + 0.5) / 256.0 - 0.5));
                    float len = 1.0 - min(1.0, 2.0 * length(p));
                    imageStore(compute_image, ivec2(x, y), vec4(0.2 + len, 0.2 + 0.5 * len, 0.2, 1.0));
                }
            "#,
        )?;
        let compute = vkx::Shader::create(
            &gpu.device,
            &vkx::ShaderCreateInfo {
                shader_binaries: &[compute_spirv],
                set_layouts: create_info.descriptors.storage.set_layouts(),
                push_constant_ranges: &[],
                specialization_info: None,
            },
        )?;

        Ok(Self { indirect, compute })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.indirect.destroy(&gpu.device);
        self.compute.destroy(&gpu.device);
    }
}

//
// Execute
//

unsafe fn dispatch(
    Gpu { device, .. }: &Gpu,
    Demo {
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        indirect_buffer,
        compute_image,
        descriptors,
        shaders,
        output,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Record commands.
    command_buffer.begin(device)?;
    command_buffer.write_timestamp(device, timestamps, 0);
    command_buffer.begin_statistics(device, statistics);

    command_buffer.bind_descriptor_storage(
        device,
        &descriptors.storage,
        vk::PipelineBindPoint::Compute,
    );
    command_buffer.bind_shader(device, &shaders.indirect);
    command_buffer.dispatch(device, 1, 1, 1);

    command_buffer.barrier(
        device,
        vk::PipelineStageFlagBits2::ComputeShader,
        vk::AccessFlagBits2::ShaderWrite,
        vk::PipelineStageFlagBits2::DrawIndirect,
        vk::AccessFlagBits2::IndirectCommandRead,
    );
    command_buffer.image_barrier(
        device,
        &compute_image.image,
        vk::PipelineStageFlagBits2::None,
        vk::AccessFlags2::empty(),
        vk::PipelineStageFlagBits2::ComputeShader,
        vk::AccessFlagBits2::ShaderWrite,
        vk::ImageLayout::Undefined,
        vk::ImageLayout::General,
    );
    command_buffer.bind_shader(device, &shaders.compute);
    command_buffer.dispatch_indirect(device, &indirect_buffer.buffer, 0);

    command_buffer.image_barrier(
        device,
        &compute_image.image,
        vk::PipelineStageFlagBits2::ComputeShader,
        vk::AccessFlagBits2::ShaderWrite,
        vk::PipelineStageFlagBits2::Copy,
        vk::AccessFlagBits2::TransferRead,
        vk::ImageLayout::General,
        vk::ImageLayout::TransferSrcOptimal,
    );
    command_buffer.copy_image_to_buffer(device, &compute_image.image, (&output.buffer, 0));

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
        println!("Compute took: {timestamp_differences:?}");
        println!("Compute statistics: {statistics:?}");
    }

    // Write output.
    {
        #[repr(C)]
        #[derive(Debug, serde::Serialize)]
        struct DispatchIndirectCommand {
            x: u32,
            y: u32,
            z: u32,
        }

        #[derive(serde::Serialize)]
        struct Output<'a> {
            indirect: &'a [DispatchIndirectCommand],
        }
        let indirect = indirect_buffer
            .buffer
            .memory()
            .as_slice::<DispatchIndirectCommand>(1);

        let output =
            ron::ser::to_string_pretty(&Output { indirect }, ron::ser::PrettyConfig::default())?;
        let output_path = work_dir_or_create()?.join(format!("{demo_name}.ron"));
        std::fs::write(&output_path, output)?;
        info!("Wrote output to {}", output_path.display());
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
