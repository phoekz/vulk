use super::*;

//
// Demo
//

pub struct Demo {
    commands: command::Commands,
    queries: query::Queries,
    textures: Textures,
    descriptors: Descriptors,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "cube";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let commands = command::Commands::create(gpu, &command::CommandsCreateInfo)?;
        let queries = query::Queries::create(gpu, &query::QueriesCreateInfo)?;
        let textures = Textures::create(gpu, &TexturesCreateInfo {})?;
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
        let render_targets = RenderTargets::create(gpu, &RenderTargetsCreateInfo {})?;
        let output = Output::create(gpu, &OutputCreateInfo {})?;
        Ok(Self {
            commands,
            queries,
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
        state.output.destroy(gpu);
        state.render_targets.destroy(gpu);
        state.shaders.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.textures.destroy(gpu);
        state.queries.destroy(gpu);
        state.commands.destroy(gpu);
        Ok(())
    }
}

//
// Textures
//

struct TexturesCreateInfo {}

struct Textures {
    images: Vec<resource::Image2d>,
    samplers: Vec<resource::Sampler>,
}

impl GpuResource for Textures {
    type CreateInfo<'a> = TexturesCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Generate textures.
        let (images, image_datas) = {
            use palette::{FromColor, Hsl, Srgb};
            use rand::prelude::*;

            let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let mut state = rand_pcg::Pcg64Mcg::seed_from_u64(seed.as_secs());
            let noise = rand::distributions::Uniform::new_inclusive(-0.1, 0.1);

            let width = 16;
            let height = 16;
            let count = 6;

            let mut images = vec![];
            let mut image_datas = vec![];

            for image_index in 0..count {
                let image = resource::Image2d::create(
                    gpu,
                    &resource::Image2dCreateInfo {
                        format: vk::Format::R8g8b8a8Unorm,
                        width,
                        height,
                        samples: vk::SampleCountFlagBits::Count1,
                        usage: vk::ImageUsageFlagBits::TransferDst
                            | vk::ImageUsageFlagBits::Sampled,
                        property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
                    },
                )?;

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

                images.push(image);
                image_datas.push(image_data);
            }

            (images, image_datas)
        };

        // Upload.
        resource::multi_upload_images(gpu, &images, &image_datas)?;

        // Sampler.
        let sampler_create_info =
            |mag_filter: vk::Filter, min_filter: vk::Filter| resource::SamplerCreateInfo {
                mag_filter,
                min_filter,
                mipmap_mode: vk::SamplerMipmapMode::Nearest,
                address_mode: vk::SamplerAddressMode::ClampToEdge,
            };
        let sampler_create_infos = [
            sampler_create_info(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_create_info(vk::Filter::Linear, vk::Filter::Nearest),
            sampler_create_info(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_create_info(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_create_info(vk::Filter::Nearest, vk::Filter::Nearest),
            sampler_create_info(vk::Filter::Nearest, vk::Filter::Nearest),
        ];
        let mut samplers = vec![];
        for sampler_create_info in sampler_create_infos {
            samplers.push(resource::Sampler::create(gpu, &sampler_create_info)?);
        }

        Ok(Self { images, samplers })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        for image in &self.images {
            image.destroy(gpu);
        }
        for sampler in &self.samplers {
            sampler.destroy(gpu);
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
        // Descriptor storage.
        let stage_flags = vk::ShaderStageFlagBits::Fragment.into();
        let image_descriptors = create_info
            .textures
            .images
            .iter()
            .map(|img| img.descriptor)
            .collect::<Vec<_>>();
        let sampler_descriptors = create_info
            .textures
            .samplers
            .iter()
            .map(|sampler| sampler.descriptor)
            .collect::<Vec<_>>();
        let storage = descriptor::DescriptorStorage::create(
            gpu,
            &descriptor::DescriptorStorageCreateInfo {
                bindings: &[
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::SampledImage,
                        stage_flags,
                        descriptors: &image_descriptors,
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::Sampler,
                        stage_flags,
                        descriptors: &sampler_descriptors,
                    },
                ],
            },
        )?;

        // Push constants.
        let push_constant_ranges = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlagBits::MeshEXT.into(),
            offset: 0,
            size: size_of::<Mat4>() as _,
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

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.storage.destroy(gpu);
        gpu.device.destroy_pipeline_layout(self.pipeline_layout);
    }
}

//
// Shaders
//

struct ShadersCreateInfo<'a> {
    descriptors: &'a Descriptors,
}

struct Shaders {
    shader: shader::Shader,
}

impl GpuResource for Shaders {
    type CreateInfo<'a> = ShadersCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Shader compiler
        let mut compiler = shader::Compiler::new()?;

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
            shader::ShaderType::Task,
            r#"
          #version 460 core
          #include "common.glsl"

          void main() {
              EmitMeshTasksEXT(1, 1, 1);
          }
      "#,
        )?;
        let mesh_spirv = compiler.compile(
      shader::ShaderType::Mesh,
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
            shader::ShaderType::Fragment,
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

        let shader = shader::Shader::create(
            gpu,
            &shader::ShaderCreateInfo {
                spirvs: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: &[create_info.descriptors.storage.set_layout()],
                push_constant_ranges: &[create_info.descriptors.push_constant_ranges],
                specialization_info: None,
            },
        )?;

        Ok(Self { shader })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.shader.destroy(gpu);
    }
}

//
// Render targets
//

struct RenderTargetsCreateInfo {}

struct RenderTargets {
    color: resource::Image2d,
    depth: resource::Image2d,
    resolve: resource::Image2d,
}

impl GpuResource for RenderTargets {
    type CreateInfo<'a> = RenderTargetsCreateInfo;

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let color = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: DEFAULT_RENDER_TARGET_SAMPLES,
                usage: vk::ImageUsageFlagBits::ColorAttachment.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;
        let depth = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_DEPTH_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: DEFAULT_RENDER_TARGET_SAMPLES,
                usage: vk::ImageUsageFlagBits::DepthStencilAttachment.into(),
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;
        let resolve = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_RESOLVE_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: vk::SampleCountFlagBits::Count1,
                usage: vk::ImageUsageFlagBits::ColorAttachment
                    | vk::ImageUsageFlagBits::TransferSrc,
                property_flags: vk::MemoryPropertyFlagBits::DeviceLocal.into(),
            },
        )?;
        Ok(Self {
            color,
            depth,
            resolve,
        })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.color.destroy(gpu);
        self.depth.destroy(gpu);
        self.resolve.destroy(gpu);
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

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.buffer.destroy(gpu);
    }
}

//
// Draw
//

unsafe fn draw(
    gpu @ Gpu { device, queue, .. }: &Gpu,
    Demo {
        commands,
        queries,
        descriptors,
        shaders,
        render_targets,
        output,
        ..
    }: &Demo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd, vk::PipelineStageFlagBits2::None.into());

    // Transition render target.
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
                dst_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                dst_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.resolve.image,
                subresource_range: render_targets.resolve.subresource_range(),
            },
        },
    );

    // Begin rendering.
    device.cmd_begin_rendering(
        cmd,
        &(vk::RenderingInfo {
            s_type: vk::StructureType::RenderingInfo,
            p_next: null(),
            flags: vk::RenderingFlags::empty(),
            render_area: render_targets.color.rect_2d(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: render_targets.color.image_view,
                image_layout: vk::ImageLayout::AttachmentOptimal,
                resolve_mode: vk::ResolveModeFlagBits::Average,
                resolve_image_view: render_targets.resolve.image_view,
                resolve_image_layout: vk::ImageLayout::AttachmentOptimal,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: DEFAULT_RENDER_TARGET_CLEAR_COLOR,
                    },
                },
            }),
            p_depth_attachment: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: render_targets.depth.image_view,
                image_layout: vk::ImageLayout::AttachmentOptimal,
                resolve_mode: vk::ResolveModeFlagBits::None,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            }),
            p_stencil_attachment: null(),
        }),
    );

    // Set rasterizer state.
    {
        let width = render_targets.color.width() as f32;
        let height = render_targets.color.height() as f32;

        device.cmd_set_cull_mode(cmd, vk::CullModeFlagBits::None.into());
        device.cmd_set_front_face(cmd, vk::FrontFace::CounterClockwise);
        device.cmd_set_depth_test_enable(cmd, vk::TRUE);
        device.cmd_set_depth_write_enable(cmd, vk::TRUE);
        device.cmd_set_depth_compare_op(cmd, vk::CompareOp::Less);
        device.cmd_set_rasterization_samples_ext(cmd, DEFAULT_RENDER_TARGET_SAMPLES);
        device.cmd_set_viewport_with_count(
            cmd,
            1,
            &vk::Viewport {
                x: 0.0,
                y: height,
                width,
                height: -height,
                min_depth: 0.0,
                max_depth: 1.0,
            },
        );
    }

    // Bind descriptors.
    descriptors.storage.bind(gpu, cmd);
    descriptors.storage.set_offsets(
        gpu,
        cmd,
        vk::PipelineBindPoint::Graphics,
        descriptors.pipeline_layout,
    );

    // Bind shaders.
    shaders.shader.bind(gpu, cmd);

    // Push constants.
    {
        let fov_y_radians = f32::to_radians(45.0);
        let aspect_ratio =
            render_targets.color.width() as f32 / render_targets.color.height() as f32;
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

        let transform = projection * view;

        device.cmd_push_constants(
            cmd,
            descriptors.pipeline_layout,
            descriptors.push_constant_ranges.stage_flags,
            descriptors.push_constant_ranges.offset,
            descriptors.push_constant_ranges.size,
            addr_of!(transform).cast(),
        );
    }

    // Draw.
    device.cmd_draw_mesh_tasks_ext(cmd, 1, 1, 1);

    // End rendering.
    device.cmd_end_rendering(cmd);

    // Transition render target.
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
                src_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                src_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::Copy.into(),
                dst_access_mask: vk::AccessFlagBits2::TransferRead.into(),
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.resolve.image,
                subresource_range: render_targets.resolve.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_targets.resolve.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_targets.resolve.width(),
                buffer_image_height: render_targets.resolve.height(),
                image_subresource: render_targets.resolve.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_targets.resolve.extent_3d(),
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
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.resolve.width();
        let height = render_targets.resolve.height();
        let pixels_byte_size = render_targets.resolve.byte_size();
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
