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
use log::{info, log, warn};
use vulk::{
    loader::{DeviceFunctions, InstanceFunctions, LoaderFunctions},
    vk,
};

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
    let loader_fn = &LoaderFunctions::load()?;
    let (ref instance_fn, instance) = create_instance(loader_fn)?;
    let debug_utils_messenger = create_debug_utils_messenger(instance_fn, instance)?;
    let physical_device = &create_physical_device(instance_fn, instance)?;
    let queue_family = &create_queue_family(physical_device)?;
    let (ref device_fn, device) = create_device(instance_fn, physical_device, queue_family)?;
    let queue = create_queue(device_fn, device, queue_family);
    let commands = &create_commands(device_fn, device, queue_family)?;
    let compute_buffer = &create_compute_buffer(device_fn, physical_device, device)?;
    let indirect_buffer = &create_indirect_buffer(device_fn, physical_device, device)?;
    let descriptors = &create_descriptors(
        device_fn,
        physical_device,
        device,
        compute_buffer,
        indirect_buffer,
    )?;
    let (indirect_shader, compute_shader) =
        create_shaders(device_fn, device, compute_buffer, descriptors)?;

    // Execute.
    execute(
        device_fn,
        device,
        queue,
        commands,
        indirect_shader,
        compute_shader,
        compute_buffer,
        indirect_buffer,
        descriptors,
    )?;

    // Destroy.
    device_fn.destroy_shader_ext(compute_shader, null());
    device_fn.destroy_shader_ext(indirect_shader, null());
    device_fn.destroy_descriptor_set_layout(descriptors.set_layout, null());
    device_fn.destroy_pipeline_layout(descriptors.pipeline_layout, null());
    compute_buffer.destroy(device_fn, device);
    indirect_buffer.destroy(device_fn, device);
    descriptors.buffer.destroy(device_fn, device);
    device_fn.destroy_semaphore(commands.semaphore, null());
    device_fn.free_command_buffers(
        commands.command_pool,
        1,
        addr_of!(commands.command_buffer).cast(),
    );
    device_fn.destroy_command_pool(commands.command_pool, null());
    device_fn.destroy_device(null());
    instance_fn.destroy_debug_utils_messenger_ext(debug_utils_messenger, null());
    instance_fn.destroy_instance(null());

    Ok(())
}

unsafe extern "C" fn debug_utils_messenger_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    // Constants.
    const WARNING_BITS: u32 = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT.bits();

    // Unpack.
    let callback_data = *p_callback_data;
    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };
    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };
    let message_id_number: u32 = std::mem::transmute(callback_data.message_id_number);

    // Filter.
    if message_id_name == "Loader Message"
        && !message.starts_with("Loading layer library")
        && message_severity.bits() < WARNING_BITS
    {
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkCreateInstance-specialuse-extension-debugging"
    {
        // CreateInstance(): Attempting to enable extension VK_EXT_debug_utils,
        // but this extension is intended to support use by applications when
        // debugging and it is strongly recommended that it be otherwise
        // avoided.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkBindBufferMemory-requirements-not-retrieved" {
        // vkBindBufferMemory2() pBindInfos[0]: Binding memory to VkBuffer
        // 0xf56c9b0000000004[] but vkGetBufferMemoryRequirements() has not been
        // called on that buffer.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkAllocateMemory-small-allocation" {
        // vkAllocateMemory(): Allocating a VkDeviceMemory of size 256. This is
        // a very small allocation (current threshold is 262144 bytes). You
        // should make large allocations and sub-allocate from one large
        // VkDeviceMemory.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkBindMemory-small-dedicated-allocation" {
        // vkBindBufferMemory2() pBindInfos[0]: Trying to bind VkBuffer
        // 0xcb3ee80000000007[] to a memory block which is fully consumed by the
        // buffer. The required size of the allocation is 256, but smaller
        // buffers like this should be sub-allocated from larger memory blocks.
        // (Current threshold is 1048576 bytes.)
        return vk::FALSE;
    }

    // Severity.
    #[allow(clippy::match_same_arms)]
    let level = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT => log::Level::Debug,
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO_EXT => log::Level::Info,
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT => log::Level::Warn,
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT => log::Level::Error,
        _ => log::Level::Warn,
    };

    // Log.
    log!(
        level,
        "message_type={message_type:?}, message_id_name={message_id_name}, message_id_number=0x{message_id_number:08x}, message={message}"
    );

    vk::FALSE
}

unsafe fn create_instance(
    loader_fn: &LoaderFunctions,
) -> Result<(InstanceFunctions, vk::Instance)> {
    // Instance-specific debug messenger.
    let debug_utils_messenger_create_info_ext = vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DebugUtilsMessengerCreateInfoEXT,
        p_next: null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::all(),
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::all(),
        pfn_user_callback: debug_utils_messenger_callback as _,
        p_user_data: null_mut(),
    };

    // Validation features.
    let enabled_validation_features = [
        vk::ValidationFeatureEnableEXT::BestPracticesEXT,
        vk::ValidationFeatureEnableEXT::SynchronizationValidationEXT,
    ];
    let validation_features_ext = vk::ValidationFeaturesEXT {
        s_type: vk::StructureType::ValidationFeaturesEXT,
        p_next: addr_of!(debug_utils_messenger_create_info_ext).cast(),
        enabled_validation_feature_count: enabled_validation_features.len() as _,
        p_enabled_validation_features: enabled_validation_features.as_ptr(),
        disabled_validation_feature_count: 0,
        p_disabled_validation_features: null(),
    };

    // Application info.
    let application_name = b"vulk-test\0";
    let engine_name = b"vulk-test\0";
    let application_info = vk::ApplicationInfo {
        s_type: vk::StructureType::ApplicationInfo,
        p_next: null(),
        p_application_name: application_name.as_ptr().cast(),
        application_version: 1,
        p_engine_name: engine_name.as_ptr().cast(),
        engine_version: 1,
        api_version: vk::make_api_version(0, 1, 3, 0),
    };

    // Create info.
    let enabled_layer_names = [b"VK_LAYER_KHRONOS_validation\0".as_ptr().cast()];
    let enabled_extension_names = [b"VK_EXT_debug_utils\0".as_ptr().cast()];
    let instance_create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::InstanceCreateInfo,
        p_next: addr_of!(validation_features_ext).cast(),
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &application_info,
        enabled_layer_count: enabled_layer_names.len() as _,
        pp_enabled_layer_names: enabled_layer_names.as_ptr(),
        enabled_extension_count: enabled_extension_names.len() as _,
        pp_enabled_extension_names: enabled_extension_names.as_ptr(),
    };

    // Create.
    let instance = loader_fn.create_instance(&instance_create_info, null())?;
    let instance_fn = InstanceFunctions::load(loader_fn, instance)?;

    Ok((instance_fn, instance))
}

unsafe fn create_debug_utils_messenger(
    instance_fn: &InstanceFunctions,
    instance: vk::Instance,
) -> Result<vk::DebugUtilsMessengerEXT> {
    let debug_utils_messenger_create_info_ext = vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DebugUtilsMessengerCreateInfoEXT,
        p_next: null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::all(),
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::all(),
        pfn_user_callback: debug_utils_messenger_callback as _,
        p_user_data: null_mut(),
    };
    instance_fn
        .create_debug_utils_messenger_ext(&debug_utils_messenger_create_info_ext, null())
        .map_err(Into::into)
}

struct PhysicalDevice {
    handle: vk::PhysicalDevice,
    #[allow(dead_code)]
    properties: vk::PhysicalDeviceProperties,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    descriptor_buffer_properties_ext: vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
}

unsafe fn create_physical_device(
    instance_fn: &InstanceFunctions,
    instance: vk::Instance,
) -> Result<PhysicalDevice> {
    // Enumerate physical devices.
    let mut physical_device_count = 0;
    instance_fn.enumerate_physical_devices(&mut physical_device_count, null_mut())?;
    let mut physical_devices = Vec::with_capacity(physical_device_count as _);
    instance_fn
        .enumerate_physical_devices(&mut physical_device_count, physical_devices.as_mut_ptr())?;
    physical_devices.set_len(physical_device_count as _);
    info!("Found {} physical devices", physical_devices.len());

    // Device properties.
    let mut physical_device_descriptor_buffer_properties_ext =
        vk::PhysicalDeviceDescriptorBufferPropertiesEXT {
            s_type: vk::StructureType::PhysicalDeviceDescriptorBufferPropertiesEXT,
            p_next: null_mut(),
            combined_image_sampler_descriptor_single_array: vk::FALSE,
            bufferless_push_descriptors: vk::FALSE,
            allow_sampler_image_view_post_submit_creation: vk::FALSE,
            descriptor_buffer_offset_alignment: 0,
            max_descriptor_buffer_bindings: 0,
            max_resource_descriptor_buffer_bindings: 0,
            max_sampler_descriptor_buffer_bindings: 0,
            max_embedded_immutable_sampler_bindings: 0,
            max_embedded_immutable_samplers: 0,
            buffer_capture_replay_descriptor_data_size: 0,
            image_capture_replay_descriptor_data_size: 0,
            image_view_capture_replay_descriptor_data_size: 0,
            sampler_capture_replay_descriptor_data_size: 0,
            acceleration_structure_capture_replay_descriptor_data_size: 0,
            sampler_descriptor_size: 0,
            combined_image_sampler_descriptor_size: 0,
            sampled_image_descriptor_size: 0,
            storage_image_descriptor_size: 0,
            uniform_texel_buffer_descriptor_size: 0,
            robust_uniform_texel_buffer_descriptor_size: 0,
            storage_texel_buffer_descriptor_size: 0,
            robust_storage_texel_buffer_descriptor_size: 0,
            uniform_buffer_descriptor_size: 0,
            robust_uniform_buffer_descriptor_size: 0,
            storage_buffer_descriptor_size: 0,
            robust_storage_buffer_descriptor_size: 0,
            input_attachment_descriptor_size: 0,
            acceleration_structure_descriptor_size: 0,
            max_sampler_descriptor_buffer_range: 0,
            max_resource_descriptor_buffer_range: 0,
            sampler_descriptor_buffer_address_space_size: 0,
            resource_descriptor_buffer_address_space_size: 0,
            descriptor_buffer_address_space_size: 0,
        };

    let mut physical_device_properties2 = vk::PhysicalDeviceProperties2 {
        s_type: vk::StructureType::PhysicalDeviceProperties2,
        p_next: addr_of_mut!(physical_device_descriptor_buffer_properties_ext).cast(),
        properties: zeroed(),
    };

    let physical_device = physical_devices[0];
    instance_fn.get_physical_device_properties2(physical_device, &mut physical_device_properties2);

    // Queue family properties.
    let mut queue_family_property_count = 0;
    instance_fn.get_physical_device_queue_family_properties2(
        physical_device,
        &mut queue_family_property_count,
        null_mut(),
    );
    let mut queue_family_properties = vec![
        vk::QueueFamilyProperties2 {
            s_type: vk::StructureType::QueueFamilyProperties2,
            p_next: null_mut(),
            queue_family_properties: zeroed()
        };
        queue_family_property_count as _
    ];
    instance_fn.get_physical_device_queue_family_properties2(
        physical_device,
        &mut queue_family_property_count,
        queue_family_properties.as_mut_ptr(),
    );

    // Memory properties.
    let mut physical_device_memory_properties2 = vk::PhysicalDeviceMemoryProperties2 {
        s_type: vk::StructureType::PhysicalDeviceMemoryProperties2,
        p_next: null_mut(),
        memory_properties: zeroed(),
    };
    instance_fn.get_physical_device_memory_properties2(
        physical_device,
        &mut physical_device_memory_properties2,
    );

    Ok(PhysicalDevice {
        handle: physical_device,
        properties: physical_device_properties2.properties,
        queue_family_properties: queue_family_properties
            .into_iter()
            .map(|queue_family_property| queue_family_property.queue_family_properties)
            .collect(),
        memory_properties: physical_device_memory_properties2.memory_properties,
        descriptor_buffer_properties_ext: physical_device_descriptor_buffer_properties_ext,
    })
}

struct QueueFamily {
    index: u32,
    #[allow(dead_code)]
    properties: vk::QueueFamilyProperties,
}

unsafe fn create_queue_family(physical_device: &PhysicalDevice) -> Result<QueueFamily> {
    physical_device
        .queue_family_properties
        .iter()
        .enumerate()
        .find_map(|(queue_family_index, queue_family_properties)| {
            let required_queue_flags =
                vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;
            if queue_family_properties
                .queue_flags
                .contains(required_queue_flags)
            {
                Some(QueueFamily {
                    index: queue_family_index as _,
                    properties: *queue_family_properties,
                })
            } else {
                None
            }
        })
        .context("Finding compatible queue families")
}

unsafe fn create_device(
    instance_fn: &InstanceFunctions,
    physical_device: &PhysicalDevice,
    queue_family: &QueueFamily,
) -> Result<(DeviceFunctions, vk::Device)> {
    // Queue create info.
    let device_queue_create_info = vk::DeviceQueueCreateInfo {
        s_type: vk::StructureType::DeviceQueueCreateInfo,
        p_next: null(),
        flags: vk::DeviceQueueCreateFlags::empty(),
        queue_family_index: queue_family.index,
        queue_count: 1,
        p_queue_priorities: [1.0].as_ptr(),
    };

    // Features.
    let mut physical_device_shader_object_features_ext =
        vk::PhysicalDeviceShaderObjectFeaturesEXT {
            s_type: vk::StructureType::PhysicalDeviceShaderObjectFeaturesEXT,
            p_next: null_mut(),
            shader_object: vk::TRUE,
        };
    let mut physical_device_descriptor_buffer_features_ext =
        vk::PhysicalDeviceDescriptorBufferFeaturesEXT {
            s_type: vk::StructureType::PhysicalDeviceDescriptorBufferFeaturesEXT,
            p_next: addr_of_mut!(physical_device_shader_object_features_ext).cast(),
            descriptor_buffer: vk::TRUE,
            descriptor_buffer_capture_replay: vk::FALSE,
            descriptor_buffer_image_layout_ignored: vk::FALSE,
            descriptor_buffer_push_descriptors: vk::FALSE,
        };
    let mut physical_device_vulkan13_features = vk::PhysicalDeviceVulkan13Features {
        s_type: vk::StructureType::PhysicalDeviceVulkan13Features,
        p_next: addr_of_mut!(physical_device_descriptor_buffer_features_ext).cast(),
        robust_image_access: vk::FALSE,
        inline_uniform_block: vk::FALSE,
        descriptor_binding_inline_uniform_block_update_after_bind: vk::FALSE,
        pipeline_creation_cache_control: vk::FALSE,
        private_data: vk::FALSE,
        shader_demote_to_helper_invocation: vk::FALSE,
        shader_terminate_invocation: vk::FALSE,
        subgroup_size_control: vk::FALSE,
        compute_full_subgroups: vk::FALSE,
        synchronization2: vk::TRUE,
        texture_compression_astc_hdr: vk::FALSE,
        shader_zero_initialize_workgroup_memory: vk::FALSE,
        dynamic_rendering: vk::FALSE,
        shader_integer_dot_product: vk::FALSE,
        maintenance4: vk::FALSE,
    };
    let mut physical_device_vulkan12_features = vk::PhysicalDeviceVulkan12Features {
        s_type: vk::StructureType::PhysicalDeviceVulkan12Features,
        p_next: addr_of_mut!(physical_device_vulkan13_features).cast(),
        sampler_mirror_clamp_to_edge: vk::FALSE,
        draw_indirect_count: vk::FALSE,
        storage_buffer8_bit_access: vk::FALSE,
        uniform_and_storage_buffer8_bit_access: vk::FALSE,
        storage_push_constant8: vk::FALSE,
        shader_buffer_int64_atomics: vk::FALSE,
        shader_shared_int64_atomics: vk::FALSE,
        shader_float16: vk::FALSE,
        shader_int8: vk::FALSE,
        descriptor_indexing: vk::FALSE,
        shader_input_attachment_array_dynamic_indexing: vk::FALSE,
        shader_uniform_texel_buffer_array_dynamic_indexing: vk::FALSE,
        shader_storage_texel_buffer_array_dynamic_indexing: vk::FALSE,
        shader_uniform_buffer_array_non_uniform_indexing: vk::FALSE,
        shader_sampled_image_array_non_uniform_indexing: vk::FALSE,
        shader_storage_buffer_array_non_uniform_indexing: vk::FALSE,
        shader_storage_image_array_non_uniform_indexing: vk::FALSE,
        shader_input_attachment_array_non_uniform_indexing: vk::FALSE,
        shader_uniform_texel_buffer_array_non_uniform_indexing: vk::FALSE,
        shader_storage_texel_buffer_array_non_uniform_indexing: vk::FALSE,
        descriptor_binding_uniform_buffer_update_after_bind: vk::FALSE,
        descriptor_binding_sampled_image_update_after_bind: vk::FALSE,
        descriptor_binding_storage_image_update_after_bind: vk::FALSE,
        descriptor_binding_storage_buffer_update_after_bind: vk::FALSE,
        descriptor_binding_uniform_texel_buffer_update_after_bind: vk::FALSE,
        descriptor_binding_storage_texel_buffer_update_after_bind: vk::FALSE,
        descriptor_binding_update_unused_while_pending: vk::FALSE,
        descriptor_binding_partially_bound: vk::FALSE,
        descriptor_binding_variable_descriptor_count: vk::FALSE,
        runtime_descriptor_array: vk::FALSE,
        sampler_filter_minmax: vk::FALSE,
        scalar_block_layout: vk::FALSE,
        imageless_framebuffer: vk::FALSE,
        uniform_buffer_standard_layout: vk::FALSE,
        shader_subgroup_extended_types: vk::FALSE,
        separate_depth_stencil_layouts: vk::FALSE,
        host_query_reset: vk::FALSE,
        timeline_semaphore: vk::TRUE,
        buffer_device_address: vk::TRUE,
        buffer_device_address_capture_replay: vk::FALSE,
        buffer_device_address_multi_device: vk::FALSE,
        vulkan_memory_model: vk::FALSE,
        vulkan_memory_model_device_scope: vk::FALSE,
        vulkan_memory_model_availability_visibility_chains: vk::FALSE,
        shader_output_viewport_index: vk::FALSE,
        shader_output_layer: vk::FALSE,
        subgroup_broadcast_dynamic_id: vk::FALSE,
    };
    let physical_device_features2 = vk::PhysicalDeviceFeatures2 {
        s_type: vk::StructureType::PhysicalDeviceFeatures2,
        p_next: addr_of_mut!(physical_device_vulkan12_features).cast(),
        features: vk::PhysicalDeviceFeatures {
            robust_buffer_access: vk::FALSE,
            full_draw_index_uint32: vk::FALSE,
            image_cube_array: vk::FALSE,
            independent_blend: vk::FALSE,
            geometry_shader: vk::FALSE,
            tessellation_shader: vk::FALSE,
            sample_rate_shading: vk::FALSE,
            dual_src_blend: vk::FALSE,
            logic_op: vk::FALSE,
            multi_draw_indirect: vk::FALSE,
            draw_indirect_first_instance: vk::FALSE,
            depth_clamp: vk::FALSE,
            depth_bias_clamp: vk::FALSE,
            fill_mode_non_solid: vk::FALSE,
            depth_bounds: vk::FALSE,
            wide_lines: vk::FALSE,
            large_points: vk::FALSE,
            alpha_to_one: vk::FALSE,
            multi_viewport: vk::FALSE,
            sampler_anisotropy: vk::FALSE,
            texture_compression_etc2: vk::FALSE,
            texture_compression_astc_ldr: vk::FALSE,
            texture_compression_bc: vk::FALSE,
            occlusion_query_precise: vk::FALSE,
            pipeline_statistics_query: vk::FALSE,
            vertex_pipeline_stores_and_atomics: vk::FALSE,
            fragment_stores_and_atomics: vk::FALSE,
            shader_tessellation_and_geometry_point_size: vk::FALSE,
            shader_image_gather_extended: vk::FALSE,
            shader_storage_image_extended_formats: vk::FALSE,
            shader_storage_image_multisample: vk::FALSE,
            shader_storage_image_read_without_format: vk::FALSE,
            shader_storage_image_write_without_format: vk::FALSE,
            shader_uniform_buffer_array_dynamic_indexing: vk::FALSE,
            shader_sampled_image_array_dynamic_indexing: vk::FALSE,
            shader_storage_buffer_array_dynamic_indexing: vk::FALSE,
            shader_storage_image_array_dynamic_indexing: vk::FALSE,
            shader_clip_distance: vk::FALSE,
            shader_cull_distance: vk::FALSE,
            shader_float64: vk::FALSE,
            shader_int64: vk::FALSE,
            shader_int16: vk::FALSE,
            shader_resource_residency: vk::FALSE,
            shader_resource_min_lod: vk::FALSE,
            sparse_binding: vk::FALSE,
            sparse_residency_buffer: vk::FALSE,
            sparse_residency_image_2d: vk::FALSE,
            sparse_residency_image_3d: vk::FALSE,
            sparse_residency2_samples: vk::FALSE,
            sparse_residency4_samples: vk::FALSE,
            sparse_residency8_samples: vk::FALSE,
            sparse_residency16_samples: vk::FALSE,
            sparse_residency_aliased: vk::FALSE,
            variable_multisample_rate: vk::FALSE,
            inherited_queries: vk::FALSE,
        },
    };

    // Extensions.
    let enabled_extension_names = [
        b"VK_KHR_map_memory2\0".as_ptr().cast(),
        b"VK_EXT_descriptor_buffer\0".as_ptr().cast(),
        b"VK_EXT_shader_object\0".as_ptr().cast(),
    ];

    // Create info.
    let device_create_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DeviceCreateInfo,
        p_next: addr_of!(physical_device_features2).cast(),
        flags: vk::DeviceCreateFlags::empty(),
        queue_create_info_count: 1,
        p_queue_create_infos: addr_of!(device_queue_create_info).cast(),
        enabled_layer_count: 0,
        pp_enabled_layer_names: null(),
        enabled_extension_count: enabled_extension_names.len() as _,
        pp_enabled_extension_names: enabled_extension_names.as_ptr(),
        p_enabled_features: null(),
    };

    // Create.
    let device = instance_fn.create_device(physical_device.handle, &device_create_info, null())?;
    let device_fn = DeviceFunctions::load(instance_fn, device)?;

    Ok((device_fn, device))
}

unsafe fn create_queue(
    device_fn: &DeviceFunctions,
    device: vk::Device,
    queue_family: &QueueFamily,
) -> vulk::vk::Queue {
    let device_queue_info2 = vk::DeviceQueueInfo2 {
        s_type: vk::StructureType::DeviceQueueInfo2,
        p_next: null(),
        flags: vk::DeviceQueueCreateFlags::empty(),
        queue_family_index: queue_family.index,
        queue_index: 0,
    };
    let mut queue = MaybeUninit::uninit();
    device_fn.get_device_queue2(&device_queue_info2, queue.as_mut_ptr());
    queue.assume_init()
}

struct Commands {
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    semaphore: vk::Semaphore,
}

unsafe fn create_commands(
    device_fn: &DeviceFunctions,
    device: vk::Device,
    queue_family: &QueueFamily,
) -> Result<Commands> {
    // Command pool.
    let command_pool = {
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: null(),
            flags: vk::CommandPoolCreateFlags::empty(),
            queue_family_index: queue_family.index,
        };
        let command_pool = device_fn.create_command_pool(&command_pool_create_info, null())?;
        device_fn.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;
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
        device_fn
            .allocate_command_buffers(&command_buffer_allocate_info, command_buffer.as_mut_ptr())?;
        command_buffer.assume_init()
    };

    // Semaphore
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
    let semaphore = device_fn.create_semaphore(addr_of!(semaphore_create_info).cast(), null())?;

    Ok(Commands {
        command_pool,
        command_buffer,
        semaphore,
    })
}

fn memory_type_index(
    memory: &vk::PhysicalDeviceMemoryProperties,
    property_flags: vk::MemoryPropertyFlags,
    requirements: &vk::MemoryRequirements,
) -> u32 {
    let memory_type_bits = requirements.memory_type_bits;
    for memory_type_index in 0..memory.memory_type_count {
        let memory_type = memory.memory_types[memory_type_index as usize];
        let type_matches = (1 << memory_type_index) & memory_type_bits != 0;
        let property_matches = memory_type.property_flags & property_flags == property_flags;
        if type_matches && property_matches {
            return memory_type_index;
        }
    }
    panic!("Unable to find suitable memory type for the buffer, memory_type_bits=0b{memory_type_bits:b}");
}

type ComputeBuffer = resource::Buffer<u32>;

unsafe fn create_compute_buffer(
    device_fn: &DeviceFunctions,
    physical_device: &PhysicalDevice,
    device: vk::Device,
) -> Result<ComputeBuffer> {
    let element_count = 8;
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = ComputeBuffer::create(
        device_fn,
        device,
        physical_device,
        element_count,
        usage,
        flags,
    )?;
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

unsafe fn create_indirect_buffer(
    device_fn: &DeviceFunctions,
    physical_device: &PhysicalDevice,
    device: vk::Device,
) -> Result<IndirectBuffer> {
    let element_count = 1;
    let usage = vk::BufferUsageFlags::INDIRECT_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = IndirectBuffer::create(
        device_fn,
        device,
        physical_device,
        element_count,
        usage,
        flags,
    )?;
    Ok(buffer)
}

type DescriptorBuffer = resource::Buffer<u8>;

struct Descriptors {
    set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    buffer: DescriptorBuffer,
}

unsafe fn create_descriptors(
    device_fn: &DeviceFunctions,
    physical_device: &PhysicalDevice,
    device: vk::Device,
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
    let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
        s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
        p_next: null(),
        flags: vk::DescriptorSetLayoutCreateFlags::DESCRIPTOR_BUFFER_EXT,
        binding_count: bindings.len() as _,
        p_bindings: bindings.as_ptr(),
    };
    let descriptor_set_layout =
        device_fn.create_descriptor_set_layout(&descriptor_set_layout_create_info, null())?;

    // Descriptor buffer.
    let mut buffer_size = MaybeUninit::uninit();
    device_fn.get_descriptor_set_layout_size_ext(descriptor_set_layout, buffer_size.as_mut_ptr());
    let buffer_size = buffer_size.assume_init();
    info!("Descriptor buffer size={buffer_size}");
    let usage = vk::BufferUsageFlags::STORAGE_BUFFER;
    let flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let buffer = DescriptorBuffer::create(
        device_fn,
        device,
        physical_device,
        buffer_size as _,
        usage,
        flags,
    )?;

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
        device_fn.get_descriptor_ext(
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
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
        s_type: vk::StructureType::PipelineLayoutCreateInfo,
        p_next: null(),
        flags: vk::PipelineLayoutCreateFlags::empty(),
        set_layout_count: set_layouts.len() as _,
        p_set_layouts: set_layouts.as_ptr(),
        push_constant_range_count: 0,
        p_push_constant_ranges: null(),
    };
    let pipeline_layout = device_fn.create_pipeline_layout(&pipeline_layout_create_info, null())?;

    Ok(Descriptors {
        set_layout: descriptor_set_layout,
        pipeline_layout,
        buffer,
    })
}

unsafe fn create_shaders(
    device_fn: &DeviceFunctions,
    device: vk::Device,
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
        device_fn.create_shaders_ext(
            1,
            addr_of!(shader_create_info_ext).cast(),
            null(),
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
            p_specialization_info: addr_of!(specialization_info).cast(),
        };
        let mut shader = MaybeUninit::uninit();
        device_fn.create_shaders_ext(
            1,
            addr_of!(shader_create_info_ext).cast(),
            null(),
            shader.as_mut_ptr(),
        )?;
        shader.assume_init()
    };

    Ok((indirect_shader, compute_shader))
}

unsafe fn execute(
    device_fn: &DeviceFunctions,
    device: vk::Device,
    queue: vk::Queue,
    commands: &Commands,
    indirect_shader: vk::ShaderEXT,
    compute_shader: vk::ShaderEXT,
    compute_buffer: &ComputeBuffer,
    indirect_buffer: &IndirectBuffer,
    descriptors: &Descriptors,
) -> Result<()> {
    // Begin command buffer.
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::CommandBufferBeginInfo,
        p_next: null(),
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        p_inheritance_info: null(),
    };
    let command_buffer = commands.command_buffer;
    device_fn.begin_command_buffer(command_buffer, &command_buffer_begin_info)?;

    // Descriptors.
    let descriptor_buffer_binding_info_ext = vk::DescriptorBufferBindingInfoEXT {
        s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
        p_next: null_mut(),
        address: descriptors.buffer.device_address,
        usage: vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER,
    };
    let binding_infos = [descriptor_buffer_binding_info_ext];
    device_fn.cmd_bind_descriptor_buffers_ext(
        command_buffer,
        binding_infos.len() as _,
        binding_infos.as_ptr(),
    );
    let buffer_indices = [0];
    let offsets = [0];
    device_fn.cmd_set_descriptor_buffer_offsets_ext(
        command_buffer,
        vk::PipelineBindPoint::Compute,
        descriptors.pipeline_layout,
        0,
        buffer_indices.len() as _,
        buffer_indices.as_ptr(),
        offsets.as_ptr(),
    );

    // Dispatch indirect shader.
    let stages = [vk::ShaderStageFlagBits::COMPUTE];
    device_fn.cmd_bind_shaders_ext(
        command_buffer,
        stages.len() as _,
        stages.as_ptr(),
        addr_of!(indirect_shader),
    );
    device_fn.cmd_dispatch(command_buffer, 1, 1, 1);

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
        device_fn.cmd_pipeline_barrier2(command_buffer, &dependency_info);
    }

    // Dispatch compute shader.
    let stages = [vk::ShaderStageFlagBits::COMPUTE];
    device_fn.cmd_bind_shaders_ext(
        command_buffer,
        stages.len() as _,
        stages.as_ptr(),
        addr_of!(compute_shader),
    );
    device_fn.cmd_dispatch_indirect(command_buffer, indirect_buffer.handle, 0);

    // End command buffer.
    device_fn.end_command_buffer(command_buffer)?;

    // Queue submit.
    let command_buffer_submit_info = vk::CommandBufferSubmitInfo {
        s_type: vk::StructureType::CommandBufferSubmitInfo,
        p_next: null(),
        command_buffer,
        device_mask: 0,
    };
    let semaphore_submit_info = vk::SemaphoreSubmitInfo {
        s_type: vk::StructureType::SemaphoreSubmitInfo,
        p_next: null(),
        semaphore: commands.semaphore,
        value: 1,
        stage_mask: vk::PipelineStageFlags2::COMPUTE_SHADER,
        device_index: 0,
    };
    let submit_info2 = vk::SubmitInfo2 {
        s_type: vk::StructureType::SubmitInfo2,
        p_next: null(),
        flags: vk::SubmitFlags::empty(),
        wait_semaphore_info_count: 0,
        p_wait_semaphore_infos: null(),
        command_buffer_info_count: 1,
        p_command_buffer_infos: addr_of!(command_buffer_submit_info).cast(),
        signal_semaphore_info_count: 1,
        p_signal_semaphore_infos: addr_of!(semaphore_submit_info).cast(),
    };
    device_fn.queue_submit2(queue, 1, &submit_info2, vk::Fence::null())?;

    // Wait.
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
    device_fn.wait_semaphores(&semaphore_wait_info, u64::MAX)?;

    // Validate.
    #[allow(clippy::cast_ptr_alignment)]
    let p_data = std::slice::from_raw_parts(indirect_buffer.ptr, indirect_buffer.element_count);
    info!("indirect_buffer={:?}", &p_data);
    let p_data = std::slice::from_raw_parts(compute_buffer.ptr, compute_buffer.element_count);
    info!("compute_buffer={:?}", &p_data);

    Ok(())
}
