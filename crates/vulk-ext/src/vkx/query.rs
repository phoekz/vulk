use super::*;

pub struct TimestampQuery {
    pub(super) query_pool: vk::QueryPool,
    query_count: u32,
    timestamp_period: f64,
}

impl TimestampQuery {
    pub unsafe fn create(
        physical_device: &PhysicalDevice,
        device: &Device,
        query_count: u32,
    ) -> Result<Self> {
        ensure!(query_count >= 2);
        let query_pool = device.create_query_pool(&vk::QueryPoolCreateInfo {
            s_type: vk::StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: vk::QueryPoolCreateFlags::empty(),
            query_type: vk::QueryType::Timestamp,
            query_count,
            pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
        })?;
        device.reset_query_pool(query_pool, 0, query_count);
        let timestamp_period = f64::from(physical_device.properties.limits.timestamp_period);
        Ok(Self {
            query_pool,
            query_count,
            timestamp_period,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_query_pool(self.query_pool);
    }

    pub unsafe fn reset(&self, device: &Device) {
        device.reset_query_pool(self.query_pool, 0, self.query_count);
    }

    pub unsafe fn get_raw_timestamps(&self, device: &Device) -> Result<Vec<u64>> {
        let stride = size_of::<u64>() as u64;
        let flags = vk::QueryResultFlagBits::Result64 | vk::QueryResultFlagBits::ResultWait;
        let mut timestamps: Vec<u64> = Vec::with_capacity(self.query_count as _);
        device.get_query_pool_results(
            self.query_pool,
            0,
            self.query_count,
            self.query_count as usize * size_of::<u64>(),
            timestamps.as_mut_ptr().cast(),
            stride,
            flags,
        )?;
        timestamps.set_len(self.query_count as _);
        Ok(timestamps)
    }

    pub unsafe fn get_differences(&self, device: &Device) -> Result<Vec<std::time::Duration>> {
        let timestamps = self.get_raw_timestamps(device)?;
        let differences = timestamps
            .windows(2)
            .map(|window| {
                let diff = window[1] - window[0];
                let diff = diff as f64 * self.timestamp_period;
                let diff = diff as u64;
                std::time::Duration::from_nanos(diff)
            })
            .collect();
        Ok(differences)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Statistics {
    pub pipeline_statistics: PipelineStatistics,
    pub mesh_primitives_generated: MeshPrimitivesGenerated,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineStatistics {
    pub clipping_invocations: u64,
    pub clipping_primitives: u64,
    pub fragment_shader_invocations: u64,
    pub compute_shader_invocations: u64,
    pub task_shader_invocations_ext: u64,
    pub mesh_shader_invocations_ext: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshPrimitivesGenerated {
    pub mesh_primitives_generated: u64,
}

pub struct StatisticsQuery {
    pub(super) pipeline_statistic_query_pool: vk::QueryPool,
    pub(super) mesh_primitives_generated_query_pool: vk::QueryPool,
}

impl StatisticsQuery {
    pub unsafe fn create(device: &Device) -> Result<Self> {
        // Pipeline statistic -query pool.
        let pipeline_statistic_query_pool = device.create_query_pool(&vk::QueryPoolCreateInfo {
            s_type: vk::StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: vk::QueryPoolCreateFlags::empty(),
            query_type: vk::QueryType::PipelineStatistics,
            query_count: 1,
            pipeline_statistics: vk::QueryPipelineStatisticFlagBits::ClippingInvocations
                | vk::QueryPipelineStatisticFlagBits::ClippingPrimitives
                | vk::QueryPipelineStatisticFlagBits::FragmentShaderInvocations
                | vk::QueryPipelineStatisticFlagBits::ComputeShaderInvocations
                | vk::QueryPipelineStatisticFlagBits::TaskShaderInvocationsEXT
                | vk::QueryPipelineStatisticFlagBits::MeshShaderInvocationsEXT,
        })?;
        device.reset_query_pool(pipeline_statistic_query_pool, 0, 1);

        // Mesh primitives generated -query pool.
        let mesh_primitives_generated_query_pool =
            device.create_query_pool(&vk::QueryPoolCreateInfo {
                s_type: vk::StructureType::QueryPoolCreateInfo,
                p_next: null(),
                flags: vk::QueryPoolCreateFlags::empty(),
                query_type: vk::QueryType::MeshPrimitivesGeneratedEXT,
                query_count: 1,
                pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
            })?;
        device.reset_query_pool(mesh_primitives_generated_query_pool, 0, 1);

        Ok(Self {
            pipeline_statistic_query_pool,
            mesh_primitives_generated_query_pool,
        })
    }

    pub unsafe fn destroy(self, device: &Device) {
        device.destroy_query_pool(self.pipeline_statistic_query_pool);
        device.destroy_query_pool(self.mesh_primitives_generated_query_pool);
    }

    pub unsafe fn reset(&self, device: &Device) {
        device.reset_query_pool(self.pipeline_statistic_query_pool, 0, 1);
        device.reset_query_pool(self.mesh_primitives_generated_query_pool, 0, 1);
    }

    pub unsafe fn get_statistics(&self, device: &Device) -> Result<Statistics> {
        let stride = size_of::<u64>() as u64;
        let flags = vk::QueryResultFlagBits::Result64 | vk::QueryResultFlagBits::ResultWait;
        let mut statistics: Statistics = zeroed();
        device.get_query_pool_results(
            self.pipeline_statistic_query_pool,
            0,
            1,
            size_of_val(&statistics.pipeline_statistics),
            addr_of_mut!(statistics.pipeline_statistics).cast(),
            stride,
            flags,
        )?;
        device.get_query_pool_results(
            self.mesh_primitives_generated_query_pool,
            0,
            1,
            size_of_val(&statistics.mesh_primitives_generated),
            addr_of_mut!(statistics.mesh_primitives_generated).cast(),
            stride,
            flags,
        )?;
        Ok(statistics)
    }
}
