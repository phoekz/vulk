use super::*;

//
// Demo
//

pub struct Demo {
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    blas: Blas,
    tlas: Tlas,
    render_image: RenderImage,
    stats: Stats,
    descriptors: Descriptors,
    pipeline: Pipeline,
    sbt: Sbt,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "raytracing";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let scene = Scene::create();
        let blas = Blas::create(gpu, &BlasCreateInfo { scene: &scene })?;
        let tlas = Tlas::create(
            gpu,
            &TlasCreateInfo {
                scene: &scene,
                blas: &blas,
            },
        )?;
        let render_image = RenderImage::create(gpu, &RenderImageCreateInfo {})?;
        let stats = Stats::create(gpu, &StatsCreateInfo {})?;
        let descriptors = Descriptors::create(
            gpu,
            &DescriptorsCreateInfo {
                render_image: &render_image,
                tlas: &tlas,
                stats: &stats,
            },
        )?;
        let pipeline = Pipeline::create(
            gpu,
            &PipelineCreateInfo {
                descriptors: &descriptors,
            },
        )?;
        let sbt = Sbt::create(
            gpu,
            &SbtCreateInfo {
                pipeline: &pipeline,
            },
        )?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;

        Ok(Self {
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
            blas,
            tlas,
            render_image,
            stats,
            descriptors,
            pipeline,
            sbt,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        dispatch(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(&gpu.device);
        state.sbt.destroy(gpu);
        state.pipeline.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.stats.destroy(gpu);
        state.render_image.destroy(gpu);
        state.tlas.destroy(gpu);
        state.blas.destroy(gpu);
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
        Ok(())
    }
}

//
// Scene
//

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: Vec3,
}

struct Scene {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    transform_data: Vec<vk::TransformMatrixKHR>,
    triangle_count: u32,
}

impl Scene {
    fn create() -> Self {
        let vertex_data = vec![
            Vertex {
                position: Vec3::new(1.0, 1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(-1.0, 1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(0.0, -1.0, 0.0),
            },
        ];
        let index_data = vec![0, 1, 2];
        let transform_data = vec![vk::TransformMatrixKHR {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }];

        Self {
            vertex_data,
            index_data,
            transform_data,
            triangle_count: 1,
        }
    }
}

//
// Bottom-level acceleration structure
//

struct BlasCreateInfo<'a> {
    scene: &'a Scene,
}

struct Blas {
    blas: vk::AccelerationStructureKHR,
    blas_buffer: vkx::BufferDedicatedResource,
    blas_address: vk::DeviceAddress,
}

impl GpuResource for Blas {
    type CreateInfo<'a> = BlasCreateInfo<'a>;

    unsafe fn create(
        gpu @ Gpu { device, .. }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        // Buffers
        let (vertex_buffer, index_buffer, transform_buffer) = {
            type VertexType = Vertex;
            type IndexType = u32;
            type TransformType = vk::TransformMatrixKHR;

            let mut vertex_buffer = vkx::BufferDedicatedTransfer::create(
                &gpu.physical_device,
                &gpu.device,
                vkx::BufferCreator::new(
                    (size_of::<VertexType>() * create_info.scene.vertex_data.len()) as _,
                    vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR,
                ),
                vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
            )?;
            let mut index_buffer = vkx::BufferDedicatedTransfer::create(
                &gpu.physical_device,
                &gpu.device,
                vkx::BufferCreator::new(
                    (size_of::<IndexType>() * create_info.scene.index_data.len()) as _,
                    vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR,
                ),
                vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
            )?;
            let mut transform_buffer = vkx::BufferDedicatedTransfer::create(
                &gpu.physical_device,
                &gpu.device,
                vkx::BufferCreator::new(
                    (size_of::<TransformType>() * create_info.scene.transform_data.len()) as _,
                    vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR,
                ),
                vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
            )?;
            vertex_buffer
                .memory_mut()
                .as_mut_slice(create_info.scene.vertex_data.len())
                .copy_from_slice(&create_info.scene.vertex_data);
            index_buffer
                .memory_mut()
                .as_mut_slice(create_info.scene.index_data.len())
                .copy_from_slice(&create_info.scene.index_data);
            transform_buffer
                .memory_mut()
                .as_mut_slice(create_info.scene.transform_data.len())
                .copy_from_slice(&create_info.scene.transform_data);

            (vertex_buffer, index_buffer, transform_buffer)
        };

        // Geometry info.
        let acceleration_structure_geometry_triangles_data_khr =
            vk::AccelerationStructureGeometryTrianglesDataKHR {
                s_type: vk::StructureType::AccelerationStructureGeometryTrianglesDataKHR,
                p_next: null(),
                vertex_format: vk::Format::R32g32b32Sfloat,
                vertex_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: vertex_buffer.memory().device_address(),
                },
                vertex_stride: size_of::<Vertex>() as _,
                max_vertex: create_info.scene.vertex_data.len() as _,
                index_type: vk::IndexType::Uint32,
                index_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: index_buffer.memory().device_address(),
                },
                transform_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: transform_buffer.memory().device_address(),
                },
            };
        let acceleration_structure_geometry_khr = vk::AccelerationStructureGeometryKHR {
            s_type: vk::StructureType::AccelerationStructureGeometryKHR,
            p_next: null(),
            geometry_type: vk::GeometryTypeKHR::TrianglesKHR,
            geometry: vk::AccelerationStructureGeometryDataKHR {
                triangles: acceleration_structure_geometry_triangles_data_khr,
            },
            flags: vk::GeometryFlagBitsKHR::OpaqueKHR.into(),
        };

        // Build sizes.
        let (acceleration_structure_size, build_scratch_size) = {
            let acceleration_structure_build_geometry_info_khr =
                vk::AccelerationStructureBuildGeometryInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildGeometryInfoKHR,
                    p_next: null(),
                    ty: vk::AccelerationStructureTypeKHR::BottomLevelKHR,
                    flags: vk::BuildAccelerationStructureFlagBitsKHR::PreferFastTraceKHR.into(),
                    mode: vk::BuildAccelerationStructureModeKHR::BuildKHR,
                    src_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    dst_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    geometry_count: create_info.scene.transform_data.len() as _,
                    p_geometries: &acceleration_structure_geometry_khr,
                    pp_geometries: null(),
                    scratch_data: zeroed(),
                };
            let mut acceleration_structure_build_sizes_info_khr =
                vk::AccelerationStructureBuildSizesInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildSizesInfoKHR,
                    p_next: null(),
                    acceleration_structure_size: zeroed(),
                    update_scratch_size: zeroed(),
                    build_scratch_size: zeroed(),
                };
            let max_primitive_counts = create_info.scene.transform_data.len() as u32;
            device.get_acceleration_structure_build_sizes_khr(
                vk::AccelerationStructureBuildTypeKHR::DeviceKHR,
                &acceleration_structure_build_geometry_info_khr,
                &max_primitive_counts,
                &mut acceleration_structure_build_sizes_info_khr,
            );
            let acceleration_structure_size =
                acceleration_structure_build_sizes_info_khr.acceleration_structure_size;
            let build_scratch_size = acceleration_structure_build_sizes_info_khr.build_scratch_size;

            (acceleration_structure_size, build_scratch_size)
        };

        // Buffer.
        let blas_buffer = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                acceleration_structure_size as _,
                vk::BufferUsageFlagBits::AccelerationStructureStorageKHR,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;

        // Create.
        let blas = {
            let acceleration_structure_create_info_khr = vk::AccelerationStructureCreateInfoKHR {
                s_type: vk::StructureType::AccelerationStructureCreateInfoKHR,
                p_next: null(),
                create_flags: vk::AccelerationStructureCreateFlagsKHR::empty(),
                buffer: blas_buffer.buffer_handle(),
                offset: 0,
                size: acceleration_structure_size as _,
                ty: vk::AccelerationStructureTypeKHR::BottomLevelKHR,
                device_address: 0,
            };
            gpu.device
                .create_acceleration_structure_khr(&acceleration_structure_create_info_khr)?
        };

        // Scratch buffer.
        let blas_scratch_buffer = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                build_scratch_size as _,
                vk::BufferUsageFlagBits::StorageBuffer,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;

        // Build.
        {
            let command_buffer = vkx::CommandBuffer::create(device)?;
            let command_buffer_done = vkx::TimelineSemaphore::create(device, 0)?;
            command_buffer.begin(device)?;
            command_buffer.build_acceleration_structures(
                device,
                vk::AccelerationStructureTypeKHR::BottomLevelKHR,
                blas,
                create_info.scene.transform_data.len() as u32 as _,
                &acceleration_structure_geometry_khr,
                create_info.scene.triangle_count,
                &blas_scratch_buffer,
            );
            command_buffer.end(device)?;
            vkx::queue_submit(
                device,
                &command_buffer,
                &[],
                &[command_buffer_done.submit_info(1, vk::PipelineStageFlagBits2::AllCommands)],
            )?;
            command_buffer_done.wait(device, 1, u64::MAX)?;
            command_buffer_done.destroy(device);
            command_buffer.destroy(device);
        }

        // Device address.
        //
        // Note: this address can be different from `blas_buffer.device_address`.
        //
        // Spec: The acceleration structure device address may be different from
        // the buffer device address corresponding to the acceleration
        // structure’s start offset in its storage buffer for acceleration
        // structure types other than VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR.
        let blas_address = {
            let acceleration_structure_device_address_info_khr =
                vk::AccelerationStructureDeviceAddressInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureDeviceAddressInfoKHR,
                    p_next: null(),
                    acceleration_structure: blas,
                };
            device.get_acceleration_structure_device_address_khr(
                &acceleration_structure_device_address_info_khr,
            )
        };

        // Cleanup.
        vertex_buffer.destroy(&gpu.device);
        index_buffer.destroy(&gpu.device);
        transform_buffer.destroy(&gpu.device);
        blas_scratch_buffer.destroy(&gpu.device);

        Ok(Self {
            blas,
            blas_buffer,
            blas_address,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.blas_buffer.destroy(&gpu.device);
        gpu.device.destroy_acceleration_structure_khr(self.blas);
    }
}

//
// Top-level acceleration structure
//

struct TlasCreateInfo<'a> {
    scene: &'a Scene,
    blas: &'a Blas,
}

struct Tlas {
    tlas: vk::AccelerationStructureKHR,
    tlas_buffer: vkx::BufferDedicatedResource,
    tlas_descriptor: vkx::Descriptor,
}

impl GpuResource for Tlas {
    type CreateInfo<'a> = TlasCreateInfo<'a>;

    unsafe fn create(
        gpu @ Gpu { device, .. }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        // Instance.
        let instance_buffer = {
            let transform_data = vk::TransformMatrixKHR {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                ],
            };
            let instance_custom_index24 = 0;
            let mask8 = 0xff;
            let instance_shader_binding_table_record_offset24 = 0;
            let flags8: u32 =
                transmute(vk::GeometryInstanceFlagBitsKHR::TriangleFacingCullDisableKHR);
            let instance = vk::AccelerationStructureInstanceKHR {
                transform: transform_data,
                instance_custom_index24_and_mask8: instance_custom_index24 | mask8 << 24,
                instance_shader_binding_table_record_offset24_and_flags8:
                    instance_shader_binding_table_record_offset24 | flags8 << 24,
                acceleration_structure_reference: create_info.blas.blas_address,
            };

            let mut instance_buffer = vkx::BufferDedicatedTransfer::create(
                &gpu.physical_device,
                &gpu.device,
                vkx::BufferCreator::new(
                    size_of::<vk::AccelerationStructureInstanceKHR>() as _,
                    vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR,
                ),
                vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
            )?;
            instance_buffer
                .memory_mut()
                .as_mut_slice(1)
                .copy_from_slice(&[instance]);

            instance_buffer
        };

        // Geometry.
        let acceleration_structure_geometry_instances_data_khr =
            vk::AccelerationStructureGeometryInstancesDataKHR {
                s_type: vk::StructureType::AccelerationStructureGeometryInstancesDataKHR,
                p_next: null(),
                array_of_pointers: vk::FALSE,
                data: vk::DeviceOrHostAddressConstKHR {
                    device_address: instance_buffer.memory().device_address(),
                },
            };
        let acceleration_structure_geometry_khr = vk::AccelerationStructureGeometryKHR {
            s_type: vk::StructureType::AccelerationStructureGeometryKHR,
            p_next: null(),
            geometry_type: vk::GeometryTypeKHR::InstancesKHR,
            geometry: vk::AccelerationStructureGeometryDataKHR {
                instances: acceleration_structure_geometry_instances_data_khr,
            },
            flags: vk::GeometryFlagBitsKHR::OpaqueKHR.into(),
        };

        // Build sizes.
        let (acceleration_structure_size, build_scratch_size) = {
            let acceleration_structure_build_geometry_info_khr =
                vk::AccelerationStructureBuildGeometryInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildGeometryInfoKHR,
                    p_next: null(),
                    ty: vk::AccelerationStructureTypeKHR::TopLevelKHR,
                    flags: vk::BuildAccelerationStructureFlagBitsKHR::PreferFastTraceKHR.into(),
                    mode: vk::BuildAccelerationStructureModeKHR::BuildKHR,
                    src_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    dst_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    geometry_count: create_info.scene.transform_data.len() as u32 as _,
                    p_geometries: &acceleration_structure_geometry_khr,
                    pp_geometries: null(),
                    scratch_data: zeroed(),
                };
            let mut acceleration_structure_build_sizes_info_khr =
                vk::AccelerationStructureBuildSizesInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildSizesInfoKHR,
                    p_next: null(),
                    acceleration_structure_size: zeroed(),
                    update_scratch_size: zeroed(),
                    build_scratch_size: zeroed(),
                };
            let max_primitive_counts = create_info.scene.transform_data.len() as u32;
            device.get_acceleration_structure_build_sizes_khr(
                vk::AccelerationStructureBuildTypeKHR::DeviceKHR,
                &acceleration_structure_build_geometry_info_khr,
                &max_primitive_counts,
                &mut acceleration_structure_build_sizes_info_khr,
            );
            let acceleration_structure_size =
                acceleration_structure_build_sizes_info_khr.acceleration_structure_size;
            let build_scratch_size = acceleration_structure_build_sizes_info_khr.build_scratch_size;

            (acceleration_structure_size, build_scratch_size)
        };

        // Buffer.
        let tlas_buffer = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                acceleration_structure_size as _,
                vk::BufferUsageFlagBits::AccelerationStructureStorageKHR,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;

        // Create.
        let tlas = {
            let acceleration_structure_create_info_khr = vk::AccelerationStructureCreateInfoKHR {
                s_type: vk::StructureType::AccelerationStructureCreateInfoKHR,
                p_next: null(),
                create_flags: vk::AccelerationStructureCreateFlagsKHR::empty(),
                buffer: tlas_buffer.buffer_handle(),
                offset: 0,
                size: acceleration_structure_size as _,
                ty: vk::AccelerationStructureTypeKHR::TopLevelKHR,
                device_address: 0,
            };
            gpu.device
                .create_acceleration_structure_khr(&acceleration_structure_create_info_khr)?
        };

        // Scratch buffer.
        let tlas_scratch_buffer = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                build_scratch_size as _,
                vk::BufferUsageFlagBits::StorageBuffer,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;

        // Build.
        {
            let command_buffer = vkx::CommandBuffer::create(device)?;
            let command_buffer_done = vkx::TimelineSemaphore::create(device, 0)?;
            command_buffer.begin(device)?;
            command_buffer.build_acceleration_structures(
                device,
                vk::AccelerationStructureTypeKHR::TopLevelKHR,
                tlas,
                create_info.scene.transform_data.len() as u32 as _,
                &acceleration_structure_geometry_khr,
                create_info.scene.transform_data.len() as u32 as _,
                &tlas_scratch_buffer,
            );
            command_buffer.end(device)?;
            vkx::queue_submit(
                device,
                &command_buffer,
                &[],
                &[command_buffer_done.submit_info(1, vk::PipelineStageFlagBits2::AllCommands)],
            )?;
            command_buffer_done.wait(device, 1, u64::MAX)?;
            command_buffer_done.destroy(device);
            command_buffer.destroy(device);
        }

        // Device address.
        let tlas_address = {
            let acceleration_structure_device_address_info_khr =
                vk::AccelerationStructureDeviceAddressInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureDeviceAddressInfoKHR,
                    p_next: null(),
                    acceleration_structure: tlas,
                };
            device.get_acceleration_structure_device_address_khr(
                &acceleration_structure_device_address_info_khr,
            )
        };

        // Descriptor.
        let tlas_descriptor = vkx::Descriptor::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::DescriptorCreateInfo::AccelerationStructure(tlas_address),
        );

        // Cleanup.
        instance_buffer.destroy(&gpu.device);
        tlas_scratch_buffer.destroy(&gpu.device);

        Ok(Self {
            tlas,
            tlas_buffer,
            tlas_descriptor,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.tlas_buffer.destroy(&gpu.device);
        gpu.device.destroy_acceleration_structure_khr(self.tlas);
    }
}

//
// Render image
//

struct RenderImageCreateInfo {}

struct RenderImage {
    image: vkx::ImageDedicatedResource,
}

impl GpuResource for RenderImage {
    type CreateInfo<'a> = RenderImageCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let image = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                vk::ImageUsageFlagBits::Storage | vk::ImageUsageFlagBits::TransferSrc,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        Ok(Self { image })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.image.destroy(&gpu.device);
    }
}

//
// Stats
//

#[repr(C)]
#[derive(Debug)]
struct StatCounters {
    rays: u64,
    hits: u64,
    misses: u64,
}

struct StatsCreateInfo {}

struct Stats {
    counters: vkx::BufferDedicatedResource,
}

impl GpuResource for Stats {
    type CreateInfo<'a> = StatsCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let counters = vkx::BufferDedicatedResource::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                size_of::<StatCounters>() as _,
                vk::BufferUsageFlagBits::StorageBuffer,
            ),
            vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
        )?;
        Ok(Self { counters })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.counters.destroy(&gpu.device);
    }
}

//
// Descriptors
//

struct DescriptorsCreateInfo<'a> {
    render_image: &'a RenderImage,
    tlas: &'a Tlas,
    stats: &'a Stats,
}

struct Descriptors {
    storage: vkx::DescriptorStorage,
}

impl GpuResource for Descriptors {
    type CreateInfo<'a> = DescriptorsCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Stage flags.
        let stages = vk::ShaderStageFlagBits::RaygenKHR
            | vk::ShaderStageFlagBits::MissKHR
            | vk::ShaderStageFlagBits::ClosestHitKHR;

        // Descriptor storage.
        let storage = vkx::DescriptorStorage::create(
            &gpu.physical_device,
            &gpu.device,
            &[
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageImage,
                    stages,
                    descriptors: &[create_info.render_image.image.descriptor()],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::AccelerationStructureKHR,
                    stages,
                    descriptors: &[create_info.tlas.tlas_descriptor],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageBuffer,
                    stages,
                    descriptors: &[create_info.stats.counters.descriptor()],
                },
            ],
            Some(vk::PushConstantRange {
                stage_flags: stages,
                offset: 0,
                size: (2 * size_of::<Mat4>()) as _,
            }),
        )?;

        Ok(Self { storage })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.storage.destroy(&gpu.device);
    }
}

//
// Raytracing pipeline
//

struct PipelineCreateInfo<'a> {
    descriptors: &'a Descriptors,
}

struct Pipeline {
    pipeline: vk::Pipeline,
    shaders: Vec<vk::ShaderModule>,
    groups: Vec<vk::RayTracingShaderGroupCreateInfoKHR>,
}

impl GpuResource for Pipeline {
    type CreateInfo<'a> = PipelineCreateInfo<'a>;

    unsafe fn create(Gpu { device, .. }: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Shader compiler
        let mut compiler = vkx::ShaderCompiler::new()?;

        // Includes.
        compiler.include(
            "common.glsl",
            r#"
                #extension GL_EXT_ray_tracing : require
                #extension GL_EXT_scalar_block_layout : require
                #extension GL_EXT_shader_atomic_int64 : require
                #extension GL_EXT_shader_explicit_arithmetic_types_int64 : require

                layout(scalar, push_constant) uniform PushBuffer {
                    mat4 view_inverse;
                    mat4 projection_inverse;
                };
                layout(binding = 0, rgba8) uniform image2D render_image;
                layout(binding = 1) uniform accelerationStructureEXT tlas;
                struct Counters {
                    uint64_t rays;
                    uint64_t hits;
                    uint64_t misses;
                };
                layout(scalar, binding = 2) buffer Stats { Counters counters; };
            "#,
        );

        // Compile shaders.
        let raygen_spirv = compiler.compile(
            vkx::ShaderType::Raygen,
            "raygen_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(location = 0) rayPayloadEXT vec3 hitValue;

                void main() {
                    vec2 pixel_center = vec2(gl_LaunchIDEXT.xy) + 0.5;
                    pixel_center = pixel_center / vec2(gl_LaunchSizeEXT.xy);
                    pixel_center = 2.0 * pixel_center - 1.0;

                    vec4 origin = view_inverse * vec4(0.0, 0.0, 0.0, 1.0);
                    vec4 target = projection_inverse * vec4(pixel_center, 1.0, 1.0);
                    vec4 direction = view_inverse * vec4(normalize(target.xyz), 0.0);
                    float tmin = 0.001;
                    float tmax = 1000.0;

                    hitValue = vec3(0.0);
                    traceRayEXT(
                        tlas,                   // accelerationStructureEXT topLevel
                        gl_RayFlagsOpaqueEXT,   // uint rayFlags
                        0xff,                   // uint cullMask
                        0,                      // uint sbtRecordOffset
                        0,                      // uint sbtRecordStride
                        0,                      // uint missIndex
                        origin.xyz,             // vec3 origin
                        tmin,                   // float Tmin
                        direction.xyz,          // vec3 direction
                        tmax,                   // float Tmax
                        0                       // int payload
                    );
                    imageStore(render_image, ivec2(gl_LaunchIDEXT.xy), vec4(hitValue, 1.0));

                    atomicAdd(counters.rays, 1);
                }
            "#,
        )?;
        let miss_spirv = compiler.compile(
            vkx::ShaderType::Miss,
            "miss_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(location = 0) rayPayloadInEXT vec3 hitValue;

                void main() {
                    hitValue = vec3(0.2, 0.2, 0.2);

                    atomicAdd(counters.misses, 1);
                }
            "#,
        )?;
        let closest_hit_spirv = compiler.compile(
            vkx::ShaderType::ClosestHit,
            "closest_hit_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(location = 0) rayPayloadInEXT vec3 hitValue;

                hitAttributeEXT vec2 attribs;

                void main() {
                    vec3 barycentric = vec3(1.0 - attribs.x - attribs.y, attribs.x, attribs.y);
                    hitValue = barycentric;

                    atomicAdd(counters.hits, 1);
                }
            "#,
        )?;

        // Shader modules.
        let raygen_shader =
            device.create_shader_module(&raygen_spirv.shader_module_create_info())?;
        let miss_shader = device.create_shader_module(&miss_spirv.shader_module_create_info())?;
        let closest_hit_shader =
            device.create_shader_module(&closest_hit_spirv.shader_module_create_info())?;
        let shaders = vec![raygen_shader, miss_shader, closest_hit_shader];

        // Ray tracing pipeline.
        let stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                stage: vk::ShaderStageFlagBits::RaygenKHR,
                module: raygen_shader,
                p_name: b"main\0".as_ptr().cast(),
                p_specialization_info: null(),
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                stage: vk::ShaderStageFlagBits::MissKHR,
                module: miss_shader,
                p_name: b"main\0".as_ptr().cast(),
                p_specialization_info: null(),
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                stage: vk::ShaderStageFlagBits::ClosestHitKHR,
                module: closest_hit_shader,
                p_name: b"main\0".as_ptr().cast(),
                p_specialization_info: null(),
            },
        ];
        let groups = vec![
            vk::RayTracingShaderGroupCreateInfoKHR {
                s_type: vk::StructureType::RayTracingShaderGroupCreateInfoKHR,
                p_next: null(),
                ty: vk::RayTracingShaderGroupTypeKHR::GeneralKHR,
                general_shader: 0,
                closest_hit_shader: vk::SHADER_UNUSED_KHR,
                any_hit_shader: vk::SHADER_UNUSED_KHR,
                intersection_shader: vk::SHADER_UNUSED_KHR,
                p_shader_group_capture_replay_handle: null(),
            },
            vk::RayTracingShaderGroupCreateInfoKHR {
                s_type: vk::StructureType::RayTracingShaderGroupCreateInfoKHR,
                p_next: null(),
                ty: vk::RayTracingShaderGroupTypeKHR::GeneralKHR,
                general_shader: 1,
                closest_hit_shader: vk::SHADER_UNUSED_KHR,
                any_hit_shader: vk::SHADER_UNUSED_KHR,
                intersection_shader: vk::SHADER_UNUSED_KHR,
                p_shader_group_capture_replay_handle: null(),
            },
            vk::RayTracingShaderGroupCreateInfoKHR {
                s_type: vk::StructureType::RayTracingShaderGroupCreateInfoKHR,
                p_next: null(),
                ty: vk::RayTracingShaderGroupTypeKHR::TrianglesHitGroupKHR,
                general_shader: vk::SHADER_UNUSED_KHR,
                closest_hit_shader: 2,
                any_hit_shader: vk::SHADER_UNUSED_KHR,
                intersection_shader: vk::SHADER_UNUSED_KHR,
                p_shader_group_capture_replay_handle: null(),
            },
        ];
        let ray_tracing_pipeline_create_info_khr = vk::RayTracingPipelineCreateInfoKHR {
            s_type: vk::StructureType::RayTracingPipelineCreateInfoKHR,
            p_next: null(),
            flags: vk::PipelineCreateFlagBits::DescriptorBufferEXT.into(),
            stage_count: stages.len() as _,
            p_stages: stages.as_ptr(),
            group_count: groups.len() as _,
            p_groups: groups.as_ptr(),
            max_pipeline_ray_recursion_depth: 1,
            p_library_info: null(),
            p_library_interface: null(),
            p_dynamic_state: null(),
            layout: create_info.descriptors.storage.pipeline_layout(),
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
        };
        let mut pipeline = MaybeUninit::zeroed();
        device.create_ray_tracing_pipelines_khr(
            vk::DeferredOperationKHR::null(),
            vk::PipelineCache::null(),
            1,
            &ray_tracing_pipeline_create_info_khr,
            pipeline.as_mut_ptr(),
        )?;
        let pipeline = pipeline.assume_init();

        Ok(Self {
            pipeline,
            shaders,
            groups,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        gpu.device.destroy_pipeline(self.pipeline);
        for &shader in &self.shaders {
            gpu.device.destroy_shader_module(shader);
        }
    }
}

//
// Shader binding table
//

struct SbtCreateInfo<'a> {
    pipeline: &'a Pipeline,
}

struct Sbt {
    shader_binding_tables: Vec<vkx::BufferShaderBindingTable>,
    buffer_allocations: vkx::BufferAllocations,
}

impl GpuResource for Sbt {
    type CreateInfo<'a> = SbtCreateInfo<'a>;

    unsafe fn create(
        Gpu {
            device,
            physical_device,
            ..
        }: &Gpu,
        create_info: &Self::CreateInfo<'_>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        // Get sizes.
        let shader_group_handle_size = physical_device
            .raytracing_pipeline_properties
            .shader_group_handle_size;
        let shader_group_handle_alignment = physical_device
            .raytracing_pipeline_properties
            .shader_group_handle_alignment;
        ensure!(shader_group_handle_size == shader_group_handle_alignment);

        // Buffers.
        let mut buffers = vec![];
        let mut buffer_create_infos = vec![];
        for _ in 0..create_info.pipeline.groups.len() {
            // Buffer.
            let (buffer, buffer_create_info) = vkx::BufferCreator::new(
                u64::from(shader_group_handle_size),
                vk::BufferUsageFlagBits::ShaderBindingTableKHR,
            )
            .create(device)?;
            buffers.push(buffer);
            buffer_create_infos.push(buffer_create_info);
        }

        // Allocation.
        let buffer_allocations = vkx::BufferAllocations::allocate(
            physical_device,
            device,
            &buffers,
            &buffer_create_infos,
            vk::MemoryPropertyFlagBits::HostVisible | vk::MemoryPropertyFlagBits::HostCoherent,
        )?;

        // Shader binding tables.
        let mut shader_binding_tables = vkx::BufferShaderBindingTable::create(
            &buffers,
            &buffer_create_infos,
            buffer_allocations.allocations(),
        )?;
        for first_group in 0..create_info.pipeline.groups.len() {
            device.get_ray_tracing_shader_group_handles_khr(
                create_info.pipeline.pipeline,
                first_group as _,
                1,
                shader_group_handle_size as _,
                shader_binding_tables[first_group].memory_mut().as_mut_ptr(),
            )?;
        }

        Ok(Self {
            shader_binding_tables,
            buffer_allocations,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        for shader_binding_table in self.shader_binding_tables {
            shader_binding_table.destroy(&gpu.device);
        }
        self.buffer_allocations.free(&gpu.device);
    }
}

//
// Execute
//

#[allow(dead_code)]
struct PushConstants {
    view_inverse: Mat4,
    projection_inverse: Mat4,
}

fn push_constants() -> PushConstants {
    let fov_y_radians = f32::to_radians(60.0);
    let aspect_ratio = DEFAULT_RENDER_TARGET_WIDTH as f32 / DEFAULT_RENDER_TARGET_HEIGHT as f32;
    let z_near = 0.001;
    let z_far = 1000.0;
    let projection = Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far);
    let projection_inverse = projection.inverse();

    let eye = Vec3::new(0.0, 0.0, -2.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let view = Mat4::look_at_rh(eye, center, up);
    let view_inverse = view.inverse();
    PushConstants {
        view_inverse,
        projection_inverse,
    }
}

unsafe fn dispatch(
    gpu @ Gpu { device, .. }: &Gpu,
    Demo {
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        render_image,
        stats,
        descriptors,
        pipeline,
        sbt,
        output,
        ..
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Record commands.
    command_buffer.begin(device)?;
    command_buffer.write_timestamp(device, timestamps, 0);
    command_buffer.begin_statistics(device, statistics);
    command_buffer.image_barrier(
        device,
        &render_image.image,
        vk::PipelineStageFlagBits2::None,
        vk::AccessFlags2::empty(),
        vk::PipelineStageFlagBits2::RayTracingShaderKHR,
        vk::AccessFlagBits2::ShaderWrite,
        vk::ImageLayout::Undefined,
        vk::ImageLayout::General,
    );
    command_buffer.bind_descriptor_storage(
        device,
        &descriptors.storage,
        vk::PipelineBindPoint::RayTracingKHR,
    );
    command_buffer.push_constants(device, &descriptors.storage, &push_constants())?;
    command_buffer.bind_ray_tracing_pipeline(device, pipeline.pipeline);
    command_buffer.trace_rays(
        &gpu.physical_device,
        &gpu.device,
        Some(&sbt.shader_binding_tables[0]),
        Some(&sbt.shader_binding_tables[1]),
        Some(&sbt.shader_binding_tables[2]),
        None,
        render_image.image.width(),
        render_image.image.height(),
    );
    command_buffer.image_barrier(
        device,
        &render_image.image,
        vk::PipelineStageFlagBits2::RayTracingShaderKHR,
        vk::AccessFlagBits2::ShaderWrite,
        vk::PipelineStageFlagBits2::Copy,
        vk::AccessFlagBits2::TransferRead,
        vk::ImageLayout::General,
        vk::ImageLayout::TransferSrcOptimal,
    );
    command_buffer.copy_image_to_buffer(device, &render_image.image, (&output.buffer, 0));
    command_buffer.end_statistics(device, statistics);
    command_buffer.write_timestamp(device, timestamps, 1);
    command_buffer.end(device)?;

    // Submit & wait.
    vkx::queue_submit(
        device,
        command_buffer,
        &[],
        &[command_buffer_done.submit_info(1, vk::PipelineStageFlagBits2::AllCommands)],
    )?;
    println!("waiting");
    command_buffer_done.wait(device, 1, u64::MAX)?;
    println!("done");

    // Query results.
    {
        let timestamp_differences = timestamps.get_differences(device)?[0];
        let statistics = statistics.get_statistics(device)?;
        println!("Rendering took: {timestamp_differences:?}");
        println!("Rendering statistics: {statistics:?}");

        let stats = &stats.counters.memory().as_slice::<StatCounters>(1)[0];
        info!("Raytracing statistics: {stats:?}",);
        ensure!(
            stats.rays == u64::from(DEFAULT_RENDER_TARGET_WIDTH * DEFAULT_RENDER_TARGET_HEIGHT)
        );
        ensure!(stats.rays == stats.hits + stats.misses);
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
