use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    indirect_buffer: IndirectBuffer,
    compute_image: ComputeImage,
    descriptors: Descriptors,
    shaders: Shaders,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "compute";

    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
        let indirect_buffer = IndirectBuffer::create(gpu, &IndirectBufferCreateInfo {})?;
        let compute_image = ComputeImage::create(gpu, &ComputeImageCreateInfo {})?;
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
        let output = Output::create(gpu, &OutputCreateInfo {})?;

        Ok(Self {
            commands,
            queries,
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
        state.output.destroy(gpu);
        state.shaders.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.compute_image.destroy(gpu);
        state.indirect_buffer.destroy(gpu);
        state.queries.destroy(gpu);
        state.commands.destroy(gpu);
        Ok(())
    }
}

//
// Indirect buffer
//

struct IndirectBufferCreateInfo {}

struct IndirectBuffer {
    buffer: vkx::BufferDedicatedResource,
}

impl GpuResource for IndirectBuffer {
    type CreateInfo<'a> = IndirectBufferCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
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

struct ComputeImageCreateInfo {}

struct ComputeImage {
    image: vkx::ImageDedicatedResource,
}

impl GpuResource for ComputeImage {
    type CreateInfo<'a> = ComputeImageCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
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
            vk::MemoryPropertyFlagBits::DeviceLocal.into(),
        )?;
        Ok(Self { image })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.image.destroy(&gpu.device);
    }
}

//
// Output
//

struct OutputCreateInfo {}

struct Output {
    buffer: vkx::BufferDedicatedTransfer,
}

impl GpuResource for Output {
    type CreateInfo<'a> = OutputCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = vkx::BufferDedicatedTransfer::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE,
                vk::BufferUsageFlagBits::TransferDst.into(),
            ),
            vk::MemoryPropertyFlagBits::HostVisible.into(),
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.buffer.destroy(&gpu.device);
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
    gpu @ Gpu { device, .. }: &Gpu,
    Demo {
        commands,
        queries,
        indirect_buffer,
        compute_image,
        descriptors,
        shaders,
        output,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd, vk::PipelineStageFlagBits2::None.into());

    // Bind descriptors.
    descriptors.storage.bind(device, cmd);
    descriptors
        .storage
        .set_offsets(&gpu.device, cmd, vk::PipelineBindPoint::Compute);

    // Dispatch indirect shader.
    {
        shaders.indirect.bind(&gpu.device, cmd);
        device.cmd_dispatch(cmd, 1, 1, 1);
    }

    // Synchronize.
    {
        device.cmd_pipeline_barrier2(
            cmd,
            &vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 1,
                p_memory_barriers: &vk::MemoryBarrier2 {
                    s_type: vk::StructureType::MemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
                    src_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::DrawIndirect.into(),
                    dst_access_mask: vk::AccessFlagBits2::IndirectCommandRead.into(),
                },
                buffer_memory_barrier_count: 0,
                p_buffer_memory_barriers: null(),
                image_memory_barrier_count: 0,
                p_image_memory_barriers: null(),
            },
        );
    }

    // Transition compute image.
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
                dst_stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
                dst_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::General,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: compute_image.image.image_handle(),
                subresource_range: compute_image.image.subresource_range(),
            },
        },
    );

    // Dispatch compute shader.
    {
        shaders.compute.bind(&gpu.device, cmd);
        device.cmd_dispatch_indirect(cmd, indirect_buffer.buffer.buffer_handle(), 0);
    }

    // Transition compute image.
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
                src_stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
                src_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::Copy.into(),
                dst_access_mask: vk::AccessFlagBits2::TransferRead.into(),
                old_layout: vk::ImageLayout::General,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: compute_image.image.image_handle(),
                subresource_range: compute_image.image.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: compute_image.image.image_handle(),
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer_handle(),
            region_count: 1,
            p_regions: &vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: 0,
                buffer_image_height: 0,
                image_subresource: compute_image.image.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: compute_image.image.extent_3d(),
            },
        },
    );

    // End queries.
    queries.end(gpu, cmd, vk::PipelineStageFlagBits2::ComputeShader.into());

    // End command buffer.
    commands.end(gpu)?;

    // Queue submit.
    device.queue_submit2(
        device.queue,
        1,
        &vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: 0,
            p_wait_semaphore_infos: null(),
            command_buffer_info_count: 1,
            p_command_buffer_infos: &vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer: cmd,
                device_mask: 0,
            },
            signal_semaphore_info_count: 1,
            p_signal_semaphore_infos: &vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: commands.semaphore.handle(),
                value: 1,
                stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
                device_index: 0,
            },
        },
        vk::Fence::null(),
    )?;

    // Wait for semaphore.
    commands.semaphore.wait(device, 1, u64::MAX)?;

    // Query results.
    {
        info!("Compute took {:?}", queries.elapsed(gpu)?);
        info!("Compute statistics: {:?}", queries.statistics(gpu)?);
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
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = compute_image.image.width();
        let height = compute_image.image.height();
        let pixels_byte_size = compute_image.image.byte_size();
        let mut pixels = vec![0_u8; pixels_byte_size as _];
        pixels.copy_from_slice(output.buffer.memory().as_slice(pixels_byte_size as _));
        let image = RgbaImage::from_raw(width, height, pixels)
            .context("Creating image from output buffer")?;
        let image_path = work_dir_or_create()?.join(format!("{demo_name}.png"));
        image.save_with_format(&image_path, ImageFormat::Png)?;
        info!("Wrote image to {}", image_path.display());
    }

    Ok(())
}
