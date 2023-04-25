use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    blas: Blas,
    tlas: Tlas,
    render_image: RenderImage,
    stats: Stats,
    descriptors: Descriptors,
    pipeline: Pipeline,
    sbt: Sbt,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "raytracing";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
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
        let output = Output::create(gpu, &OutputCreateInfo {})?;

        Ok(Self {
            commands,
            queries,
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
        state.output.destroy(gpu);
        state.sbt.destroy(gpu);
        state.pipeline.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.stats.destroy(gpu);
        state.render_image.destroy(gpu);
        state.tlas.destroy(gpu);
        state.blas.destroy(gpu);
        state.queries.destroy(gpu);
        state.commands.destroy(gpu);
        Ok(())
    }
}

//
// Types
//

type VertexBuffer = resource::Buffer<Vertex>;
type IndexBuffer = resource::Buffer<u32>;
type TransformBuffer = resource::Buffer<vk::TransformMatrixKHR>;
type AccelerationStructureBuffer = resource::Buffer<u8>;
type AccelerationStructureScratchBuffer = resource::Buffer<u8>;
type ShaderBindingTableBuffer = resource::Buffer<u8>;

//
// Scene
//

#[repr(C)]
struct Vertex {
    position: Vec3,
}

struct Scene {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    transform_data: Vec<vk::TransformMatrixKHR>,
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
        }
    }
}

//
// Bottom-level acceleration structure
//

struct BlasCreateInfo<'a> {
    scene: &'a Scene,
}

#[derive(Debug)]
struct Blas {
    blas: vk::AccelerationStructureKHR,
    blas_buffer: AccelerationStructureBuffer,
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
            let vertex_buffer = VertexBuffer::create(
                gpu,
                &resource::BufferCreateInfo {
                    size: create_info.scene.vertex_data.len() as _,
                    usage: vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR
                        .into(),
                    property_flags: vk::MemoryPropertyFlagBits::HostVisible
                        | vk::MemoryPropertyFlagBits::HostCoherent,
                },
            )?;
            let index_buffer = IndexBuffer::create(
                gpu,
                &resource::BufferCreateInfo {
                    size: create_info.scene.index_data.len() as _,
                    usage: vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR
                        .into(),
                    property_flags: vk::MemoryPropertyFlagBits::HostVisible
                        | vk::MemoryPropertyFlagBits::HostCoherent,
                },
            )?;
            let transform_buffer = TransformBuffer::create(
                gpu,
                &resource::BufferCreateInfo {
                    size: create_info.scene.transform_data.len() as _,
                    usage: vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR
                        .into(),
                    property_flags: vk::MemoryPropertyFlagBits::HostVisible
                        | vk::MemoryPropertyFlagBits::HostCoherent,
                },
            )?;
            vertex_buffer.ptr.copy_from_nonoverlapping(
                create_info.scene.vertex_data.as_ptr(),
                create_info.scene.vertex_data.len(),
            );
            index_buffer.ptr.copy_from_nonoverlapping(
                create_info.scene.index_data.as_ptr(),
                create_info.scene.index_data.len(),
            );
            transform_buffer.ptr.copy_from_nonoverlapping(
                create_info.scene.transform_data.as_ptr(),
                create_info.scene.transform_data.len(),
            );

            (vertex_buffer, index_buffer, transform_buffer)
        };

        // Geometry info.
        let acceleration_structure_geometry_triangles_data_khr =
            vk::AccelerationStructureGeometryTrianglesDataKHR {
                s_type: vk::StructureType::AccelerationStructureGeometryTrianglesDataKHR,
                p_next: null(),
                vertex_format: vk::Format::R32g32b32Sfloat,
                vertex_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: vertex_buffer.device_address,
                },
                vertex_stride: size_of::<Vertex>() as _,
                max_vertex: create_info.scene.vertex_data.len() as _,
                index_type: vk::IndexType::Uint32,
                index_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: index_buffer.device_address,
                },
                transform_data: vk::DeviceOrHostAddressConstKHR {
                    device_address: transform_buffer.device_address,
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
        let blas_buffer = AccelerationStructureBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: acceleration_structure_size as _,
                usage: vk::BufferUsageFlagBits::AccelerationStructureStorageKHR.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;

        // Create.
        let blas = {
            let acceleration_structure_create_info_khr = vk::AccelerationStructureCreateInfoKHR {
                s_type: vk::StructureType::AccelerationStructureCreateInfoKHR,
                p_next: null(),
                create_flags: vk::AccelerationStructureCreateFlagsKHR::empty(),
                buffer: blas_buffer.buffer,
                offset: 0,
                size: acceleration_structure_size as _,
                ty: vk::AccelerationStructureTypeKHR::BottomLevelKHR,
                device_address: 0,
            };
            gpu.device
                .create_acceleration_structure_khr(&acceleration_structure_create_info_khr)?
        };

        // Scratch buffer.
        let blas_scratch_buffer = AccelerationStructureScratchBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: build_scratch_size as _,
                usage: vk::BufferUsageFlagBits::StorageBuffer.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;

        // Build.
        {
            let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
            let cmd = commands.begin(gpu)?;
            let acceleration_structure_build_geometry_info_khr =
                vk::AccelerationStructureBuildGeometryInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildGeometryInfoKHR,
                    p_next: null(),
                    ty: vk::AccelerationStructureTypeKHR::BottomLevelKHR,
                    flags: vk::BuildAccelerationStructureFlagBitsKHR::PreferFastTraceKHR.into(),
                    mode: vk::BuildAccelerationStructureModeKHR::BuildKHR,
                    src_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    dst_acceleration_structure: blas,
                    geometry_count: create_info.scene.transform_data.len() as u32 as _,
                    p_geometries: &acceleration_structure_geometry_khr,
                    pp_geometries: null(),
                    scratch_data: vk::DeviceOrHostAddressKHR {
                        device_address: blas_scratch_buffer.device_address,
                    },
                };
            let acceleration_structure_build_range_info_khr =
                vk::AccelerationStructureBuildRangeInfoKHR {
                    primitive_count: create_info.scene.transform_data.len() as u32 as _,
                    primitive_offset: 0,
                    first_vertex: 0,
                    transform_offset: 0,
                };
            device.cmd_build_acceleration_structures_khr(
                cmd,
                1,
                &acceleration_structure_build_geometry_info_khr,
                [addr_of!(acceleration_structure_build_range_info_khr)].as_ptr(),
            );
            commands.end(gpu)?;
            commands.submit_and_wait(gpu, vk::PipelineStageFlagBits2::AllCommands.into())?;
            commands.destroy(gpu);
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
        vertex_buffer.destroy(gpu);
        index_buffer.destroy(gpu);
        transform_buffer.destroy(gpu);
        blas_scratch_buffer.destroy(gpu);

        Ok(Self {
            blas,
            blas_buffer,
            blas_address,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.blas_buffer.destroy(gpu);
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

#[derive(Debug)]
struct Tlas {
    tlas: vk::AccelerationStructureKHR,
    tlas_buffer: AccelerationStructureBuffer,
    tlas_descriptor: descriptor::Descriptor,
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
        // Types.
        type InstanceBuffer = resource::Buffer<vk::AccelerationStructureInstanceKHR>;

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

            let instance_buffer = InstanceBuffer::create(
                gpu,
                &resource::BufferCreateInfo {
                    size: 1,
                    usage: vk::BufferUsageFlagBits::AccelerationStructureBuildInputReadOnlyKHR
                        .into(),
                    property_flags: vk::MemoryPropertyFlagBits::HostVisible
                        | vk::MemoryPropertyFlagBits::HostCoherent,
                },
            )?;
            instance_buffer
                .ptr
                .copy_from_nonoverlapping(addr_of!(instance), 1);

            instance_buffer
        };

        // Geometry.
        let acceleration_structure_geometry_instances_data_khr =
            vk::AccelerationStructureGeometryInstancesDataKHR {
                s_type: vk::StructureType::AccelerationStructureGeometryInstancesDataKHR,
                p_next: null(),
                array_of_pointers: vk::FALSE,
                data: vk::DeviceOrHostAddressConstKHR {
                    device_address: instance_buffer.device_address,
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
        let tlas_buffer = AccelerationStructureBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: acceleration_structure_size as _,
                usage: vk::BufferUsageFlagBits::AccelerationStructureStorageKHR.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;

        // Create.
        let tlas = {
            let acceleration_structure_create_info_khr = vk::AccelerationStructureCreateInfoKHR {
                s_type: vk::StructureType::AccelerationStructureCreateInfoKHR,
                p_next: null(),
                create_flags: vk::AccelerationStructureCreateFlagsKHR::empty(),
                buffer: tlas_buffer.buffer,
                offset: 0,
                size: acceleration_structure_size as _,
                ty: vk::AccelerationStructureTypeKHR::TopLevelKHR,
                device_address: 0,
            };
            gpu.device
                .create_acceleration_structure_khr(&acceleration_structure_create_info_khr)?
        };

        // Scratch buffer.
        let tlas_scratch_buffer = AccelerationStructureScratchBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: build_scratch_size as _,
                usage: vk::BufferUsageFlagBits::StorageBuffer.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;

        // Build.
        {
            let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
            let cmd = commands.begin(gpu)?;
            let acceleration_structure_build_geometry_info_khr =
                vk::AccelerationStructureBuildGeometryInfoKHR {
                    s_type: vk::StructureType::AccelerationStructureBuildGeometryInfoKHR,
                    p_next: null(),
                    ty: vk::AccelerationStructureTypeKHR::TopLevelKHR,
                    flags: vk::BuildAccelerationStructureFlagBitsKHR::PreferFastTraceKHR.into(),
                    mode: vk::BuildAccelerationStructureModeKHR::BuildKHR,
                    src_acceleration_structure: vk::AccelerationStructureKHR::null(),
                    dst_acceleration_structure: tlas,
                    geometry_count: create_info.scene.transform_data.len() as u32 as _,
                    p_geometries: &acceleration_structure_geometry_khr,
                    pp_geometries: null(),
                    scratch_data: vk::DeviceOrHostAddressKHR {
                        device_address: tlas_scratch_buffer.device_address,
                    },
                };
            let acceleration_structure_build_range_info_khr =
                vk::AccelerationStructureBuildRangeInfoKHR {
                    primitive_count: create_info.scene.transform_data.len() as u32 as _,
                    primitive_offset: 0,
                    first_vertex: 0,
                    transform_offset: 0,
                };
            device.cmd_build_acceleration_structures_khr(
                cmd,
                1,
                &acceleration_structure_build_geometry_info_khr,
                [addr_of!(acceleration_structure_build_range_info_khr)].as_ptr(),
            );
            commands.end(gpu)?;
            commands.submit_and_wait(gpu, vk::PipelineStageFlagBits2::AllCommands.into())?;
            commands.destroy(gpu);
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
        let tlas_descriptor =
            descriptor::Descriptor::create_acceleration_structure(gpu, tlas_address);

        // Cleanup.
        instance_buffer.destroy(gpu);
        tlas_scratch_buffer.destroy(gpu);

        Ok(Self {
            tlas,
            tlas_buffer,
            tlas_descriptor,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.tlas_buffer.destroy(gpu);
        gpu.device.destroy_acceleration_structure_khr(self.tlas);
    }
}

//
// Render image
//

struct RenderImageCreateInfo {}

struct RenderImage {
    image: resource::Image2d,
}

impl GpuResource for RenderImage {
    type CreateInfo<'a> = RenderImageCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let image = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: vk::SampleCountFlagBits::Count1,
                usage: vk::ImageUsageFlagBits::Storage | vk::ImageUsageFlagBits::TransferSrc,
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;
        Ok(Self { image })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.image.destroy(gpu);
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
    counters: resource::Buffer<StatCounters>,
}

impl GpuResource for Stats {
    type CreateInfo<'a> = StatsCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let counters = resource::Buffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: 1,
                usage: vk::BufferUsageFlagBits::StorageBuffer.into(),
                property_flags: vk::MemoryPropertyFlagBits::HostVisible
                    | vk::MemoryPropertyFlagBits::HostCoherent,
            },
        )?;
        Ok(Self { counters })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.counters.destroy(gpu);
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
    storage: descriptor::DescriptorStorage,
    pipeline_layout: vk::PipelineLayout,
    push_constant_ranges: vk::PushConstantRange,
}

impl GpuResource for Descriptors {
    type CreateInfo<'a> = DescriptorsCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Stage flags.
        let stage_flags = vk::ShaderStageFlagBits::RaygenKHR
            | vk::ShaderStageFlagBits::MissKHR
            | vk::ShaderStageFlagBits::ClosestHitKHR;

        // Descriptor storage.
        let storage = descriptor::DescriptorStorage::create(
            gpu,
            &descriptor::DescriptorStorageCreateInfo {
                bindings: &[
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::StorageImage,
                        stage_flags,
                        descriptors: &[create_info.render_image.image.descriptor],
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::AccelerationStructureKHR,
                        stage_flags,
                        descriptors: &[create_info.tlas.tlas_descriptor],
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::StorageBuffer,
                        stage_flags,
                        descriptors: &[create_info.stats.counters.descriptor],
                    },
                ],
            },
        )?;

        // Push constants.
        let push_constant_ranges = vk::PushConstantRange {
            stage_flags,
            offset: 0,
            size: (2 * size_of::<Mat4>()) as _,
        };

        // Pipeline layout.
        let pipeline_layout = gpu.device.create_pipeline_layout(
            &(vk::PipelineLayoutCreateInfo {
                s_type: vk::StructureType::PipelineLayoutCreateInfo,
                p_next: null(),
                flags: vk::PipelineLayoutCreateFlags::empty(),
                set_layout_count: 1,
                p_set_layouts: &storage.set_layout(),
                push_constant_range_count: 1,
                p_push_constant_ranges: &push_constant_ranges,
            }),
        )?;

        Ok(Self {
            storage,
            pipeline_layout,
            push_constant_ranges,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        gpu.device.destroy_pipeline_layout(self.pipeline_layout);
        self.storage.destroy(gpu);
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
        let mut compiler = shader::Compiler::new()?;

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
            shader::ShaderType::Raygen,
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
            shader::ShaderType::Miss,
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
            shader::ShaderType::ClosestHit,
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
        let raygen_shader = device.create_shader_module(&vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: raygen_spirv.code.len(),
            p_code: raygen_spirv.code.as_ptr().cast(),
        })?;
        let miss_shader = device.create_shader_module(&vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: miss_spirv.code.len(),
            p_code: miss_spirv.code.as_ptr().cast(),
        })?;
        let closest_hit_shader = device.create_shader_module(&vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: closest_hit_spirv.code.len(),
            p_code: closest_hit_spirv.code.as_ptr().cast(),
        })?;
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
            layout: create_info.descriptors.pipeline_layout,
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
    binding_tables: Vec<ShaderBindingTableBuffer>,
    shader_group_handle_size: vk::DeviceSize,
}

impl GpuResource for Sbt {
    type CreateInfo<'a> = SbtCreateInfo<'a>;

    unsafe fn create(
        gpu @ Gpu {
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

        // Create binding tables.
        let mut binding_tables = vec![];
        for first_group in 0..create_info.pipeline.groups.len() {
            let buffer = ShaderBindingTableBuffer::create(
                gpu,
                &resource::BufferCreateInfo {
                    size: shader_group_handle_size as _,
                    usage: vk::BufferUsageFlagBits::ShaderBindingTableKHR.into(),
                    property_flags: vk::MemoryPropertyFlagBits::HostVisible
                        | vk::MemoryPropertyFlagBits::HostCoherent,
                },
            )?;
            device.get_ray_tracing_shader_group_handles_khr(
                create_info.pipeline.pipeline,
                first_group as _,
                1,
                shader_group_handle_size as _,
                buffer.ptr.cast(),
            )?;
            binding_tables.push(buffer);
        }
        Ok(Self {
            binding_tables,
            shader_group_handle_size: shader_group_handle_size.into(),
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        for binding_table in self.binding_tables {
            binding_table.destroy(gpu);
        }
    }
}

//
// Output
//

struct OutputCreateInfo {}

struct Output {
    buffer: resource::Buffer<u32>,
}

impl GpuResource for Output {
    type CreateInfo<'a> = OutputCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = resource::Buffer::create(
            gpu,
            &resource::BufferCreateInfo {
                size: DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE as _,
                usage: vk::BufferUsageFlagBits::TransferDst.into(),
                property_flags: vk::MemoryPropertyFlagBits::HostVisible.into(),
            },
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.buffer.destroy(gpu);
    }
}

//
// Execute
//

unsafe fn dispatch(
    gpu @ Gpu { device, queue, .. }: &Gpu,
    Demo {
        commands,
        queries,
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
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd, vk::PipelineStageFlagBits2::None.into());

    // Transition render image.
    device.cmd_pipeline_barrier2(
        cmd,
        &vk::DependencyInfo {
            s_type: vk::StructureType::DependencyInfo,
            p_next: null(),
            dependency_flags: vk::DependencyFlags::empty(),
            memory_barrier_count: 0,
            p_memory_barriers: null(),
            buffer_memory_barrier_count: 0,
            p_buffer_memory_barriers: null(),
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                s_type: vk::StructureType::ImageMemoryBarrier2,
                p_next: null(),
                src_stage_mask: vk::PipelineStageFlagBits2::None.into(),
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlagBits2::RayTracingShaderKHR.into(),
                dst_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::General,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_image.image.image,
                subresource_range: render_image.image.subresource_range(),
            },
        },
    );

    // Bind descriptors.
    descriptors.storage.bind(gpu, cmd);
    descriptors.storage.set_offsets(
        gpu,
        cmd,
        vk::PipelineBindPoint::RayTracingKHR,
        descriptors.pipeline_layout,
    );

    // Bind pipeline.
    device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::RayTracingKHR, pipeline.pipeline);

    // Push constants.
    {
        #[allow(dead_code)]
        struct PushBuffer {
            view_inverse: Mat4,
            projection_inverse: Mat4,
        }

        let fov_y_radians = f32::to_radians(60.0);
        let aspect_ratio = render_image.image.width() as f32 / render_image.image.height() as f32;
        let z_near = 0.001;
        let z_far = 1000.0;
        let projection = Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far);
        let projection_inverse = projection.inverse();

        let eye = Vec3::new(0.0, 0.0, -2.0);
        let center = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view = Mat4::look_at_rh(eye, center, up);
        let view_inverse = view.inverse();
        let push_buffer = PushBuffer {
            view_inverse,
            projection_inverse,
        };
        device.cmd_push_constants(
            cmd,
            descriptors.pipeline_layout,
            descriptors.push_constant_ranges.stage_flags,
            descriptors.push_constant_ranges.offset,
            descriptors.push_constant_ranges.size,
            addr_of!(push_buffer).cast(),
        );
    }

    // Trace rays.
    let raygen_sbt = vk::StridedDeviceAddressRegionKHR {
        device_address: sbt.binding_tables[0].device_address,
        stride: sbt.shader_group_handle_size,
        size: sbt.shader_group_handle_size,
    };
    let miss_sbt = vk::StridedDeviceAddressRegionKHR {
        device_address: sbt.binding_tables[1].device_address,
        stride: sbt.shader_group_handle_size,
        size: sbt.shader_group_handle_size,
    };
    let hit_sbt = vk::StridedDeviceAddressRegionKHR {
        device_address: sbt.binding_tables[2].device_address,
        stride: sbt.shader_group_handle_size,
        size: sbt.shader_group_handle_size,
    };
    let callable_sbt: vk::StridedDeviceAddressRegionKHR = zeroed();
    device.cmd_trace_rays_khr(
        cmd,
        &raygen_sbt,
        &miss_sbt,
        &hit_sbt,
        &callable_sbt,
        render_image.image.width(),
        render_image.image.height(),
        1,
    );

    // Transition render image.
    device.cmd_pipeline_barrier2(
        cmd,
        &vk::DependencyInfo {
            s_type: vk::StructureType::DependencyInfo,
            p_next: null(),
            dependency_flags: vk::DependencyFlags::empty(),
            memory_barrier_count: 0,
            p_memory_barriers: null(),
            buffer_memory_barrier_count: 0,
            p_buffer_memory_barriers: null(),
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &vk::ImageMemoryBarrier2 {
                s_type: vk::StructureType::ImageMemoryBarrier2,
                p_next: null(),
                src_stage_mask: vk::PipelineStageFlagBits2::RayTracingShaderKHR.into(),
                src_access_mask: vk::AccessFlagBits2::ShaderWrite.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::Copy.into(),
                dst_access_mask: vk::AccessFlagBits2::TransferRead.into(),
                old_layout: vk::ImageLayout::General,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_image.image.image,
                subresource_range: render_image.image.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_image.image.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_image.image.width(),
                buffer_image_height: render_image.image.height(),
                image_subresource: render_image.image.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_image.image.extent_3d(),
            }),
        }),
    );

    // End queries.
    queries.end(gpu, cmd, vk::PipelineStageFlagBits2::AllTransfer.into());

    // End command buffer.
    commands.end(gpu)?;

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
                stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
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
                flags: vk::SemaphoreWaitFlagBits::Any.into(),
                semaphore_count: semaphores.len() as _,
                p_semaphores: semaphores.as_ptr(),
                p_values: values.as_ptr(),
            }),
            u64::MAX,
        )?;
    }

    // Query results.
    {
        info!("Rendering took {:?}", queries.elapsed(gpu)?);
        info!("Rendering statistics: {:?}", queries.statistics(gpu)?);

        let stats = &std::slice::from_raw_parts(stats.counters.ptr, 1)[0];
        info!("Raytracing statistics: {stats:?}",);
        ensure!(
            stats.rays == u64::from(DEFAULT_RENDER_TARGET_WIDTH * DEFAULT_RENDER_TARGET_HEIGHT)
        );
        ensure!(stats.rays == stats.hits + stats.misses);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_image.image.width();
        let height = render_image.image.height();
        let pixels_byte_size = render_image.image.byte_size();
        let mut pixels = vec![0_u8; pixels_byte_size as _];
        std::ptr::copy_nonoverlapping(
            output.buffer.ptr.cast::<u8>(),
            pixels.as_mut_ptr(),
            pixels_byte_size as _,
        );
        let image = RgbaImage::from_raw(width, height, pixels)
            .context("Creating image from output buffer")?;
        let image_path = work_dir_or_create()?.join(format!("{demo_name}.png"));
        image.save_with_format(&image_path, ImageFormat::Png)?;
        info!("Wrote image to {}", image_path.display());
    }

    Ok(())
}
