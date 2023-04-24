use super::*;

//
// Gpu
//

pub struct Gpu {
    pub instance: vkx::Instance,
    pub physical_device: PhysicalDevice,
    pub queue_family: QueueFamily,
    pub device: vulk::Device,
    pub queue: vk::Queue,
    pub timestamp_calibration: TimestampCalibration,
}

impl Gpu {
    pub unsafe fn create() -> Result<Self> {
        let instance = vkx::Instance::create(&vkx::InstanceCreateInfo {
            application_name: "vulk-test",
            engine_name: "vulk-test",
            validation_layers: true,
        })?;
        let physical_device =
            create_physical_device(&instance).context("Creating physical device")?;
        let queue_family = find_queue_family(&physical_device).context("Find queue family")?;
        let device = create_logical_device(&instance, &physical_device, &queue_family)
            .context("Creating device")?;
        let queue = get_queue(&device, &queue_family);
        let timestamp_calibration = get_timestamp_calibration(&instance, &physical_device, &device)
            .context("Getting timestamp calibration")?;
        Ok(Self {
            instance,
            physical_device,
            queue_family,
            device,
            queue,
            timestamp_calibration,
        })
    }

    pub unsafe fn destroy(self) {
        self.device.destroy_device();
        self.instance.destroy();
    }
}

//
// Physical device
//

pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub queue_family_properties: Vec<vk::QueueFamilyProperties>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub descriptor_buffer_properties_ext: vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
    pub mesh_shader_properties_ext: vk::PhysicalDeviceMeshShaderPropertiesEXT,
    pub subgroup_properties: vk::PhysicalDeviceSubgroupProperties,
    pub acceleration_structure_properties: vk::PhysicalDeviceAccelerationStructurePropertiesKHR,
    pub raytracing_pipeline_properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR,
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
    let mut as_props: vk::PhysicalDeviceAccelerationStructurePropertiesKHR = zeroed();
    as_props.s_type = vk::StructureType::PhysicalDeviceAccelerationStructurePropertiesKHR;
    let mut rtp_props: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR = zeroed();
    rtp_props.s_type = vk::StructureType::PhysicalDeviceRayTracingPipelinePropertiesKHR;
    rtp_props.p_next = addr_of_mut!(as_props).cast();
    let mut sg_props: vk::PhysicalDeviceSubgroupProperties = zeroed();
    sg_props.s_type = vk::StructureType::PhysicalDeviceSubgroupProperties;
    sg_props.p_next = addr_of_mut!(rtp_props).cast();
    let mut ms_props: vk::PhysicalDeviceMeshShaderPropertiesEXT = zeroed();
    ms_props.s_type = vk::StructureType::PhysicalDeviceMeshShaderPropertiesEXT;
    ms_props.p_next = addr_of_mut!(sg_props).cast();
    let mut db_props: vk::PhysicalDeviceDescriptorBufferPropertiesEXT = zeroed();
    db_props.s_type = vk::StructureType::PhysicalDeviceDescriptorBufferPropertiesEXT;
    db_props.p_next = addr_of_mut!(ms_props).cast();
    let mut props2 = vk::PhysicalDeviceProperties2 {
        s_type: vk::StructureType::PhysicalDeviceProperties2,
        p_next: addr_of_mut!(db_props).cast(),
        properties: zeroed(),
    };
    instance.get_physical_device_properties2(physical_device, &mut props2);

    // Assert that our descriptor type can fit any kind of descriptor.
    descriptor::validate_descriptor_sizes(&db_props)?;

    // Queue family properties.
    let qf_props = {
        let qf_props2 = vulk::read_to_vec(
            |a, b| {
                instance.get_physical_device_queue_family_properties2(physical_device, a, b);
                Ok(())
            },
            Some(vk::StructureType::QueueFamilyProperties2),
        )?;
        qf_props2
            .into_iter()
            .map(|qf_prop| qf_prop.queue_family_properties)
            .collect()
    };

    // Memory properties.
    let mem_props = {
        let mut mem_props2 = vk::PhysicalDeviceMemoryProperties2 {
            s_type: vk::StructureType::PhysicalDeviceMemoryProperties2,
            p_next: null_mut(),
            memory_properties: zeroed(),
        };
        instance.get_physical_device_memory_properties2(physical_device, &mut mem_props2);
        mem_props2.memory_properties
    };

    Ok(PhysicalDevice {
        handle: physical_device,
        properties: props2.properties,
        queue_family_properties: qf_props,
        memory_properties: mem_props,
        descriptor_buffer_properties_ext: db_props,
        mesh_shader_properties_ext: ms_props,
        subgroup_properties: sg_props,
        acceleration_structure_properties: as_props,
        raytracing_pipeline_properties: rtp_props,
    })
}

//
// Queue family
//

pub struct QueueFamily {
    pub index: u32,
    pub properties: vk::QueueFamilyProperties,
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
            if queue_flags.contains(required_queue_flags) {
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

//
// Logical device
//

unsafe fn create_logical_device(
    instance: &vulk::Instance,
    physical_device: &PhysicalDevice,
    queue_family: &QueueFamily,
) -> Result<vulk::Device> {
    // Features.
    let mut physical_device_ray_tracing_maintenance1_features_khr =
        vk::PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
            s_type: vk::StructureType::PhysicalDeviceRayTracingMaintenance1FeaturesKHR,
            p_next: null_mut(),
            ray_tracing_maintenance1: vk::TRUE,
            ray_tracing_pipeline_trace_rays_indirect2: vk::TRUE,
        };
    let mut physical_device_acceleration_structure_features_khr =
        vk::PhysicalDeviceAccelerationStructureFeaturesKHR {
            s_type: vk::StructureType::PhysicalDeviceAccelerationStructureFeaturesKHR,
            p_next: addr_of_mut!(physical_device_ray_tracing_maintenance1_features_khr).cast(),
            acceleration_structure: vk::TRUE,
            acceleration_structure_capture_replay: vk::FALSE,
            acceleration_structure_indirect_build: vk::FALSE,
            acceleration_structure_host_commands: vk::FALSE,
            descriptor_binding_acceleration_structure_update_after_bind: vk::FALSE,
        };
    let mut physical_device_ray_tracing_pipeline_features_khr =
        vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {
            s_type: vk::StructureType::PhysicalDeviceRayTracingPipelineFeaturesKHR,
            p_next: addr_of_mut!(physical_device_acceleration_structure_features_khr).cast(),
            ray_tracing_pipeline: vk::TRUE,
            ray_tracing_pipeline_shader_group_handle_capture_replay: vk::FALSE,
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: vk::FALSE,
            ray_tracing_pipeline_trace_rays_indirect: vk::TRUE,
            ray_traversal_primitive_culling: vk::TRUE,
        };
    let mut physical_device_ray_query_features_khr = vk::PhysicalDeviceRayQueryFeaturesKHR {
        s_type: vk::StructureType::PhysicalDeviceRayQueryFeaturesKHR,
        p_next: addr_of_mut!(physical_device_ray_tracing_pipeline_features_khr).cast(),
        ray_query: vk::TRUE,
    };
    let mut physical_device_mesh_shader_features_ext = vk::PhysicalDeviceMeshShaderFeaturesEXT {
        s_type: vk::StructureType::PhysicalDeviceMeshShaderFeaturesEXT,
        p_next: addr_of_mut!(physical_device_ray_query_features_khr).cast(),
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
        shader_buffer_int64_atomics: vk::TRUE,
        shader_shared_int64_atomics: vk::FALSE,
        shader_float16: vk::FALSE,
        shader_int8: vk::FALSE,
        descriptor_indexing: vk::TRUE,
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
            shader_int64: vk::TRUE,
            shader_int16: vk::TRUE,
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
            enabled_extension_count: vulk::REQUIRED_DEVICE_EXTENSIONS.len() as _,
            pp_enabled_extension_names: vulk::REQUIRED_DEVICE_EXTENSIONS.as_ptr(),
            p_enabled_features: null(),
        }),
    )?;
    let device = vulk::Device::load(instance, device)?;

    Ok(device)
}

//
// Queue
//

unsafe fn get_queue(device: &vulk::Device, queue_family: &QueueFamily) -> vk::Queue {
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

//
// Timestamp calibration
//

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
    {
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
                vk::TimeDomainEXT::ClockMonotonicEXT
                    | vk::TimeDomainEXT::QueryPerformanceCounterEXT
            )
        });
        let supports_device_domain = time_domains
            .iter()
            .any(|td| matches!(*td, vk::TimeDomainEXT::DeviceEXT));
        ensure!(supports_host_domain);
        ensure!(supports_device_domain);
    }

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
    debug!(
        "Timestamp calibration: \
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
