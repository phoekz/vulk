use std::mem::size_of_val;

use super::*;

#[derive(Debug)]
pub struct QueriesCreateInfo;

#[derive(Debug)]
pub struct Queries {
    timestamp: vk::QueryPool,
    pipeline: vk::QueryPool,
    mesh_shader: vk::QueryPool,
    timestamp_period: f32,
}

impl GpuResource for Queries {
    type CreateInfo<'a> = QueriesCreateInfo;

    unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        _: &Self::CreateInfo<'_>,
    ) -> Result<Self> {
        let timestamp = device.create_query_pool(&vk::QueryPoolCreateInfo {
            s_type: vk::StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: vk::QueryPoolCreateFlags::empty(),
            query_type: vk::QueryType::Timestamp,
            query_count: 2,
            pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
        })?;
        device.reset_query_pool(timestamp, 0, 2);

        let pipeline = device.create_query_pool(&vk::QueryPoolCreateInfo {
            s_type: vk::StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: vk::QueryPoolCreateFlags::empty(),
            query_type: vk::QueryType::PipelineStatistics,
            query_count: 1,
            pipeline_statistics: vk::QueryPipelineStatisticFlagBits::FragmentShaderInvocations
                | vk::QueryPipelineStatisticFlagBits::ComputeShaderInvocations
                | vk::QueryPipelineStatisticFlagBits::TaskShaderInvocationsEXT
                | vk::QueryPipelineStatisticFlagBits::MeshShaderInvocationsEXT,
        })?;
        device.reset_query_pool(pipeline, 0, 1);

        let mesh_shader = device.create_query_pool(&vk::QueryPoolCreateInfo {
            s_type: vk::StructureType::QueryPoolCreateInfo,
            p_next: null(),
            flags: vk::QueryPoolCreateFlags::empty(),
            query_type: vk::QueryType::MeshPrimitivesGeneratedEXT,
            query_count: 1,
            pipeline_statistics: vk::QueryPipelineStatisticFlags::empty(),
        })?;
        device.reset_query_pool(mesh_shader, 0, 1);

        Ok(Self {
            timestamp,
            pipeline,
            mesh_shader,
            timestamp_period: physical_device.properties.limits.timestamp_period,
        })
    }

    unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        device.destroy_query_pool(self.timestamp);
        device.destroy_query_pool(self.pipeline);
        device.destroy_query_pool(self.mesh_shader);
    }
}

impl Queries {
    pub unsafe fn begin(&self, Gpu { device, .. }: &Gpu, cmd: vk::CommandBuffer) {
        device.cmd_write_timestamp2(
            cmd,
            vk::PipelineStageFlagBits2::TopOfPipe.into(),
            self.timestamp,
            0,
        );
        device.cmd_begin_query(cmd, self.pipeline, 0, vk::QueryControlFlags::empty());
        device.cmd_begin_query(cmd, self.mesh_shader, 0, vk::QueryControlFlags::empty());
    }

    pub unsafe fn end(&self, Gpu { device, .. }: &Gpu, cmd: vk::CommandBuffer) {
        device.cmd_end_query(cmd, self.mesh_shader, 0);
        device.cmd_end_query(cmd, self.pipeline, 0);
        device.cmd_write_timestamp2(
            cmd,
            vk::PipelineStageFlagBits2::BottomOfPipe.into(),
            self.timestamp,
            1,
        );
    }

    pub unsafe fn elapsed(&self, Gpu { device, .. }: &Gpu) -> Result<Duration> {
        let stride = size_of::<u64>() as u64;
        let flags = vk::QueryResultFlagBits::Result64 | vk::QueryResultFlagBits::ResultWait;
        let mut result: [u64; 2] = zeroed();
        device.get_query_pool_results(
            self.timestamp,
            0,
            2,
            size_of_val(&result),
            addr_of_mut!(result).cast(),
            stride,
            flags,
        )?;
        let elapsed_nanos = result[1]
            .checked_sub(result[0])
            .expect("Before-timestamp is larger than after-timestamp");
        let elapsed_nanos = (elapsed_nanos as f64 * f64::from(self.timestamp_period)) as u64;
        Ok(Duration::from_nanos(elapsed_nanos))
    }

    pub unsafe fn statistics(&self, Gpu { device, .. }: &Gpu) -> Result<Statistics> {
        let stride = size_of::<u64>() as u64;
        let flags = vk::QueryResultFlagBits::Result64 | vk::QueryResultFlagBits::ResultWait;
        let mut result: Statistics = zeroed();
        device.get_query_pool_results(
            self.pipeline,
            0,
            1,
            size_of_val(&result.pipeline),
            addr_of_mut!(result.pipeline).cast(),
            stride,
            flags,
        )?;
        device.get_query_pool_results(
            self.mesh_shader,
            0,
            1,
            size_of_val(&result.mesh_shader),
            addr_of_mut!(result.mesh_shader).cast(),
            stride,
            flags,
        )?;
        Ok(result)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Statistics {
    pub pipeline: Pipeline,
    pub mesh_shader: MeshShader,
}

#[repr(C)]
#[derive(Debug)]
pub struct Pipeline {
    pub fragment_shader_invocations: u64,
    pub compute_shader_invocations: u64,
    pub task_shader_invocations: u64,
    pub mesh_shader_invocations: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshShader {
    pub primitives_generated: u64,
}
