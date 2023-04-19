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
        let commands = command::Commands::create(gpu)?;
        let queries = query::Queries::create(gpu)?;
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
        device.destroy_descriptor_set_layout(descriptors.set_layout);
        device.destroy_pipeline_layout(descriptors.pipeline_layout);
        compute_buffer.destroy(gpu);
        indirect_buffer.destroy(gpu);
        descriptors.buffer.destroy(gpu);
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
    let element_count = 8;
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = ComputeBuffer::create(gpu, element_count, usage, flags)?;
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
    let element_count = 1;
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = IndirectBuffer::create(gpu, element_count, usage, flags)?;
    Ok(buffer)
}

//
// Descriptors
//

type DescriptorBuffer = resource::Buffer<u8>;

struct Descriptors {
    set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    buffer: DescriptorBuffer,
}

unsafe fn create_descriptors(
    gpu @ Gpu { device, .. }: &Gpu,
    compute_buffer: &ComputeBuffer,
    indirect_buffer: &IndirectBuffer,
) -> Result<Descriptors> {
    // Descriptor set layout.
    let bindings = [
        vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::StorageBuffer,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            p_immutable_samplers: null(),
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::StorageBuffer,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            p_immutable_samplers: null(),
        },
    ];
    let descriptor_set_layout = device.create_descriptor_set_layout(
        &(vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
            p_next: null(),
            flags: vk::DescriptorSetLayoutCreateFlags::DESCRIPTOR_BUFFER_EXT,
            binding_count: bindings.len() as _,
            p_bindings: bindings.as_ptr(),
        }),
    )?;

    // Descriptor buffer.
    let buffer = {
        let buffer_size = device.get_descriptor_set_layout_size_ext(descriptor_set_layout);
        let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
        let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        DescriptorBuffer::create(gpu, buffer_size as _, usage, flags)?
    };

    // Descriptors.
    {
        let mut dst_offset = 0;
        std::ptr::copy_nonoverlapping(
            indirect_buffer.descriptor.as_ptr(),
            buffer.ptr.add(dst_offset),
            indirect_buffer.descriptor.byte_size(),
        );
        dst_offset += indirect_buffer.descriptor.byte_size();
        std::ptr::copy_nonoverlapping(
            compute_buffer.descriptor.as_ptr(),
            buffer.ptr.add(dst_offset),
            compute_buffer.descriptor.byte_size(),
        );
    }

    // Pipeline layout.
    let set_layouts = [descriptor_set_layout];
    let pipeline_layout = device.create_pipeline_layout(
        &(vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PipelineLayoutCreateInfo,
            p_next: null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: set_layouts.len() as _,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
        }),
    )?;

    Ok(Descriptors {
        set_layout: descriptor_set_layout,
        pipeline_layout,
        buffer,
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

            layout (local_size_x = 1) in;

            struct IndirectCommand {
                uint x;
                uint y;
                uint z;
            };

            layout (set = 0, binding = 0) buffer IndirectCommands {
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

            layout (local_size_x_id = 0) in;

            layout (set = 0, binding = 1) buffer Values {
                uint data[];
            } values;

            void main() {
                uint id = gl_GlobalInvocationID.x;
                values.data[id] = 1 + id;
            }
        "#,
    )?;

    // Create shaders.
    let indirect_shader =
        shader::Shader::create(gpu, &[indirect_spirv], &[descriptors.set_layout], &[], None)?;
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
        &[compute_spirv],
        &[descriptors.set_layout],
        &[],
        Some(&specialization_info),
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

    // Descriptors.
    {
        let binding_infos = [vk::DescriptorBufferBindingInfoEXT {
            s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
            p_next: null_mut(),
            address: descriptors.buffer.device_address,
            usage: vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER,
        }];
        device.cmd_bind_descriptor_buffers_ext(
            cmd,
            binding_infos.len() as _,
            binding_infos.as_ptr(),
        );
        let buffer_indices = [0];
        let offsets = [0];
        device.cmd_set_descriptor_buffer_offsets_ext(
            cmd,
            vk::PipelineBindPoint::Compute,
            descriptors.pipeline_layout,
            0,
            buffer_indices.len() as _,
            buffer_indices.as_ptr(),
            offsets.as_ptr(),
        );
    }

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
                    src_stage_mask: vk::PipelineStageFlags2::COMPUTE_SHADER,
                    src_access_mask: vk::AccessFlags2::SHADER_WRITE,
                    dst_stage_mask: vk::PipelineStageFlags2::DRAW_INDIRECT,
                    dst_access_mask: vk::AccessFlags2::INDIRECT_COMMAND_READ,
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
                stage_mask: vk::PipelineStageFlags2::COMPUTE_SHADER,
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
