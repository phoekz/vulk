use super::*;

pub struct Device {
    device: vulk::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    queue_family_properties: vk::QueueFamilyProperties,
    pub(crate) command_pool: vk::CommandPool,
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("device", &"Device {..}")
            .field("queue", &self.queue)
            .field("queue_family_index", &self.queue_family_index)
            .field("queue_family_properties", &self.queue_family_properties)
            .field("command_pool", &self.command_pool)
            .finish()
    }
}

#[derive(Debug)]
pub struct TimestampCalibration {
    // On Posix, host_domain seems to match machine uptime.
    pub host_domain: u64,
    pub device_domain: u64,
    pub max_deviation: u64,
}

impl Device {
    pub unsafe fn create(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        surface: Option<&Surface>,
    ) -> Result<Self> {
        // Find compatible queue family.
        let (queue_family_index, queue_family_properties) = physical_device
            .queue_family_properties
            .iter()
            .copied()
            .enumerate()
            .find_map(|(queue_family_index, queue_family_properties)| {
                // Present support.
                if let Some(surface) = surface {
                    let present_supported = instance
                        .get_physical_device_surface_support_khr(
                            physical_device.handle(),
                            queue_family_index as _,
                            surface.handle(),
                        )
                        .unwrap();
                    if present_supported != vk::TRUE {
                        return None;
                    }
                }

                // Queue capabilities.
                if queue_family_properties.queue_flags.contains(
                    vk::QueueFlagBits::Graphics
                        | vk::QueueFlagBits::Compute
                        | vk::QueueFlagBits::Transfer,
                ) {
                    Some((queue_family_index as u32, queue_family_properties))
                } else {
                    None
                }
            })
            .context("Finding compatible queue families")?;

        // Required features.
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
        let mut physical_device_mesh_shader_features_ext =
            vk::PhysicalDeviceMeshShaderFeaturesEXT {
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

        // Extensions.
        let mut enabled_extension_names = vec![];
        enabled_extension_names.extend_from_slice(&vulk::REQUIRED_DEVICE_EXTENSIONS);
        if instance.validation_layers() {
            enabled_extension_names.extend_from_slice(&vulk::DEBUGGING_DEVICE_EXTENSIONS);
        }
        if cfg!(windows) {
            enabled_extension_names.extend_from_slice(&vulk::WIN32_DEVICE_EXTENSIONS);
        }

        // Device.
        let device = instance.create_device(
            physical_device.handle(),
            &vk::DeviceCreateInfo {
                s_type: vk::StructureType::DeviceCreateInfo,
                p_next: addr_of!(physical_device_features2).cast(),
                flags: vk::DeviceCreateFlags::empty(),
                queue_create_info_count: 1,
                p_queue_create_infos: &vk::DeviceQueueCreateInfo {
                    s_type: vk::StructureType::DeviceQueueCreateInfo,
                    p_next: null(),
                    flags: vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index,
                    queue_count: 1,
                    p_queue_priorities: [1.0].as_ptr(),
                },
                enabled_layer_count: 0,
                pp_enabled_layer_names: null(),
                enabled_extension_count: enabled_extension_names.len() as _,
                pp_enabled_extension_names: enabled_extension_names.as_ptr(),
                p_enabled_features: null(),
            },
        )?;
        let device = vulk::Device::load(instance, device)?;

        // Queue.
        let queue = device.get_device_queue2(&vk::DeviceQueueInfo2 {
            s_type: vk::StructureType::DeviceQueueInfo2,
            p_next: null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index,
            queue_index: 0,
        });

        // Command pool.
        let command_pool = device.create_command_pool(&vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: null(),
            flags: vk::CommandPoolCreateFlagBits::ResetCommandBuffer.into(),
            queue_family_index,
        })?;

        // Timestamp calibration support.
        let time_domains = vulk::read_to_vec(
            |count, ptr| {
                instance.get_physical_device_calibrateable_time_domains_ext(
                    physical_device.handle(),
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
        ensure!(supports_host_domain);
        let supports_device_domain = time_domains
            .iter()
            .any(|td| matches!(*td, vk::TimeDomainEXT::DeviceEXT));
        ensure!(supports_device_domain);

        Ok(Self {
            device,
            queue,
            queue_family_index,
            queue_family_properties,
            command_pool,
        })
    }

    pub unsafe fn destroy(self) {
        self.device.destroy_command_pool(self.command_pool);
        self.device.destroy_device();
    }

    #[must_use]
    pub fn queue_handle(&self) -> vk::Queue {
        self.queue
    }

    #[must_use]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    #[must_use]
    pub fn queue_family_properties(&self) -> vk::QueueFamilyProperties {
        self.queue_family_properties
    }

    pub unsafe fn timestamp_calibration(&self) -> Result<TimestampCalibration> {
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
        self.device.get_calibrated_timestamps_ext(
            calibrated_timestamp_info_ext.len() as _,
            calibrated_timestamp_info_ext.as_ptr(),
            timestamps.as_mut_ptr(),
            &mut max_deviation,
        )?;
        let host_domain = timestamps[0];
        let device_domain = timestamps[1];
        Ok(TimestampCalibration {
            host_domain,
            device_domain,
            max_deviation,
        })
    }
}

impl std::ops::Deref for Device {
    type Target = vulk::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
