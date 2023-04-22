use super::*;

pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub queue_family_properties: Vec<vk::QueueFamilyProperties>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub descriptor_buffer_properties_ext: vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
    pub mesh_shader_properties_ext: vk::PhysicalDeviceMeshShaderPropertiesEXT,
    pub subgroup_properties: vk::PhysicalDeviceSubgroupProperties,
}

pub struct QueueFamily {
    pub index: u32,
    pub properties: vk::QueueFamilyProperties,
}

pub struct Gpu {
    pub init: vulk::Init,
    pub instance: vulk::Instance,
    pub debug: debug::Debug,
    pub physical_device: PhysicalDevice,
    pub queue_family: QueueFamily,
    pub device: vulk::Device,
    pub queue: vk::Queue,
    pub timestamp_calibration: TimestampCalibration,
}

impl Gpu {
    pub unsafe fn create() -> Result<Self> {
        let init = vulk::Init::load()?;
        let instance = create_instance(&init).context("Creating instance")?;
        let debug = debug::Debug::create(&instance).context("Creating debug")?;
        let physical_device =
            create_physical_device(&instance).context("Creating physical device")?;
        let queue_family = find_queue_family(&physical_device).context("Find queue family")?;
        let device =
            create_device(&instance, &physical_device, &queue_family).context("Creating device")?;
        let queue = create_queue(&device, &queue_family);
        let timestamp_calibration =
            get_timestamp_calibration(&instance, &physical_device, &device)?;

        Ok(Self {
            init,
            instance,
            debug,
            physical_device,
            queue_family,
            device,
            queue,
            timestamp_calibration,
        })
    }

    pub unsafe fn destroy(self) {
        self.device.destroy_device();
        self.debug.destroy(&self.instance);
        self.instance.destroy_instance();
    }
}

pub struct TimestampCalibration {
    // On Posix, host_domain seems to match machine uptime.
    pub host_domain: u64,
    pub device_domain: u64,
    pub max_deviation: u64,
}

unsafe fn get_timestamp_calibration(
    instance: &vulk::Instance,
    physical_device: &PhysicalDevice,
    device: &vulk::Device,
) -> Result<TimestampCalibration> {
    // Check support.
    let time_domains = vulk::read_to_vec(
        |count, ptr| {
            instance.get_physical_device_calibrateable_time_domains_ext(
                physical_device.handle,
                count,
                ptr,
            )
        },
        None,
    )?;
    let supports_host_domain = time_domains.iter().any(|td| {
        matches!(
            *td,
            vk::TimeDomainEXT::ClockMonotonicEXT | vk::TimeDomainEXT::QueryPerformanceCounterEXT
        )
    });
    let supports_device_domain = time_domains
        .iter()
        .any(|td| matches!(*td, vk::TimeDomainEXT::DeviceEXT));
    ensure!(supports_host_domain);
    ensure!(supports_device_domain);

    // Get timestamps.
    let calibrated_timestamp_info_ext = [
        vk::CalibratedTimestampInfoEXT {
            s_type: vk::StructureType::CalibratedTimestampInfoEXT,
            p_next: null(),
            time_domain: vk::TimeDomainEXT::ClockMonotonicEXT,
        },
        vk::CalibratedTimestampInfoEXT {
            s_type: vk::StructureType::CalibratedTimestampInfoEXT,
            p_next: null(),
            time_domain: vk::TimeDomainEXT::DeviceEXT,
        },
    ];
    let mut timestamps = [0_u64; 2];
    let mut max_deviation = 0;
    device.get_calibrated_timestamps_ext(
        calibrated_timestamp_info_ext.len() as _,
        calibrated_timestamp_info_ext.as_ptr(),
        timestamps.as_mut_ptr(),
        &mut max_deviation,
    )?;
    let host_domain = timestamps[0];
    let device_domain = timestamps[1];
    info!(
        "\
        host_domain={host_domain}, \
        device_domain={device_domain}, \
        max_deviation={max_deviation}"
    );

    Ok(TimestampCalibration {
        host_domain,
        device_domain,
        max_deviation,
    })
}

unsafe fn create_instance(init: &vulk::Init) -> Result<vulk::Instance> {
    // Instance-specific debug messenger.
    let debug_utils_messenger_create_info_ext = debug::debug_utils_messenger_create_info_ext();

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

    // Layers.
    let enabled_layer_names = [b"VK_LAYER_KHRONOS_validation\0".as_ptr().cast()];

    // Extensions.
    let enabled_extension_names = [b"VK_EXT_debug_utils\0".as_ptr().cast()];

    // Create.
    let instance = init.create_instance(
        &(vk::InstanceCreateInfo {
            s_type: vk::StructureType::InstanceCreateInfo,
            p_next: addr_of!(validation_features_ext).cast(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &(vk::ApplicationInfo {
                s_type: vk::StructureType::ApplicationInfo,
                p_next: null(),
                p_application_name: b"vulk-test\0".as_ptr().cast(),
                application_version: 1,
                p_engine_name: b"vulk-test\0".as_ptr().cast(),
                engine_version: 1,
                api_version: vk::make_api_version(0, 1, 3, 0),
            }),
            enabled_layer_count: enabled_layer_names.len() as _,
            pp_enabled_layer_names: enabled_layer_names.as_ptr(),
            enabled_extension_count: enabled_extension_names.len() as _,
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
        }),
    )?;
    let instance = vulk::Instance::load(init, instance)?;

    Ok(instance)
}

unsafe fn create_physical_device(instance: &vulk::Instance) -> Result<PhysicalDevice> {
    // Find physical devices.
    let physical_devices = vulk::read_to_vec(
        |count, ptr| instance.enumerate_physical_devices(count, ptr),
        None,
    )?;
    info!("Found {} physical devices", physical_devices.len());

    // Pick a physical device.
    let physical_device = physical_devices[0];

    // Device properties.
    let mut subgroup_properties: vk::PhysicalDeviceSubgroupProperties = zeroed();
    subgroup_properties.s_type = vk::StructureType::PhysicalDeviceSubgroupProperties;

    let mut mesh_shader_properties: vk::PhysicalDeviceMeshShaderPropertiesEXT = zeroed();
    mesh_shader_properties.s_type = vk::StructureType::PhysicalDeviceMeshShaderPropertiesEXT;
    mesh_shader_properties.p_next = addr_of_mut!(subgroup_properties).cast();

    let mut descriptor_buffer_properties: vk::PhysicalDeviceDescriptorBufferPropertiesEXT =
        zeroed();
    descriptor_buffer_properties.s_type =
        vk::StructureType::PhysicalDeviceDescriptorBufferPropertiesEXT;
    descriptor_buffer_properties.p_next = addr_of_mut!(mesh_shader_properties).cast();

    let mut properties2 = vk::PhysicalDeviceProperties2 {
        s_type: vk::StructureType::PhysicalDeviceProperties2,
        p_next: addr_of_mut!(descriptor_buffer_properties).cast(),
        properties: zeroed(),
    };
    instance.get_physical_device_properties2(physical_device, &mut properties2);

    // Assert that our descriptor type can fit any kind of descriptor.
    descriptor::assert_descriptor_sizes(&descriptor_buffer_properties);

    // Queue family properties.
    let queue_family_properties = vulk::read_to_vec(
        |a, b| {
            instance.get_physical_device_queue_family_properties2(physical_device, a, b);
            Ok(())
        },
        Some(vk::StructureType::QueueFamilyProperties2),
    )?;

    // Memory properties.
    let mut physical_device_memory_properties2 = vk::PhysicalDeviceMemoryProperties2 {
        s_type: vk::StructureType::PhysicalDeviceMemoryProperties2,
        p_next: null_mut(),
        memory_properties: zeroed(),
    };
    instance.get_physical_device_memory_properties2(
        physical_device,
        &mut physical_device_memory_properties2,
    );

    Ok(PhysicalDevice {
        handle: physical_device,
        properties: properties2.properties,
        queue_family_properties: queue_family_properties
            .into_iter()
            .map(|queue_family_property| queue_family_property.queue_family_properties)
            .collect(),
        memory_properties: physical_device_memory_properties2.memory_properties,
        descriptor_buffer_properties_ext: descriptor_buffer_properties,
        mesh_shader_properties_ext: mesh_shader_properties,
        subgroup_properties,
    })
}

unsafe fn find_queue_family(physical_device: &PhysicalDevice) -> Result<QueueFamily> {
    physical_device
        .queue_family_properties
        .iter()
        .enumerate()
        .find_map(|(queue_family_index, queue_family_properties)| {
            let required_queue_flags = vk::QueueFlagBits::Graphics
                | vk::QueueFlagBits::Compute
                | vk::QueueFlagBits::Transfer;
            let queue_flags = queue_family_properties.queue_flags;
            if (queue_flags & required_queue_flags) == required_queue_flags {
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
    instance: &vulk::Instance,
    physical_device: &PhysicalDevice,
    queue_family: &QueueFamily,
) -> Result<vulk::Device> {
    // Features.
    let mut physical_device_mesh_shader_features_ext = vk::PhysicalDeviceMeshShaderFeaturesEXT {
        s_type: vk::StructureType::PhysicalDeviceMeshShaderFeaturesEXT,
        p_next: null_mut(),
        task_shader: vk::TRUE,
        mesh_shader: vk::TRUE,
        multiview_mesh_shader: vk::FALSE,
        primitive_fragment_shading_rate_mesh_shader: vk::FALSE,
        mesh_shader_queries: vk::TRUE,
    };
    let mut physical_device_shader_object_features_ext =
        vk::PhysicalDeviceShaderObjectFeaturesEXT {
            s_type: vk::StructureType::PhysicalDeviceShaderObjectFeaturesEXT,
            p_next: addr_of_mut!(physical_device_mesh_shader_features_ext).cast(),
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
        dynamic_rendering: vk::TRUE,
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
        scalar_block_layout: vk::TRUE,
        imageless_framebuffer: vk::FALSE,
        uniform_buffer_standard_layout: vk::FALSE,
        shader_subgroup_extended_types: vk::FALSE,
        separate_depth_stencil_layouts: vk::FALSE,
        host_query_reset: vk::TRUE,
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
    let mut physical_device_vulkan11_features = vk::PhysicalDeviceVulkan11Features {
        s_type: vk::StructureType::PhysicalDeviceVulkan11Features,
        p_next: addr_of_mut!(physical_device_vulkan12_features).cast(),
        storage_buffer16_bit_access: vk::TRUE,
        uniform_and_storage_buffer16_bit_access: vk::TRUE,
        storage_push_constant16: vk::TRUE,
        storage_input_output16: vk::FALSE,
        multiview: vk::FALSE,
        multiview_geometry_shader: vk::FALSE,
        multiview_tessellation_shader: vk::FALSE,
        variable_pointers_storage_buffer: vk::FALSE,
        variable_pointers: vk::FALSE,
        protected_memory: vk::FALSE,
        sampler_ycbcr_conversion: vk::FALSE,
        shader_draw_parameters: vk::FALSE,
    };
    let physical_device_features2 = vk::PhysicalDeviceFeatures2 {
        s_type: vk::StructureType::PhysicalDeviceFeatures2,
        p_next: addr_of_mut!(physical_device_vulkan11_features).cast(),
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
            pipeline_statistics_query: vk::TRUE,
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
        b"VK_EXT_calibrated_timestamps\0".as_ptr().cast(),
        b"VK_EXT_mesh_shader\0".as_ptr().cast(),
    ];

    // Create.
    let device = instance.create_device(
        physical_device.handle,
        &(vk::DeviceCreateInfo {
            s_type: vk::StructureType::DeviceCreateInfo,
            p_next: addr_of!(physical_device_features2).cast(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DeviceQueueCreateInfo,
                p_next: null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family.index,
                queue_count: 1,
                p_queue_priorities: [1.0].as_ptr(),
            },
            enabled_layer_count: 0,
            pp_enabled_layer_names: null(),
            enabled_extension_count: enabled_extension_names.len() as _,
            pp_enabled_extension_names: enabled_extension_names.as_ptr(),
            p_enabled_features: null(),
        }),
    )?;
    let device = vulk::Device::load(instance, device)?;

    Ok(device)
}

unsafe fn create_queue(device: &vulk::Device, queue_family: &QueueFamily) -> vk::Queue {
    device.get_device_queue2(
        &(vk::DeviceQueueInfo2 {
            s_type: vk::StructureType::DeviceQueueInfo2,
            p_next: null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family.index,
            queue_index: 0,
        }),
    )
}
