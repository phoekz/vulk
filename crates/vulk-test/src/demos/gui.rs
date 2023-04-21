use super::*;

pub struct Demo {
    gui: GuiData,
    commands: command::Commands,
    queries: query::Queries,
    geometry: Geometry,
    textures: Textures,
    descriptors: Descriptors,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: Output,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "gui";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let gui = GuiData::create();
        let commands = command::Commands::create(gpu)?;
        let queries = query::Queries::create(gpu)?;
        let geometry = Geometry::create(gpu, &gui)?;
        let textures = Textures::create(gpu, &gui)?;
        let descriptors = Descriptors::create(gpu, &geometry, &textures)?;
        let shaders = Shaders::create(gpu, &descriptors)?;
        let render_targets = RenderTargets::create(gpu)?;
        let output = Output::create(gpu)?;

        Ok(Self {
            gui,
            commands,
            queries,
            geometry,
            textures,
            descriptors,
            shaders,
            render_targets,
            output,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &Self) -> Result<()> {
        draw(gpu, state, Self::NAME)
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.output.destroy(gpu);
        state.queries.destroy(gpu);
        state.render_targets.destroy(gpu);
        state.shaders.destroy(gpu);
        state.descriptors.destroy(gpu);
        state.textures.destroy(gpu);
        state.geometry.destroy(gpu);
        state.commands.destroy(gpu);
        Ok(())
    }
}

//
// Gui
//

#[derive(Clone, Copy, Debug)]
struct DrawCommand {
    scissor: vk::Rect2D,
    vertex_offset: u32,
    index_offset: u32,
    primitive_count: u32,
    scale: Vec2,
    translation: Vec2,
}

struct GuiData {
    vertex_data: Vec<imgui::DrawVert>,
    index_data: Vec<u16>,
    texture_width: u32,
    texture_height: u32,
    texture_data: Vec<u8>,
    draw_commands: Vec<DrawCommand>,
    total_primitive_count: u64,
}

impl GuiData {
    fn create() -> Self {
        // Init.
        let mut imgui = {
            let mut imgui = imgui::Context::create();
            imgui.set_ini_filename(None);
            let mut io = imgui.io_mut();
            io.display_framebuffer_scale = [1.0, 1.0];
            io.display_size = [
                DEFAULT_RENDER_TARGET_WIDTH as _,
                DEFAULT_RENDER_TARGET_HEIGHT as _,
            ];
            io.backend_flags
                .insert(imgui::BackendFlags::RENDERER_HAS_VTX_OFFSET);
            imgui
        };

        // Font atlas.
        let (texture_width, texture_height, texture_data) = {
            let mut fonts = imgui.fonts();
            fonts.tex_id = imgui::TextureId::new(usize::MAX);
            let texture = fonts.build_rgba32_texture();
            let texture_width = texture.width;
            let texture_height = texture.height;
            let texture_data = texture.data.to_vec();
            (texture_width, texture_height, texture_data)
        };

        // Render UI.
        let draw_data = {
            let ui = imgui.new_frame();
            ui.window("vulk-test")
                .size([200.0, 200.0], imgui::Condition::FirstUseEver)
                .position(
                    [
                        DEFAULT_RENDER_TARGET_WIDTH as f32 / 2.0,
                        DEFAULT_RENDER_TARGET_HEIGHT as f32 / 2.0,
                    ],
                    imgui::Condition::FirstUseEver,
                )
                .position_pivot([0.5, 0.5])
                .build(|| {
                    ui.text_wrapped("Hello world!");
                    ui.separator();
                    let mut value = 42;
                    ui.input_int("int", &mut value).build();
                    ui.button("button");
                });
            imgui.render()
        };

        // Geometry.
        let mut vertex_data = vec![];
        let mut index_data = vec![];
        for draw_list in draw_data.draw_lists() {
            vertex_data.extend_from_slice(draw_list.vtx_buffer());
            index_data.extend_from_slice(draw_list.idx_buffer());
        }

        // Draw commands.
        let mut draw_commands = vec![];
        let total_primitive_count = (draw_data.total_idx_count / 3) as u64;
        for draw_list in draw_data.draw_lists() {
            for draw_cmd in draw_list.commands() {
                let imgui::DrawCmd::Elements { count, cmd_params } = draw_cmd else {
                    continue;
                };
                assert!(count % 3 == 0);

                let imgui::DrawCmdParams {
                    clip_rect,
                    vtx_offset,
                    idx_offset,
                    ..
                } = cmd_params;

                let scale = Vec2::new(
                    2.0 / draw_data.display_size[0],
                    2.0 / draw_data.display_size[1],
                );
                let translation = Vec2::new(
                    -1.0 - draw_data.display_pos[0] * scale[0],
                    -1.0 - draw_data.display_pos[1] * scale[1],
                );

                draw_commands.push(DrawCommand {
                    scissor: vk::Rect2D {
                        offset: vk::Offset2D {
                            x: clip_rect[0] as i32,
                            y: clip_rect[1] as i32,
                        },
                        extent: vk::Extent2D {
                            width: (clip_rect[2] - clip_rect[0]) as u32,
                            height: (clip_rect[3] - clip_rect[1]) as u32,
                        },
                    },
                    vertex_offset: vtx_offset as u32,
                    index_offset: idx_offset as u32,
                    primitive_count: count as u32 / 3,
                    scale,
                    translation,
                });
            }
        }

        Self {
            vertex_data,
            index_data,
            texture_width,
            texture_height,
            texture_data,
            draw_commands,
            total_primitive_count,
        }
    }
}

//
// Geometry
//

type VertexBuffer = resource::Buffer<imgui::DrawVert>;
type IndexBuffer = resource::Buffer<u16>;

struct Geometry {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

impl Geometry {
    unsafe fn create(gpu: &Gpu, gui: &GuiData) -> Result<Self> {
        // Buffers.
        let vertex_buffer = VertexBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                element_count: gui.vertex_data.len(),
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
                property_flags: vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
            },
        )?;
        let index_buffer = IndexBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                element_count: gui.index_data.len(),
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
                property_flags: vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
            },
        )?;

        // Copy.
        std::ptr::copy_nonoverlapping(
            gui.vertex_data.as_ptr(),
            vertex_buffer.ptr,
            gui.vertex_data.len(),
        );
        std::ptr::copy_nonoverlapping(
            gui.index_data.as_ptr(),
            index_buffer.ptr,
            gui.index_data.len(),
        );

        Ok(Self {
            vertex_buffer,
            index_buffer,
        })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.vertex_buffer.destroy(gpu);
        self.index_buffer.destroy(gpu);
    }
}

//
// Textures
//

struct Textures {
    image: resource::Image2d,
    sampler: resource::Sampler,
}

impl Textures {
    unsafe fn create(gpu: &Gpu, gui: &GuiData) -> Result<Self> {
        let image = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: vk::Format::R8g8b8a8Unorm,
                width: gui.texture_width,
                height: gui.texture_height,
                samples: vk::SampleCountFlagBits::NUM_1,
                usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            },
        )?;
        resource::multi_upload_images(
            gpu,
            std::slice::from_ref(&image),
            std::slice::from_ref(&gui.texture_data),
        )?;
        let sampler = resource::Sampler::create(
            gpu,
            &resource::SamplerCreateInfo {
                mag_filter: vk::Filter::Linear,
                min_filter: vk::Filter::Linear,
                mipmap_mode: vk::SamplerMipmapMode::Linear,
                address_mode: vk::SamplerAddressMode::ClampToEdge,
            },
        )?;
        Ok(Self { image, sampler })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.image.destroy(gpu);
        self.sampler.destroy(gpu);
    }
}

//
// Descriptors
//

#[repr(C)]
struct PushConstants {
    scale: Vec2,
    translation: Vec2,
    vertex_offset: u32,
    index_offset: u32,
    primitive_count: u32,
}

struct Descriptors {
    storage: descriptor::DescriptorStorage,
    pipeline_layout: vk::PipelineLayout,
    push_constant_range: vk::PushConstantRange,
}

impl Descriptors {
    unsafe fn create(
        gpu @ Gpu { device, .. }: &Gpu,
        geometry: &Geometry,
        textures: &Textures,
    ) -> Result<Self> {
        // Descriptor storage.
        let stage_flags = vk::ShaderStageFlags::TASK_EXT
            | vk::ShaderStageFlags::MESH_EXT
            | vk::ShaderStageFlags::FRAGMENT;
        let storage = descriptor::DescriptorStorage::create(
            gpu,
            &descriptor::DescriptorStorageCreateInfo {
                bindings: &[
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::StorageBuffer,
                        stage_flags,
                        descriptors: &[geometry.vertex_buffer.descriptor],
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::StorageBuffer,
                        stage_flags,
                        descriptors: &[geometry.index_buffer.descriptor],
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::SampledImage,
                        stage_flags,
                        descriptors: &[textures.image.descriptor],
                    },
                    descriptor::DescriptorStorageBinding {
                        descriptor_type: vk::DescriptorType::Sampler,
                        stage_flags,
                        descriptors: &[textures.sampler.descriptor],
                    },
                ],
            },
        )?;

        // Pipeline layout.
        let push_constant_range = vk::PushConstantRange {
            stage_flags,
            offset: 0,
            size: size_of::<PushConstants>() as _,
        };
        let pipeline_layout = device.create_pipeline_layout(
            &(vk::PipelineLayoutCreateInfo {
                s_type: vk::StructureType::PipelineLayoutCreateInfo,
                p_next: null(),
                flags: vk::PipelineLayoutCreateFlags::empty(),
                set_layout_count: 1,
                p_set_layouts: &storage.set_layout(),
                push_constant_range_count: 1,
                p_push_constant_ranges: &push_constant_range,
            }),
        )?;

        Ok(Self {
            storage,
            pipeline_layout,
            push_constant_range,
        })
    }

    unsafe fn destroy(&self, gpu @ Gpu { device, .. }: &Gpu) {
        device.destroy_pipeline_layout(self.pipeline_layout);
        self.storage.destroy(gpu);
    }
}

//
// Shaders
//

struct Shaders {
    shader: shader::Shader,
}

impl Shaders {
    unsafe fn create(gpu: &Gpu, descriptors: &Descriptors) -> Result<Self> {
        // Shader compiler
        let mut compiler = shader::Compiler::new()?;

        // Includes.
        compiler.include(
            "common.glsl",
            r#"
            #extension GL_EXT_shader_16bit_storage : require
            #extension GL_EXT_scalar_block_layout : require
            #extension GL_EXT_mesh_shader : require

            #define WORKGROUP_SIZE 32
            #define MAX_VERTICES (3 * WORKGROUP_SIZE)
            #define MAX_PRIMITIVES WORKGROUP_SIZE

            layout(scalar, push_constant) uniform Constants {
                vec2 scale;
                vec2 translation;
                uint vertex_offset;
                uint index_offset;
                uint primitive_count;
            } constants;

            struct Vertex {
                vec2 position;
                vec2 texcoord;
                uint color;
            };
            layout(scalar, binding = 0) readonly buffer VertexBuffer {
                Vertex vertex_buffer[];
            };

            layout(scalar, binding = 1) readonly buffer IndexBuffer {
                uint16_t index_buffer[];
            };

            layout(binding = 2) uniform texture2D atlas_texture;

            layout(binding = 3) uniform sampler atlas_sampler;

            struct MeshVertex {
                vec2 texcoord;
                vec4 color;
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
                    uint workgroup_count = (constants.primitive_count + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
                    EmitMeshTasksEXT(workgroup_count, 1, 1);
                }
            "#,
        )?;
        let mesh_spirv = compiler.compile(
            shader::ShaderType::Mesh,
            r#"
                #version 460 core
                #include "common.glsl"

                layout(local_size_x = WORKGROUP_SIZE, local_size_y = 1, local_size_z = 1) in;
                layout(triangles, max_vertices = MAX_VERTICES, max_primitives = MAX_PRIMITIVES) out;
                layout(location = 0) out MeshVertex mesh_vertices[];

                uvec3 vertex_indices(uint primitive_index) {
                    uint v0 = constants.vertex_offset + uint(index_buffer[constants.index_offset + 3 * primitive_index + 0]);
                    uint v1 = constants.vertex_offset + uint(index_buffer[constants.index_offset + 3 * primitive_index + 1]);
                    uint v2 = constants.vertex_offset + uint(index_buffer[constants.index_offset + 3 * primitive_index + 2]);
                    return uvec3(v0, v1, v2);
                }

                vec4 vertex_position(uint vertex_index) {
                    vec2 position = vertex_buffer[vertex_index].position;
                    return vec4(constants.translation + constants.scale * position, 0.0, 1.0);
                }

                vec2 vertex_texcoord(uint vertex_index) {
                    return vertex_buffer[vertex_index].texcoord;
                }

                vec4 vertex_color(uint vertex_index) {
                    uint color = vertex_buffer[vertex_index].color;
                    return unpackUnorm4x8(color);
                }

                void main() {
                    // Source.
                    uint src_primitive_index = gl_GlobalInvocationID.x;
                    uvec3 src = vertex_indices(src_primitive_index);

                    // Avoid overruns.
                    if (src_primitive_index >= constants.primitive_count) {
                        return;
                    }

                    // Destination.
                    uint dst_primitive_index = gl_LocalInvocationID.x;
                    uvec3 dst = 3 * dst_primitive_index + uvec3(0, 1, 2);

                    // Only the first invocation in a workgroup sets mesh output sizes.
                    if (dst_primitive_index == 0) {
                        uint remaining_count = constants.primitive_count - gl_WorkGroupID.x * gl_WorkGroupSize.x;
                        uint primitive_count = min(gl_WorkGroupSize.x, remaining_count);
                        uint vertex_count = 3 * primitive_count;
                        SetMeshOutputsEXT(vertex_count, primitive_count);
                    }

                    // Output.
                    gl_MeshVerticesEXT[dst[0]].gl_Position = vertex_position(src[0]);
                    gl_MeshVerticesEXT[dst[1]].gl_Position = vertex_position(src[1]);
                    gl_MeshVerticesEXT[dst[2]].gl_Position = vertex_position(src[2]);
                    mesh_vertices[dst[0]].texcoord = vertex_texcoord(src[0]);
                    mesh_vertices[dst[1]].texcoord = vertex_texcoord(src[1]);
                    mesh_vertices[dst[2]].texcoord = vertex_texcoord(src[2]);
                    mesh_vertices[dst[0]].color = vertex_color(src[0]);
                    mesh_vertices[dst[1]].color = vertex_color(src[1]);
                    mesh_vertices[dst[2]].color = vertex_color(src[2]);
                    gl_PrimitiveTriangleIndicesEXT[dst_primitive_index][0] = dst[0];
                    gl_PrimitiveTriangleIndicesEXT[dst_primitive_index][1] = dst[1];
                    gl_PrimitiveTriangleIndicesEXT[dst_primitive_index][2] = dst[2];
                }
            "#,
        )?;
        let fragment_spirv = compiler.compile(
            shader::ShaderType::Fragment,
            r#"
                #version 460 core
                #include "common.glsl"

                layout(location = 0) in MeshVertex mesh_vertex;
                layout(location = 0) out vec4 fragment_color;

                void main() {
                    vec2 texcoord = mesh_vertex.texcoord;
                    vec4 vertex_color = mesh_vertex.color;
                    vec4 atlas_color = texture(sampler2D(atlas_texture, atlas_sampler), texcoord);
                    fragment_color = vertex_color * atlas_color;
                }
            "#,
        )?;

        let shader = shader::Shader::create(
            gpu,
            &shader::ShaderCreateInfo {
                spirvs: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: &[descriptors.storage.set_layout()],
                push_constant_ranges: &[descriptors.push_constant_range],
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

struct RenderTargets {
    color: resource::Image2d,
}

impl RenderTargets {
    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let color = resource::Image2d::create(
            gpu,
            &resource::Image2dCreateInfo {
                format: DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                width: DEFAULT_RENDER_TARGET_WIDTH,
                height: DEFAULT_RENDER_TARGET_HEIGHT,
                samples: vk::SampleCountFlagBits::NUM_1,
                usage: vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
                property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            },
        )?;
        Ok(Self { color })
    }

    unsafe fn destroy(&self, gpu: &Gpu) {
        self.color.destroy(gpu);
    }
}

//
// Output
//

type OutputBuffer = resource::Buffer<u32>;

struct Output {
    buffer: OutputBuffer,
}

impl Output {
    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let buffer = OutputBuffer::create(
            gpu,
            &resource::BufferCreateInfo {
                element_count: DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE as _,
                usage: vk::BufferUsageFlags::TRANSFER_DST,
                property_flags: vk::MemoryPropertyFlags::HOST_VISIBLE,
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
    GuiDemo {
        gui,
        commands,
        queries,
        descriptors,
        shaders,
        render_targets,
        output,
        ..
    }: &GuiDemo,
    demo_name: &str,
) -> Result<()> {
    // Begin command buffer.
    let cmd = commands.begin(gpu)?;

    // Begin queries.
    queries.begin(gpu, cmd);

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
                src_stage_mask: vk::PipelineStageFlags2::TOP_OF_PIPE,
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
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
                resolve_mode: vk::ResolveModeFlagBits::NONE,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: DEFAULT_RENDER_TARGET_CLEAR_COLOR,
                    },
                },
            }),
            p_depth_attachment: null(),
            p_stencil_attachment: null(),
        }),
    );

    // Bind descriptors.
    descriptors.storage.bind(gpu, cmd);
    descriptors.storage.set_offsets(
        gpu,
        cmd,
        vk::PipelineBindPoint::Graphics,
        descriptors.pipeline_layout,
    );

    // Set rasterizer state.
    {
        let width = render_targets.color.width() as f32;
        let height = render_targets.color.height() as f32;

        device.cmd_set_cull_mode(cmd, vk::CullModeFlags::NONE);
        device.cmd_set_viewport_with_count(
            cmd,
            1,
            &vk::Viewport {
                x: 0.0,
                y: 0.0,
                width,
                height,
                min_depth: 0.0,
                max_depth: 1.0,
            },
        );

        device.cmd_set_color_blend_enable_ext(cmd, 0, 1, [vk::TRUE].as_ptr());
        device.cmd_set_color_blend_equation_ext(
            cmd,
            0,
            1,
            &vk::ColorBlendEquationEXT {
                src_color_blend_factor: vk::BlendFactor::SrcAlpha,
                dst_color_blend_factor: vk::BlendFactor::OneMinusSrcAlpha,
                color_blend_op: vk::BlendOp::Add,
                src_alpha_blend_factor: vk::BlendFactor::One,
                dst_alpha_blend_factor: vk::BlendFactor::One,
                alpha_blend_op: vk::BlendOp::Add,
            },
        );
        device.cmd_set_color_write_mask_ext(
            cmd,
            0,
            1,
            [vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A]
            .as_ptr(),
        );
    }

    // Bind shaders.
    shaders.shader.bind(gpu, cmd);

    // Draws.
    for draw in &gui.draw_commands {
        let push_constants = PushConstants {
            scale: draw.scale,
            translation: draw.translation,
            vertex_offset: draw.vertex_offset,
            index_offset: draw.index_offset,
            primitive_count: draw.primitive_count,
        };
        device.cmd_push_constants(
            cmd,
            descriptors.pipeline_layout,
            descriptors.push_constant_range.stage_flags,
            descriptors.push_constant_range.offset,
            descriptors.push_constant_range.size,
            addr_of!(push_constants).cast(),
        );
        device.cmd_set_scissor_with_count(cmd, 1, &draw.scissor);
        device.cmd_draw_mesh_tasks_ext(cmd, 1, 1, 1);
    }

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
                src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                src_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags2::COPY,
                dst_access_mask: vk::AccessFlags2::TRANSFER_READ,
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::TransferSrcOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: render_targets.color.image,
                subresource_range: render_targets.color.subresource_range(),
            },
        },
    );

    // Copy to output.
    device.cmd_copy_image_to_buffer2(
        cmd,
        &(vk::CopyImageToBufferInfo2 {
            s_type: vk::StructureType::CopyImageToBufferInfo2,
            p_next: null(),
            src_image: render_targets.color.image,
            src_image_layout: vk::ImageLayout::TransferSrcOptimal,
            dst_buffer: output.buffer.buffer,
            region_count: 1,
            p_regions: &(vk::BufferImageCopy2 {
                s_type: vk::StructureType::BufferImageCopy2,
                p_next: null(),
                buffer_offset: 0,
                buffer_row_length: render_targets.color.width(),
                buffer_image_height: render_targets.color.height(),
                image_subresource: render_targets.color.subresource_layers(),
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: render_targets.color.extent_3d(),
            }),
        }),
    );

    // End queries.
    queries.end(gpu, cmd);

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
                stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
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
                flags: vk::SemaphoreWaitFlags::ANY,
                semaphore_count: semaphores.len() as _,
                p_semaphores: semaphores.as_ptr(),
                p_values: values.as_ptr(),
            }),
            u64::MAX,
        )?;
    }

    // Query results.
    {
        let duration = queries.elapsed(gpu)?;
        let statistics = queries.statistics(gpu)?;
        info!("Rendering took {duration:?}");
        info!("Rendering statistics: {statistics:?}");
        ensure!(statistics.mesh_shader.primitives_generated == gui.total_primitive_count);
    }

    // Write image.
    {
        use imagelib::{ImageFormat, RgbaImage};
        let width = render_targets.color.width();
        let height = render_targets.color.height();
        let pixels_byte_size = render_targets.color.byte_size();
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