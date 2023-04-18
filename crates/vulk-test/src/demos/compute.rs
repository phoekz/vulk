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
    indirect_shader: vk::ShaderEXT,
    compute_shader: vk::ShaderEXT,
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
        device.destroy_shader_ext(compute_shader);
        device.destroy_shader_ext(indirect_shader);
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
    let usage = vk::BufferUsageFlags::INDIRECT_BUFFER;
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
    gpu @ Gpu {
        device,
        physical_device,
        ..
    }: &Gpu,
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
    let buffer_size = device.get_descriptor_set_layout_size_ext(descriptor_set_layout);
    info!("Descriptor buffer size={buffer_size}");
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = DescriptorBuffer::create(gpu, buffer_size as _, usage, flags)?;

    // Descriptors.
    let storage_buffer_descriptor_size = physical_device
        .descriptor_buffer_properties_ext
        .storage_buffer_descriptor_size;
    for (buffer_index, (device_address, byte_size)) in [
        (indirect_buffer.device_address, indirect_buffer.byte_size()),
        (compute_buffer.device_address, compute_buffer.byte_size()),
    ]
    .into_iter()
    .enumerate()
    {
        device.get_descriptor_ext(
            &(vk::DescriptorGetInfoEXT {
                s_type: vk::StructureType::DescriptorGetInfoEXT,
                p_next: null(),
                ty: vk::DescriptorType::StorageBuffer,
                data: vk::DescriptorDataEXT {
                    p_storage_buffer: &(vk::DescriptorAddressInfoEXT {
                        s_type: vk::StructureType::DescriptorAddressInfoEXT,
                        p_next: null_mut(),
                        address: device_address,
                        range: byte_size as _,
                        format: vk::Format::Undefined,
                    }),
                },
            }),
            storage_buffer_descriptor_size,
            buffer
                .ptr
                .add(buffer_index * storage_buffer_descriptor_size)
                .cast(),
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
    Gpu { device, .. }: &Gpu,
    compute_buffer: &ComputeBuffer,
    descriptors: &Descriptors,
) -> Result<(vk::ShaderEXT, vk::ShaderEXT)> {
    // Shader compiler
    let compiler = shader::Compiler::new()?;

    // Shaders.
    let indirect_spirv = compiler.compile(
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
        shader::ShaderType::Compute,
    )?;
    let compute_spirv = compiler.compile(
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
        shader::ShaderType::Compute,
    )?;

    // Indirect shader.
    let indirect_shader = {
        let set_layouts = [descriptors.set_layout];
        let shader_create_info_ext = vk::ShaderCreateInfoEXT {
            s_type: vk::StructureType::ShaderCreateInfoEXT,
            p_next: null(),
            flags: vk::ShaderCreateFlagsEXT::empty(),
            stage: vk::ShaderStageFlagBits::COMPUTE,
            next_stage: vk::ShaderStageFlags::empty(),
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: indirect_spirv.len(),
            p_code: indirect_spirv.as_ptr().cast(),
            p_name: b"main\0".as_ptr().cast(),
            set_layout_count: set_layouts.len() as _,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
            p_specialization_info: null(),
        };
        let mut shader = MaybeUninit::uninit();
        device.create_shaders_ext(
            1,
            addr_of!(shader_create_info_ext).cast(),
            shader.as_mut_ptr(),
        )?;
        shader.assume_init()
    };

    // Compute shader.
    let compute_shader = {
        let set_layouts = [descriptors.set_layout];
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
        let shader_create_info_ext = vk::ShaderCreateInfoEXT {
            s_type: vk::StructureType::ShaderCreateInfoEXT,
            p_next: null(),
            flags: vk::ShaderCreateFlagsEXT::empty(),
            stage: vk::ShaderStageFlagBits::COMPUTE,
            next_stage: vk::ShaderStageFlags::empty(),
            code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
            code_size: compute_spirv.len(),
            p_code: compute_spirv.as_ptr().cast(),
            p_name: b"main\0".as_ptr().cast(),
            set_layout_count: set_layouts.len() as _,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
            p_specialization_info: &specialization_info,
        };
        let mut shader = MaybeUninit::uninit();
        device.create_shaders_ext(
            1,
            addr_of!(shader_create_info_ext).cast(),
            shader.as_mut_ptr(),
        )?;
        shader.assume_init()
    };

    Ok((indirect_shader, compute_shader))
}

//
// Execute
//

unsafe fn dispatch(
    gpu @ Gpu {
        device,
        queue,
        physical_device,
        ..
    }: &Gpu,
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
        let stages = [vk::ShaderStageFlagBits::COMPUTE];
        device.cmd_bind_shaders_ext(
            cmd,
            stages.len() as _,
            stages.as_ptr(),
            addr_of!(*indirect_shader),
        );
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
        let stages = [vk::ShaderStageFlagBits::COMPUTE];
        device.cmd_bind_shaders_ext(
            cmd,
            stages.len() as _,
            stages.as_ptr(),
            addr_of!(*compute_shader),
        );
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
