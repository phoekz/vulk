use super::*;

//
// Demo
//

pub struct Demo {
    gui: GuiData,
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    geometry: Geometry,
    textures: Textures,
    descriptors: Descriptors,
    shaders: Shaders,
    render_targets: RenderTargets,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "gui";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let gui = GuiData::create();
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let geometry = Geometry::create(gpu, &gui)?;
        let textures = Textures::create(gpu, &gui)?;
        let descriptors = Descriptors::create(gpu, &geometry, &textures)?;
        let shaders = Shaders::create(gpu, &descriptors)?;
        let render_targets = RenderTargets::create(gpu)?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;

        Ok(Self {
            gui,
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
            geometry,
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
        state.geometry.destroy(gpu);
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
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

struct Geometry {
    vertex_buffer: vkx::BufferResource,
    index_buffer: vkx::BufferResource,
    buffer_allocations: vkx::BufferAllocations,
}

impl Geometry {
    unsafe fn create(gpu: &Gpu, gui: &GuiData) -> Result<Self> {
        // Sizes.
        let vertex_buffer_size = size_of::<imgui::DrawVert>() * gui.vertex_data.len();
        let index_buffer_size = size_of::<u16>() * gui.index_data.len();

        // Flags.
        let usage = vk::BufferUsageFlagBits::StorageBuffer | vk::BufferUsageFlagBits::TransferDst;
        let property_flags = vk::MemoryPropertyFlagBits::DeviceLocal;

        // Creators.
        let vertex_buffer_creator = vkx::BufferCreator::new(vertex_buffer_size as _, usage);
        let index_buffer_creator = vkx::BufferCreator::new(index_buffer_size as _, usage);

        // Buffer resources.
        let (mut buffer_resources, buffer_allocations) = vkx::BufferResource::create(
            &gpu.physical_device,
            &gpu.device,
            &[vertex_buffer_creator, index_buffer_creator],
            property_flags,
        )?;

        // Buffer datas.
        let vertex_data =
            std::slice::from_raw_parts(gui.vertex_data.as_ptr().cast::<u8>(), vertex_buffer_size);
        let index_data =
            std::slice::from_raw_parts(gui.index_data.as_ptr().cast::<u8>(), index_buffer_size);

        // Transfer.
        vkx::transfer_resources(
            &gpu.physical_device,
            &gpu.device,
            &buffer_resources,
            &[vertex_data, index_data],
            &[],
            &[],
        )?;

        let vertex_buffer = buffer_resources.remove(0);
        let index_buffer = buffer_resources.remove(0);

        Ok(Self {
            vertex_buffer,
            index_buffer,
            buffer_allocations,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.vertex_buffer.destroy(&gpu.device);
        self.index_buffer.destroy(&gpu.device);
        self.buffer_allocations.free(&gpu.device);
    }
}

//
// Textures
//

struct Textures {
    image: vkx::ImageResource,
    image_allocations: vkx::ImageAllocations,
    sampler: vkx::SamplerResource,
}

impl Textures {
    unsafe fn create(gpu: &Gpu, gui: &GuiData) -> Result<Self> {
        // Images.
        let (mut image_resources, image_allocations) = vkx::ImageResource::create(
            &gpu.physical_device,
            &gpu.device,
            &[vkx::ImageCreator::new_2d(
                gui.texture_width,
                gui.texture_height,
                vk::Format::R8g8b8a8Unorm,
                vk::ImageUsageFlagBits::TransferDst | vk::ImageUsageFlagBits::Sampled,
            )],
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        vkx::transfer_resources(
            &gpu.physical_device,
            &gpu.device,
            &[],
            &[],
            &image_resources,
            &[&gui.texture_data],
        )?;
        let image_resource = image_resources.swap_remove(0);

        // Sampler.
        let (sampler, sampler_create_info) = vkx::SamplerCreator::new()
            .mag_filter(vk::Filter::Linear)
            .min_filter(vk::Filter::Linear)
            .mipmap_mode(vk::SamplerMipmapMode::Linear)
            .address_mode_uvw(vk::SamplerAddressMode::ClampToEdge)
            .create(&gpu.device)?;
        let mut samplers = vkx::SamplerResource::create(
            &gpu.physical_device,
            &gpu.device,
            &[sampler],
            &[sampler_create_info],
        )?;
        let sampler = samplers.swap_remove(0);

        Ok(Self {
            image: image_resource,
            image_allocations,
            sampler,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.image.destroy(&gpu.device);
        self.image_allocations.free(&gpu.device);
        self.sampler.destroy(&gpu.device);
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
    storage: vkx::DescriptorStorage,
}

impl Descriptors {
    unsafe fn create(gpu: &Gpu, geometry: &Geometry, textures: &Textures) -> Result<Self> {
        // Descriptor storage.
        let stages = vk::ShaderStageFlagBits::TaskEXT
            | vk::ShaderStageFlagBits::MeshEXT
            | vk::ShaderStageFlagBits::Fragment;
        let storage = vkx::DescriptorStorage::create(
            &gpu.physical_device,
            &gpu.device,
            &[
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageBuffer,
                    stages,
                    descriptors: &[geometry.vertex_buffer.descriptor()],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::StorageBuffer,
                    stages,
                    descriptors: &[geometry.index_buffer.descriptor()],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::SampledImage,
                    stages,
                    descriptors: &[textures.image.descriptor()],
                },
                vkx::DescriptorBinding {
                    ty: vk::DescriptorType::Sampler,
                    stages,
                    descriptors: &[textures.sampler.descriptor()],
                },
            ],
            Some(vk::PushConstantRange {
                stage_flags: stages,
                offset: 0,
                size: size_of::<PushConstants>() as _,
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

struct Shaders {
    shader: vkx::Shader,
}

impl Shaders {
    unsafe fn create(gpu: &Gpu, descriptors: &Descriptors) -> Result<Self> {
        // Shader compiler
        let mut compiler = vkx::ShaderCompiler::new()?;

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
            layout(scalar, binding = 0) readonly buffer VertexBuffer { Vertex vertex_buffer[]; };
            layout(scalar, binding = 1) readonly buffer IndexBuffer { uint16_t index_buffer[]; };
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
            vkx::ShaderType::Task,
            "task_shader",
            "main",
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
            vkx::ShaderType::Mesh,
            "mesh_shader",
            "main",
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
            vkx::ShaderType::Fragment,
            "fragment_shader",
            "main",
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

        let shader = vkx::Shader::create(
            &gpu.device,
            &vkx::ShaderCreateInfo {
                shader_binaries: &[task_spirv, mesh_spirv, fragment_spirv],
                set_layouts: descriptors.storage.set_layouts(),
                push_constant_ranges: descriptors.storage.push_constant_ranges(),
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
}

impl RenderTargets {
    unsafe fn create(gpu: &Gpu) -> Result<Self> {
        let color = vkx::ImageDedicatedResource::create_2d(
            &gpu.physical_device,
            &gpu.device,
            vkx::ImageCreator::new_2d(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
                DEFAULT_RENDER_TARGET_COLOR_FORMAT,
                vk::ImageUsageFlagBits::InputAttachment
                    | vk::ImageUsageFlagBits::ColorAttachment
                    | vk::ImageUsageFlagBits::TransferSrc,
            ),
            vk::MemoryPropertyFlagBits::DeviceLocal,
        )?;
        Ok(Self { color })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.color.destroy(&gpu.device);
    }
}

//
// Output
//

struct Output {
    buffer: vkx::BufferDedicatedTransfer,
}

impl GpuResource for Output {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let buffer = vkx::BufferDedicatedTransfer::create(
            &gpu.physical_device,
            &gpu.device,
            vkx::BufferCreator::new(
                DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE,
                vk::BufferUsageFlagBits::TransferDst,
            ),
            vk::MemoryPropertyFlagBits::HostVisible,
        )?;
        Ok(Self { buffer })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.buffer.destroy(&gpu.device);
    }
}

//
// Draw
//

unsafe fn draw(
    Gpu { device, .. }: &Gpu,
    GuiDemo {
        gui,
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        descriptors,
        shaders,
        render_targets,
        output,
        ..
    }: &GuiDemo,
    demo_name: &str,
) -> Result<()> {
    // Record commands.
    command_buffer.begin(device)?;
    command_buffer.write_timestamp(device, timestamps, 0);
    command_buffer.begin_statistics(device, statistics);
    command_buffer.image_barrier(
        device,
        &render_targets.color,
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
        None,
        None,
    );
    command_buffer.set_viewport(
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
    command_buffer.set_cull_mode(device, vk::CullModeFlagBits::None);
    command_buffer.set_color_blend(device, 0, true);
    command_buffer.set_color_blend_equation(
        device,
        0,
        &vk::ColorBlendEquationEXT {
            src_color_blend_factor: vk::BlendFactor::SrcAlpha,
            dst_color_blend_factor: vk::BlendFactor::OneMinusSrcAlpha,
            color_blend_op: vk::BlendOp::Add,
            src_alpha_blend_factor: vk::BlendFactor::One,
            dst_alpha_blend_factor: vk::BlendFactor::One,
            alpha_blend_op: vk::BlendOp::Add,
        },
    );
    command_buffer.set_color_write_mask_rgba(device, 0);
    command_buffer.bind_descriptor_storage(
        device,
        &descriptors.storage,
        vk::PipelineBindPoint::Graphics,
    );
    command_buffer.bind_shader(device, &shaders.shader);
    for draw in &gui.draw_commands {
        let push_constants = PushConstants {
            scale: draw.scale,
            translation: draw.translation,
            vertex_offset: draw.vertex_offset,
            index_offset: draw.index_offset,
            primitive_count: draw.primitive_count,
        };
        command_buffer.push_constants(device, &descriptors.storage, &push_constants)?;
        command_buffer.set_scissor(device, &draw.scissor);
        command_buffer.draw_mesh_tasks(device, 1, 1, 1);
    }
    command_buffer.end_rendering(device);

    command_buffer.image_barrier(
        device,
        &render_targets.color,
        vk::PipelineStageFlagBits2::ColorAttachmentOutput,
        vk::AccessFlagBits2::ColorAttachmentWrite,
        vk::PipelineStageFlagBits2::Copy,
        vk::AccessFlagBits2::TransferRead,
        vk::ImageLayout::AttachmentOptimal,
        vk::ImageLayout::TransferSrcOptimal,
    );
    command_buffer.copy_image_to_buffer(device, &render_targets.color, (&output.buffer, 0));
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
        let mesh_primitives_generated = statistics
            .mesh_primitives_generated
            .mesh_primitives_generated;
        ensure!(mesh_primitives_generated == gui.total_primitive_count);
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
