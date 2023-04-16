#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::wildcard_imports,
    clippy::too_many_arguments
)]

use std::{
    borrow::Cow,
    ffi::CStr,
    mem::{size_of, zeroed, MaybeUninit},
    ptr::{addr_of, addr_of_mut, null, null_mut},
    time::Instant,
};

use anyhow::{Context, Result};
use gpu::Gpu;
use log::{info, log, warn};
use vulk::vk;

mod gpu;
mod resource;
mod shader;

fn main() -> Result<()> {
    // Timing.
    let start_time = Instant::now();

    // Logging.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .try_init()?;

    // Vulkan.
    unsafe { vulkan()? };

    // Execution time.
    info!(
        "Execution took {} seconds",
        start_time.elapsed().as_secs_f64()
    );

    Ok(())
}

unsafe fn vulkan() -> Result<()> {
    // Create.
    let gpu = Gpu::create()?;
    let commands = &create_commands(&gpu)?;
    let compute_buffer = &create_compute_buffer(&gpu)?;
    let indirect_buffer = &create_indirect_buffer(&gpu)?;
    let descriptors = &create_descriptors(&gpu, compute_buffer, indirect_buffer)?;
    let (indirect_shader, compute_shader) = create_shaders(&gpu, compute_buffer, descriptors)?;

    // Execute.
    execute(
        &gpu,
        commands,
        indirect_shader,
        compute_shader,
        compute_buffer,
        indirect_buffer,
        descriptors,
    )?;

    // Destroy.
    {
        let Gpu { device, .. } = &gpu;
        device.destroy_shader_ext(compute_shader);
        device.destroy_shader_ext(indirect_shader);
        device.destroy_descriptor_set_layout(descriptors.set_layout);
        device.destroy_pipeline_layout(descriptors.pipeline_layout);
        compute_buffer.destroy(device);
        indirect_buffer.destroy(device);
        descriptors.buffer.destroy(device);
        device.destroy_semaphore(commands.semaphore);
        device.free_command_buffers(
            commands.command_pool,
            1,
            addr_of!(commands.command_buffer).cast(),
        );
        device.destroy_command_pool(commands.command_pool);
    }
    gpu.destroy();

    Ok(())
}

struct Commands {
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    semaphore: vk::Semaphore,
}

unsafe fn create_commands(
    Gpu {
        device,
        queue_family,
        ..
    }: &Gpu,
) -> Result<Commands> {
    // Command pool.
    let command_pool = {
        let command_pool = device.create_command_pool(
            &(vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::CommandPoolCreateInfo,
                p_next: null(),
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: queue_family.index,
            }),
        )?;
        device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;
        command_pool
    };

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
        device
            .allocate_command_buffers(&command_buffer_allocate_info, command_buffer.as_mut_ptr())?;
        command_buffer.assume_init()
    };

    // Semaphore
    let semaphore = {
        let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
            s_type: vk::StructureType::SemaphoreTypeCreateInfo,
            p_next: null(),
            semaphore_type: vk::SemaphoreType::Timeline,
            initial_value: 0,
        };
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: addr_of!(semaphore_type_create_info).cast(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };
        device.create_semaphore(addr_of!(semaphore_create_info).cast())?
    };

    Ok(Commands {
        command_pool,
        command_buffer,
        semaphore,
    })
}

type ComputeBuffer = resource::Buffer<u32>;

unsafe fn create_compute_buffer(gpu: &Gpu) -> Result<ComputeBuffer> {
    let element_count = 8;
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = ComputeBuffer::create(gpu, element_count, usage, flags)?;
    Ok(buffer)
}

#[repr(C)]
#[derive(Debug)]
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
        let descriptor_address_info_ext = vk::DescriptorAddressInfoEXT {
            s_type: vk::StructureType::DescriptorAddressInfoEXT,
            p_next: null_mut(),
            address: device_address,
            range: byte_size as _,
            format: vk::Format::Undefined,
        };
        let descriptor_get_info_ext = vk::DescriptorGetInfoEXT {
            s_type: vk::StructureType::DescriptorGetInfoEXT,
            p_next: null(),
            ty: vk::DescriptorType::StorageBuffer,
            data: vk::DescriptorDataEXT {
                p_storage_buffer: addr_of!(descriptor_address_info_ext).cast(),
            },
        };
        device.get_descriptor_ext(
            &descriptor_get_info_ext,
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

unsafe fn create_shaders(
    Gpu { device, .. }: &Gpu,
    compute_buffer: &ComputeBuffer,
    descriptors: &Descriptors,
) -> Result<(vk::ShaderEXT, vk::ShaderEXT)> {
    // Shader compiler
    let compiler = shader::Compiler::new()?;
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

unsafe fn execute(
    Gpu { device, queue, .. }: &Gpu,
    commands: &Commands,
    indirect_shader: vk::ShaderEXT,
    compute_shader: vk::ShaderEXT,
    compute_buffer: &ComputeBuffer,
    indirect_buffer: &IndirectBuffer,
    descriptors: &Descriptors,
) -> Result<()> {
    // Begin command buffer.
    let command_buffer = commands.command_buffer;
    device.begin_command_buffer(
        command_buffer,
        &(vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: null(),
        }),
    )?;

    // Descriptors.
    {
        let descriptor_buffer_binding_info_ext = vk::DescriptorBufferBindingInfoEXT {
            s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
            p_next: null_mut(),
            address: descriptors.buffer.device_address,
            usage: vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER,
        };
        let binding_infos = [descriptor_buffer_binding_info_ext];
        device.cmd_bind_descriptor_buffers_ext(
            command_buffer,
            binding_infos.len() as _,
            binding_infos.as_ptr(),
        );
        let buffer_indices = [0];
        let offsets = [0];
        device.cmd_set_descriptor_buffer_offsets_ext(
            command_buffer,
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
            command_buffer,
            stages.len() as _,
            stages.as_ptr(),
            addr_of!(indirect_shader),
        );
        device.cmd_dispatch(command_buffer, 1, 1, 1);
    }

    // Synchronize.
    {
        let memory_barrier2 = vk::MemoryBarrier2 {
            s_type: vk::StructureType::MemoryBarrier2,
            p_next: null(),
            src_stage_mask: vk::PipelineStageFlags2::COMPUTE_SHADER,
            src_access_mask: vk::AccessFlags2::SHADER_WRITE,
            dst_stage_mask: vk::PipelineStageFlags2::DRAW_INDIRECT,
            dst_access_mask: vk::AccessFlags2::INDIRECT_COMMAND_READ,
        };
        let dependency_info = vk::DependencyInfo {
            s_type: vk::StructureType::DependencyInfo,
            p_next: null(),
            dependency_flags: vk::DependencyFlags::empty(),
            memory_barrier_count: 1,
            p_memory_barriers: addr_of!(memory_barrier2).cast(),
            buffer_memory_barrier_count: 0,
            p_buffer_memory_barriers: null(),
            image_memory_barrier_count: 0,
            p_image_memory_barriers: null(),
        };
        device.cmd_pipeline_barrier2(command_buffer, &dependency_info);
    }

    // Dispatch compute shader.
    {
        let stages = [vk::ShaderStageFlagBits::COMPUTE];
        device.cmd_bind_shaders_ext(
            command_buffer,
            stages.len() as _,
            stages.as_ptr(),
            addr_of!(compute_shader),
        );
        device.cmd_dispatch_indirect(command_buffer, indirect_buffer.handle, 0);
    }

    // End command buffer.
    device.end_command_buffer(command_buffer)?;

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
                command_buffer,
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
        let semaphore_wait_info = vk::SemaphoreWaitInfo {
            s_type: vk::StructureType::SemaphoreWaitInfo,
            p_next: null(),
            flags: vk::SemaphoreWaitFlags::ANY,
            semaphore_count: semaphores.len() as _,
            p_semaphores: semaphores.as_ptr(),
            p_values: values.as_ptr(),
        };
        device.wait_semaphores(&semaphore_wait_info, u64::MAX)?;
    }

    // Validate.
    #[allow(clippy::cast_ptr_alignment)]
    let p_data = std::slice::from_raw_parts(indirect_buffer.ptr, indirect_buffer.element_count);
    info!("indirect_buffer={:?}", &p_data);
    let p_data = std::slice::from_raw_parts(compute_buffer.ptr, compute_buffer.element_count);
    info!("compute_buffer={:?}", &p_data);

    Ok(())
}
