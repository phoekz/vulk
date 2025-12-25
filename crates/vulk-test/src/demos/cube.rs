use super::*;

//
// Demo
//

pub struct Demo {
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    textures: Textures,
    descriptors: Descriptors,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "cube";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let textures = Textures::create(gpu, &())?;
        let descriptors = Descriptors::create(
            gpu,
            &DescriptorsCreateInfo {
                textures: &textures,
            },
        )?;
        let shaders = Shaders::create(
            gpu,
            &ShadersCreateInfo {
                descriptors: &descriptors,
            },
        )?;
        let render_targets = RenderTargets::create(gpu, &())?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;
        Ok(Self {
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
            textures,
            descriptors,
            shaders,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(&gpu.device);
        state.render_targets.destroy(gpu);
        state.shaders.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.textures.destroy(gpu);
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
        Ok(())
    }
}

//
// Textures
//

struct Textures {
    images: Vec<vkx::ImageResource>,
    image_allocations: vkx::ImageAllocations,
    samplers: Vec<vkx::SamplerResource>,
}

impl GpuResource for Textures {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, (): &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Generate textures.
        let (image_creators, image_datas) = {
            use palette::{FromColor, Hsl, Srgb};
            use rand::prelude::*;

            let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let mut state = rand_pcg::Pcg64Mcg::seed_from_u64(seed.as_secs());
            let noise = rand::distr::Uniform::new_inclusive(-0.1, 0.1)?;

            let width = 16;
            let height = 16;
            let count = 6;

            let mut image_creators = vec![];
            let mut image_datas = vec![];
            for image_index in 0..count {
                image_creators.push(vkx::ImageCreator::new_2d(
                    width,
                    height,
                    vk::Format::R8g8b8a8Unorm,
                    vk::ImageUsageFlagBits::TransferDst | vk::ImageUsageFlagBits::Sampled,
                ));

                let image_data = {
                    let hue = (image_index as f32 + 0.5) / count as f32;
                    let hsl = Hsl::new(360.0 * hue, 0.75, 0.75);
                    let rgb = Srgb::from_color(hsl);
                    let mut data = vec![0_u8; (4 * width * height) as _];
                    for y in 0..height {
                        for x in 0..width {
                            let r = f32::clamp(rgb.red + noise.sample(&mut state), 0.0, 1.0);
                            let g = f32::clamp(rgb.green + noise.sample(&mut state), 0.0, 1.0);
                            let b = f32::clamp(rgb.blue + noise.sample(&mut state), 0.0, 1.0);

                            let i = (4 * (x + y * width)) as usize;
                            data[i] = f32::round(r * 255.0) as u8;
                            data[i + 1] = f32::round(g * 255.0) as u8;
                            data[i + 2] = f32::round(b * 255.0) as u8;
                            data[i + 3] = 255;
                        }
                    }
                    data
                };
                image_datas.push(image_data);
            }

            (image_creators, image_datas)
        };

        // Image resources.
        let (image_resources, image_allocations) = vkx::ImageResource::create(
            &gpu.physical_device,
            &gpu.device,
            &image_creators,
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;

        // Upload.
        vkx::transfer_resources(
            &gpu.physical_device,
            &gpu.device,
            &[],
            &[],
            &image_resources,
            &image_datas.iter().map(Vec::as_slice).collect::<Vec<_>>(),
        )?;

        // Samplers.
        let sampler_creator = |mag_filter: vk::Filter, min_filter: vk::Filter| {
            vkx::SamplerCreator::new()
                .mag_filter(mag_filter)
                .min_filter(min_filter)
                .mipmap_mode(vk::SamplerMipmapMode::Nearest)
                .address_mode_uvw(vk::SamplerAddressMode::ClampToEdge)
        };
        let sampler_creators = [
            sampler_creator(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_creator(vk::Filter::Linear, vk::Filter::Nearest),
            sampler_creator(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_creator(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_creator(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_creator(vk::Filter::Nearest, vk::Filter::Nearest),
        ];
        let mut samplers = vec![];
        let mut sampler_create_infos = vec![];
        for sampler_creator in sampler_creators {
            let (sampler, sampler_create_info) = sampler_creator.create(&gpu.device)?;
            samplers.push(sampler);
            sampler_create_infos.push(sampler_create_info);
        }
        let samplers = vkx::SamplerResource::create(
            &gpu.physical_device,
            &gpu.device,
            &samplers,
            &sampler_create_infos,
        )?;

        Ok(Self {
            images: image_resources,
            image_allocations,
            samplers,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        for image in self.images {
            image.destroy(&gpu.device);
        }
        self.image_allocations.free(&gpu.device);
        for sampler in self.samplers {
            sampler.destroy(&gpu.device);
        }
    }
}

//
// Descriptors
//

struct DescriptorsCreateInfo<'a> {
    textures: &'a Textures,
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
        // Descriptor storage.
        let stages = vk::ShaderStageFlagBits::Fragment.into();
        let image_descriptors = create_info
            .textures
            .images
            .iter()
            .map(vkx::ImageResource::descriptor)
            .collect::<Vec<_>>();
        let sampler_descriptors = create_info
            .textures
            .samplers
            .iter()
            .map(vkx::SamplerResource::descriptor)
            .collect::<Vec<_>>();
        let storage = vkx::DescriptorStorage::create(
            &gpu.physical_device,
            &gpu.device,
            &[
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::SampledImage,
                    stages,
                    descriptors: &image_descriptors,
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::Sampler,
                    stages,
                    descriptors: &sampler_descriptors,
                },
            ],
            Some(vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlagBits::MeshEXT.into(),
                offset: 0,
                size: size_of::<Mat4>() as _,
            }),
        )?;

        Ok(Self { storage })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.storage.destroy(&gpu.device);
    }
}

//
// Shaders
//

struct ShadersCreateInfo<'a> {
    descriptors: &'a Descriptors,
}

struct Shaders {
    shader: vkx::Shader,
}

impl GpuResource for Shaders {
    type CreateInfo<'a> = ShadersCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Shader compiler
        let mut compiler = vkx::ShaderCompiler::new()?;

        // Includes.
        compiler.include(
            "common.glsl",
            r#"
            #extension GL_EXT_mesh_shader : require
            #extension GL_EXT_nonuniform_qualifier : require
            #extension GL_EXT_scalar_block_layout : require

            struct MeshVertex {
                vec2 texcoord;
            };

            struct MeshPrimitive {
                vec3 normal;
                uint texture_id;
                uint sampler_id;
                };
            "#,
        );

        // Shaders.
        let task_spirv = compiler.compile(
            vkx::ShaderType::Task,
            "task_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                void main() {
                    EmitMeshTasksEXT(1, 1, 1);
                }
            "#,
        )?;
        let mesh_spirv = compiler.compile(
            vkx::ShaderType::Mesh,
            "mesh_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"
                #define MAX_VERTICES 36
                #define MAX_PRIMITIVES 12

                layout(scalar, push_constant) uniform PushBuffer {
                    mat4 transform;
                };
                layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
                layout(triangles, max_vertices = MAX_VERTICES, max_primitives = MAX_PRIMITIVES) out;
                layout(location = 0) out MeshVertex mesh_vertices[];
                layout(location = 1) perprimitiveEXT flat out MeshPrimitive mesh_primitives[];

                void main() {
                    const vec3 vertices[8] = {
                        vec3(-0.5, -0.5, -0.5), // 0
                        vec3(+0.5, -0.5, -0.5), // 1
                        vec3(+0.5, +0.5, -0.5), // 2
                        vec3(-0.5, +0.5, -0.5), // 3
                        vec3(-0.5, -0.5, +0.5), // 4
                        vec3(+0.5, -0.5, +0.5), // 5
                        vec3(+0.5, +0.5, +0.5), // 6
                        vec3(-0.5, +0.5, +0.5), // 7
                    };

                    const vec2 texcoords[4] = {
                        vec2(0.0, 0.0), // 0
                        vec2(1.0, 0.0), // 1
                        vec2(1.0, 1.0), // 2
                        vec2(0.0, 1.0), // 3
                    };

                    const vec3 normals[6] = {
                        vec3(-1.0, +0.0, +0.0), // 0, -x
                        vec3(+1.0, +0.0, +0.0), // 1, +x
                        vec3(+0.0, -1.0, +0.0), // 2, -y
                        vec3(+0.0, +1.0, +0.0), // 3, +y
                        vec3(+0.0, +0.0, -1.0), // 4, -z
                        vec3(+0.0, +0.0, +1.0), // 5, +z
                    };

                    const uvec3 indices[12] = {
                        uvec3(4, 0, 7), uvec3(7, 0, 3), // -x
                        uvec3(1, 5, 2), uvec3(2, 5, 6), // +x
                        uvec3(4, 5, 0), uvec3(0, 5, 1), // -y
                        uvec3(3, 2, 7), uvec3(7, 2, 6), // +y
                        uvec3(0, 1, 3), uvec3(3, 1, 2), // -z
                        uvec3(5, 4, 6), uvec3(6, 4, 7), // +z
                    };

                    const uvec3 texcoord_indices[2] = {
                        uvec3(0, 1, 3), uvec3(3, 1, 2),
                    };

                    const bool culled[6] = {
                        false, // -x
                        false, // +x
                        false, // -y
                        false, // +y
                        false, // -z
                        false, // +z
                    };

                    SetMeshOutputsEXT(MAX_VERTICES, MAX_PRIMITIVES);
                    for (uint i = 0; i < MAX_PRIMITIVES; i++) {
                        gl_MeshVerticesEXT[3 * i + 0].gl_Position = transform * vec4(vertices[indices[i][0]], 1.0);
                        gl_MeshVerticesEXT[3 * i + 1].gl_Position = transform * vec4(vertices[indices[i][1]], 1.0);
                        gl_MeshVerticesEXT[3 * i + 2].gl_Position = transform * vec4(vertices[indices[i][2]], 1.0);
                        mesh_vertices[3 * i + 0].texcoord = texcoords[texcoord_indices[i % 2][0]];
                        mesh_vertices[3 * i + 1].texcoord = texcoords[texcoord_indices[i % 2][1]];
                        mesh_vertices[3 * i + 2].texcoord = texcoords[texcoord_indices[i % 2][2]];
                        gl_PrimitiveTriangleIndicesEXT[i] = uvec3(3 * i + 0, 3 * i + 1, 3 * i + 2);
                        mesh_primitives[i].normal = normals[i / 2];
                        mesh_primitives[i].texture_id = i / 2;
                        mesh_primitives[i].sampler_id = i / 2;
                        gl_MeshPrimitivesEXT[i].gl_CullPrimitiveEXT = culled[i / 2];
                    }
                }
            "#,
        )?;
        let fragment_spirv = compiler.compile(
            vkx::ShaderType::Fragment,
            "fragment_shader",
            "main",
            r#"
                #version 460 core
                #include "common.glsl"

                layout(location = 0) in MeshVertex mesh_vertex;
                layout(location = 1) perprimitiveEXT flat in MeshPrimitive mesh_primitive;
                layout(binding = 0) uniform texture2D textures[];
                layout(binding = 1) uniform sampler samplers[];
                layout(location = 0) out vec4 fragment_color;

                void main() {
                    vec3 light = normalize(vec3(3.0, 1.5, 3.0));
                    float diffuse = 0.25 + 0.75 * max(0.0, dot(light, mesh_primitive.normal));
                    fragment_color.rgb = diffuse * texture(
                        sampler2D(
                            textures[nonuniformEXT(mesh_primitive.texture_id)],
                            samplers[nonuniformEXT(mesh_primitive.sampler_id)]
                        ), mesh_vertex.texcoord
                    ).rgb;
                    fragment_color.a = 1.0;
                }
            "#,
        )?;

        let shader = vkx::Shader::create(
            &gpu.device,
            &vkx::ShaderCreateInfo {
                shader_binaries: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: create_info.descriptors.storage.set_layouts(),
                push_constant_ranges: create_info.descriptors.storage.push_constant_ranges(),
                specialization_info: None,
            },
        )?;

        Ok(Self { shader })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.shader.destroy(&gpu.device);
    }
}

//
// Render targets
//

struct RenderTargets {
    color: vkx::ImageDedicatedResource,
    depth: vkx::ImageDedicatedResource,
    resolve: vkx::ImageDedicatedResource,
}

impl GpuResource for RenderTargets {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, (): &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let color = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d_samples(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                vk::ImageUsageFlagBits::InputAttachment | vk::ImageUsageFlagBits::ColorAttachment,
                DEFAULT_RENDER_TARGET_SAMPLES,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        let depth = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d_samples(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_DEPTH_FORMAT,
                vk::ImageUsageFlagBits::InputAttachment
                    | vk::ImageUsageFlagBits::DepthStencilAttachment,
                DEFAULT_RENDER_TARGET_SAMPLES,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        let resolve = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_RESOLVE_FORMAT,
                vk::ImageUsageFlagBits::InputAttachment
                    | vk::ImageUsageFlagBits::ColorAttachment
                    | vk::ImageUsageFlagBits::TransferSrc,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        Ok(Self {
            color,
            depth,
            resolve,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.color.destroy(&gpu.device);
        self.depth.destroy(&gpu.device);
        self.resolve.destroy(&gpu.device);
    }
}

//
// Draw
//

fn push_constants() -> Mat4 {
    let fov_y_radians = f32::to_radians(45.0);
    let aspect_ratio = DEFAULT_RENDER_TARGET_WIDTH as f32 / DEFAULT_RENDER_TARGET_HEIGHT as f32;
    let z_near = 0.1;
    let z_far = 100.0;
    let projection = Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far);

    let eye_angle = f32::to_radians(30.0);
    let eye_radius = 2.5;
    let eye_x = eye_radius * f32::cos(eye_angle);
    let eye_y = eye_radius * f32::sin(eye_angle);
    let eye = Vec3::new(eye_x, eye_y, 1.25);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 0.0, 1.0);
    let view = Mat4::look_at_rh(eye, center, up);

    projection * view
}

unsafe fn draw(
    Gpu { device, .. }: &Gpu,
    Demo {
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        descriptors,
        shaders,
        render_targets,
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
        &render_targets.resolve,
        vk::PipelineStageFlagBits2::None,
        vk::AccessFlags2::empty(),
        vk::PipelineStageFlagBits2::ColorAttachmentOutput,
        vk::AccessFlagBits2::ColorAttachmentWrite,
        vk::ImageLayout::Undefined,
        vk::ImageLayout::AttachmentOptimal,
    );

    command_buffer.begin_rendering(
        device,
        (&render_targets.color, DEFAULT_RENDER_TARGET_CLEAR_COLOR),
        Some((&render_targets.depth, 1.0)),
        Some(&render_targets.resolve),
    );
    command_buffer.set_cull_mode(device, vk::CullModeFlagBits::None);
    command_buffer.set_front_face(device, vk::FrontFace::CounterClockwise);
    command_buffer.set_depth_test(device, true);
    command_buffer.set_depth_write(device, true);
    command_buffer.set_depth_compare_op(device, vk::CompareOp::Less);
    command_buffer.set_samples(device, DEFAULT_RENDER_TARGET_SAMPLES);
    command_buffer.set_viewport_flip_y(
        device,
        &vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: render_targets.color.width() as f32,
            height: render_targets.color.height() as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        },
    );
    command_buffer.bind_descriptor_storage(
        device,
        &descriptors.storage,
        vk::PipelineBindPoint::Graphics,
    );
    command_buffer.bind_shader(device, &shaders.shader);
    command_buffer.push_constants(device, &descriptors.storage, &push_constants())?;
    command_buffer.draw_mesh_tasks(device, 1, 1, 1);
    command_buffer.end_rendering(device);

    command_buffer.image_barrier(
        device,
        &render_targets.resolve,
        vk::PipelineStageFlagBits2::ColorAttachmentOutput,
        vk::AccessFlagBits2::ColorAttachmentWrite,
        vk::PipelineStageFlagBits2::Copy,
        vk::AccessFlagBits2::TransferRead,
        vk::ImageLayout::AttachmentOptimal,
        vk::ImageLayout::TransferSrcOptimal,
    );
    command_buffer.copy_image_to_buffer(device, &render_targets.resolve, (&output.buffer, 0));
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
    command_buffer_done.wait(device, 1, u64::MAX)?;

    // Query results.
    {
        let timestamp_differences = timestamps.get_differences(device)?[0];
        let statistics = statistics.get_statistics(device)?;
        println!("Rendering took: {timestamp_differences:?}");
        println!("Rendering statistics: {statistics:?}");
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
