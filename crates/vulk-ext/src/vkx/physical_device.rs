use super::*;

pub struct PhysicalDevice {
    physical_device: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub queue_family_properties: Vec<vk::QueueFamilyProperties>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub descriptor_buffer_properties_ext: vk::PhysicalDeviceDescriptorBufferPropertiesEXT,
    pub mesh_shader_properties_ext: vk::PhysicalDeviceMeshShaderPropertiesEXT,
    pub subgroup_properties: vk::PhysicalDeviceSubgroupProperties,
    pub acceleration_structure_properties: vk::PhysicalDeviceAccelerationStructurePropertiesKHR,
    pub raytracing_pipeline_properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR,
}

// Todo: descriptor::validate_descriptor_sizes(&db_props)?;

impl PhysicalDevice {
    pub unsafe fn create(instance: &Instance) -> Result<Self> {
        // Find physical devices.
        let physical_devices = vulk::read_to_vec(
            |count, ptr| instance.enumerate_physical_devices(count, ptr),
            None,
        )?;

        // Pick a physical device.
        let physical_device = physical_devices[0];

        // Physical device properties.
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

        Ok(Self {
            physical_device,
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
}

impl std::ops::Deref for PhysicalDevice {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        &self.physical_device
    }
}
