use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    compute_buffer: ComputeBuffer,
    indirect_buffer: IndirectBuffer,
    descriptors: Descriptors,
    indirect_shader: shader::Shader,
    compute_shader: shader::Shader,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "compute";

    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
        let compute_buffer = create_compute_buffer(gpu)?;
        let indirect_buffer = create_indirect_buffer(gpu)?;
        let descriptors = create_descriptors(gpu, &compute_buffer, &indirect_buffer)?;
        let (indirect_shader, compute_shader) = create_shaders(gpu, &compute_buffer, &descriptors)?;

        Ok(Self {
            commands,
            queries,
            compute_buffer,
            indirect_buffer,
            descriptors,
            indirect_shader,
            compute_shader,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &Self) -> Result<()> {
        dispatch(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        let Gpu { device, .. } = &gpu;
        let Self {
            commands,
            queries,
            compute_buffer,
            indirect_buffer,
            descriptors,
            indirect_shader,
            compute_shader,
        } = state;
        compute_shader.destroy(gpu);
        indirect_shader.destroy(gpu);
        device.destroy_pipeline_layout(descriptors.pipeline_layout);
        compute_buffer.destroy(gpu);
        indirect_buffer.destroy(gpu);
        descriptors.storage.destroy(gpu);
        queries.destroy(gpu);
        commands.destroy(gpu);
        Ok(())
    }
}

//
// Buffers
//

type ComputeBuffer = resource::Buffer<u32>;

unsafe fn create_compute_buffer(gpu: &Gpu) -> Result<ComputeBuffer> {
    let buffer = ComputeBuffer::create(
        gpu,
        &resource::BufferCreateInfo {
            element_count: 8,
            usage: vk::BufferUsageFlagBits::StorageBuffer.into(),
            property_flags: vk::MemoryPropertyFlagBits::HostVisible
                | vk::MemoryPropertyFlagBits::HostCoherent,
        },
    )?;
    Ok(buffer)
}

#[repr(C)]
#[derive(Debug, serde::Serialize)]
struct IndirectDispatch {
    x: u32,
    y: u32,
    z: u32,
}

type IndirectBuffer = resource::Buffer<IndirectDispatch>;

unsafe fn create_indirect_buffer(gpu: &Gpu) -> Result<IndirectBuffer> {
    let buffer = IndirectBuffer::create(
        gpu,
        &resource::BufferCreateInfo {
            element_count: 1,
            usage: vk::BufferUsageFlagBits::StorageBuffer | vk::BufferUsageFlagBits::IndirectBuffer,
            property_flags: vk::MemoryPropertyFlagBits::HostVisible
                | vk::MemoryPropertyFlagBits::HostCoherent,
        },
    )?;
    Ok(buffer)
}

//
// Descriptors
//

struct Descriptors {
    storage: descriptor::DescriptorStorage,
    pipeline_layout: vk::PipelineLayout,
}

unsafe fn create_descriptors(
    gpu @ Gpu { device, .. }: &Gpu,
    compute_buffer: &ComputeBuffer,
    indirect_buffer: &IndirectBuffer,
) -> Result<Descriptors> {
    // Descriptor storage.
    let stage_flags = vk::ShaderStageFlagBits::Compute.into();
    let storage = descriptor::DescriptorStorage::create(
        gpu,
        &descriptor::DescriptorStorageCreateInfo {
            bindings: &[
                descriptor::DescriptorStorageBinding {
                    descriptor_type: vk::DescriptorType::StorageBuffer,
                    stage_flags,
                    descriptors: &[indirect_buffer.descriptor],
                },
                descriptor::DescriptorStorageBinding {
                    descriptor_type: vk::DescriptorType::StorageBuffer,
                    stage_flags,
                    descriptors: &[compute_buffer.descriptor],
                },
            ],
        },
    )?;

    // Pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &(vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PipelineLayoutCreateInfo,
            p_next: null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 1,
            p_set_layouts: &storage.set_layout(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
        }),
    )?;

    Ok(Descriptors {
        storage,
        pipeline_layout,
    })
}

//
// Shaders
//

unsafe fn create_shaders(
    gpu: &Gpu,
    compute_buffer: &ComputeBuffer,
    descriptors: &Descriptors,
) -> Result<(shader::Shader, shader::Shader)> {
    // Shader compiler
    let compiler = shader::Compiler::new()?;

    // Shaders.
    let indirect_spirv = compiler.compile(
        shader::ShaderType::Compute,
        r#"
            #version 460 core
            #extension GL_EXT_scalar_block_layout : require

            layout(local_size_x = 1) in;

            struct IndirectCommand {
                uint x;
                uint y;
                uint z;
            };

            layout(scalar, binding = 0) buffer IndirectCommands {
                IndirectCommand data[];
            } indirect_commands;

            void main() {
                IndirectCommand command;
                command.x = 1;
                command.y = 1;
                command.z = 1;
                indirect_commands.data[0] = command;
            }
        "#,
    )?;
    let compute_spirv = compiler.compile(
        shader::ShaderType::Compute,
        r#"
            #version 460 core
            #extension GL_EXT_scalar_block_layout : require

            layout(local_size_x_id = 0) in;

            layout(scalar, binding = 1) buffer Values {
                uint data[];
            } values;

            void main() {
                uint id = gl_GlobalInvocationID.x;
                values.data[id] = 1 + id;
            }
        "#,
    )?;

    // Create shaders.
    let indirect_shader = shader::Shader::create(
        gpu,
        &shader::ShaderCreateInfo {
            spirvs: &[indirect_spirv],
            set_layouts: &[descriptors.storage.set_layout()],
            push_constant_ranges: &[],
            specialization_info: None,
        },
    )?;
    let specialization_map_entry = vk::SpecializationMapEntry {
        constant_id: 0,
        offset: 0,
        size: size_of::<u32>(),
    };
    let data = compute_buffer.element_count as u32;
    let specialization_info = vk::SpecializationInfo {
        map_entry_count: 1,
        p_map_entries: addr_of!(specialization_map_entry).cast(),
        data_size: size_of::<u32>(),
        p_data: addr_of!(data).cast(),
    };
    let compute_shader = shader::Shader::create(
        gpu,
        &shader::ShaderCreateInfo {
            spirvs: &[compute_spirv],
            set_layouts: &[descriptors.storage.set_layout()],
            push_constant_ranges: &[],
            specialization_info: Some(&specialization_info),
        },
    )?;

    Ok((indirect_shader, compute_shader))
}

//
// Execute
//

unsafe fn dispatch(
    gpu @ Gpu { device, queue, .. }: &Gpu,
    Demo {
        commands,
        queries,
        indirect_shader,
        compute_shader,
        compute_buffer,
        indirect_buffer,
        descriptors,
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd);

    // Bind descriptors.
    descriptors.storage.bind(gpu, cmd);
    descriptors.storage.set_offsets(
        gpu,
        cmd,
        vk::PipelineBindPoint::Compute,
        descriptors.pipeline_layout,
    );

    // Dispatch indirect shader.
    {
        indirect_shader.bind(gpu, cmd);
        device.cmd_dispatch(cmd, 1, 1, 1);
    }

    // Synchronize.
    {
        device.cmd_pipeline_barrier2(
            cmd,
            &(vk::DependencyInfo {
                s_type: vk::StructureType::DependencyInfo,
                p_next: null(),
                dependency_flags: vk::DependencyFlags::empty(),
                memory_barrier_count: 1,
                p_memory_barriers: &(vk::MemoryBarrier2 {
                    s_type: vk::StructureType::MemoryBarrier2,
                    p_next: null(),
                    src_stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
                    src_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                    dst_stage_mask: vk::PipelineStageFlagBits2::DrawIndirect.into(),
                    dst_access_mask: vk::AccessFlagBits2::IndirectCommandRead.into(),
                }),
                buffer_memory_barrier_count: 0,
                p_buffer_memory_barriers: null(),
                image_memory_barrier_count: 0,
                p_image_memory_barriers: null(),
            }),
        );
    }

    // Dispatch compute shader.
    {
        compute_shader.bind(gpu, cmd);
        device.cmd_dispatch_indirect(cmd, indirect_buffer.buffer, 0);
    }

    // End queries.
    queries.end(gpu, cmd);

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
                stage_mask: vk::PipelineStageFlagBits2::ComputeShader.into(),
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
        info!("Compute took {:?}", queries.elapsed(gpu)?);
        info!("Compute statistics: {:?}", queries.statistics(gpu)?);
    }

    // Write output.
    {
        #[derive(serde::Serialize)]
        struct Output<'a> {
            indirect: &'a [IndirectDispatch],
            compute: &'a [u32],
        }
        #[allow(clippy::cast_ptr_alignment)]
        let indirect =
            std::slice::from_raw_parts(indirect_buffer.ptr, indirect_buffer.element_count);
        let compute = std::slice::from_raw_parts(compute_buffer.ptr, compute_buffer.element_count);

        let output = ron::ser::to_string_pretty(
            &Output { indirect, compute },
            ron::ser::PrettyConfig::default(),
        )?;
        let output_path = work_dir_or_create()?.join(format!("{demo_name}.ron"));
        std::fs::write(&output_path, output)?;
        info!("Wrote output to {}", output_path.display());
    }

    Ok(())
}
