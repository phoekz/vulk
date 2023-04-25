use super::*;

//
// Demo
//

pub struct Demo {
    window_system: WindowSystem,
    renderer: Renderer,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "window";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let window = WindowSystem::create()?;
        let renderer = Renderer::create(
            gpu,
            &RendererCreateInfo {
                window_system: &window,
            },
        )?;
        Ok(Self {
            window_system: window,
            renderer,
        })
    }

    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()> {
        execute(gpu, &mut state.window_system, &state.renderer, Self::NAME)?;
        gpu.device.device_wait_idle()?;
        Ok(())
    }

    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()> {
        state.renderer.destroy(gpu);
        Ok(())
    }
}

//
// Window
//

struct UserEvent {}

struct WindowSystem {
    event_loop: winit::event_loop::EventLoop<UserEvent>,
    window: winit::window::Window,
}

impl WindowSystem {
    fn create() -> Result<Self> {
        let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build();
        let window = winit::window::WindowBuilder::new()
            .with_active(true)
            .with_visible(true)
            .with_resizable(false)
            .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
            .with_inner_size(winit::dpi::PhysicalSize::new(
                DEFAULT_RENDER_TARGET_WIDTH,
                DEFAULT_RENDER_TARGET_HEIGHT,
            ))
            .build(&event_loop)?;
        let monitor = window
            .primary_monitor()
            .context("Getting primary monitor")?;
        window.set_outer_position(winit::dpi::PhysicalPosition::new(
            (monitor.size().width - DEFAULT_RENDER_TARGET_WIDTH) / 2,
            (monitor.size().height - DEFAULT_RENDER_TARGET_HEIGHT) / 2,
        ));
        Ok(Self { event_loop, window })
    }
}

//
// Renderer
//

struct RendererCreateInfo<'a> {
    window_system: &'a WindowSystem,
}

struct Renderer {
    surface: vkx::Surface,
    swapchain: Swapchain,
    commands: Commands,
}

impl GpuResource for Renderer {
    type CreateInfo<'a> = RendererCreateInfo<'a>;

    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let surface = vkx::Surface::create(
            &gpu.instance,
            &gpu.physical_device,
            &create_info.window_system.window,
        )?;
        let swapchain = Swapchain::create(gpu, &SwapchainCreateInfo { surface: &surface })?;
        let commands = Commands::create(
            gpu,
            &CommandsCreateInfo {
                image_count: swapchain.image_count(),
            },
        )?;
        Ok(Self {
            surface,
            swapchain,
            commands,
        })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.commands.destroy(gpu);
        self.swapchain.destroy(gpu);
        self.surface.destroy(&gpu.instance);
    }
}

//
// Swapchain
//

struct SwapchainCreateInfo<'a> {
    surface: &'a vkx::Surface,
}

struct Swapchain {
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<(vk::Image, vk::ImageView)>,
    swapchain_create_info_khr: vk::SwapchainCreateInfoKHR,
}

impl GpuResource for Swapchain {
    type CreateInfo<'a> = SwapchainCreateInfo<'a>;

    unsafe fn create(
        Gpu { device, .. }: &Gpu,
        Self::CreateInfo { surface }: &Self::CreateInfo<'_>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        // Swapchain.
        let swapchain_create_info_khr = surface.swapchain_create_info_khr();
        let swapchain = device.create_swapchain_khr(&swapchain_create_info_khr)?;

        // Swapchain images.
        let mut swapchain_images = vec![];
        for image in vulk::read_to_vec(
            |count, ptr| device.get_swapchain_images_khr(swapchain, count, ptr),
            None,
        )? {
            let image_view = device.create_image_view(&surface.image_view_create_info(image))?;
            swapchain_images.push((image, image_view));
        }

        Ok(Self {
            swapchain,
            swapchain_images,
            swapchain_create_info_khr,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_swapchain_khr(self.swapchain);
        for &(_, image_view) in &self.swapchain_images {
            device.destroy_image_view(image_view);
        }
    }
}

impl Swapchain {
    fn image_count(&self) -> u64 {
        self.swapchain_images.len() as u64
    }

    fn image(&self, image_index: u32) -> vk::Image {
        self.swapchain_images[image_index as usize].0
    }

    fn image_view(&self, image_index: u32) -> vk::ImageView {
        self.swapchain_images[image_index as usize].1
    }

    fn image_subresource_range(&self) -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange {
            aspect_mask: self.swapchain_create_info_khr.image_format.aspect_mask(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        }
    }

    fn render_area(&self) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain_create_info_khr.image_extent,
        }
    }
}

//
// Commands
//

struct CommandsCreateInfo {
    image_count: u64,
}

struct Commands {
    command_pool: vk::CommandPool,
    command_buffer_available: vk::Semaphore,
    command_buffers: Vec<vk::CommandBuffer>,
    present_completes: Vec<vk::Semaphore>,
    rendering_completes: Vec<vk::Semaphore>,
}

impl GpuResource for Commands {
    type CreateInfo<'a> = CommandsCreateInfo;

    unsafe fn create(Gpu { device, .. }: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        // Command pool.
        let command_pool = device.create_command_pool(
            &(vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::CommandPoolCreateInfo,
                p_next: null(),
                flags: vk::CommandPoolCreateFlagBits::ResetCommandBuffer.into(),
                queue_family_index: device.queue_family_index,
            }),
        )?;

        // Command buffer available -semaphore.
        let command_buffer_available = {
            let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
                s_type: vk::StructureType::SemaphoreTypeCreateInfo,
                p_next: null(),
                semaphore_type: vk::SemaphoreType::Timeline,
                initial_value: 0,
            };
            device.create_semaphore(&vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SemaphoreCreateInfo,
                p_next: addr_of!(semaphore_type_create_info).cast(),
                flags: vk::SemaphoreCreateFlags::empty(),
            })?
        };

        // Per-frame resources.
        let mut command_buffers = vec![];
        let mut present_complete = vec![];
        let mut rendering_complete = vec![];
        for _ in 0..create_info.image_count {
            // Command buffer.
            command_buffers.push({
                let mut command_buffer = MaybeUninit::uninit();
                device.allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo {
                        s_type: vk::StructureType::CommandBufferAllocateInfo,
                        p_next: null(),
                        command_pool,
                        level: vk::CommandBufferLevel::Primary,
                        command_buffer_count: 1,
                    },
                    command_buffer.as_mut_ptr(),
                )?;
                command_buffer.assume_init()
            });

            // Present complete -semaphore.
            present_complete.push(device.create_semaphore(&vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SemaphoreCreateInfo,
                p_next: null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            })?);

            // Rendering complete -semaphore.
            rendering_complete.push(device.create_semaphore(&vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SemaphoreCreateInfo,
                p_next: null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            })?);
        }

        Ok(Self {
            command_pool,
            command_buffer_available,
            command_buffers,
            present_completes: present_complete,
            rendering_completes: rendering_complete,
        })
    }

    unsafe fn destroy(self, Gpu { device, .. }: &Gpu) {
        device.destroy_command_pool(self.command_pool);
        device.destroy_semaphore(self.command_buffer_available);
        for &present_complete in &self.present_completes {
            device.destroy_semaphore(present_complete);
        }
        for &rendering_complete in &self.rendering_completes {
            device.destroy_semaphore(rendering_complete);
        }
    }
}

impl Commands {
    fn command_buffer_available(&self) -> vk::Semaphore {
        self.command_buffer_available
    }

    fn command_buffer(&self, frame_index: u64) -> vk::CommandBuffer {
        self.command_buffers[frame_index as usize]
    }

    fn present_complete(&self, frame_index: u64) -> vk::Semaphore {
        self.present_completes[frame_index as usize]
    }

    fn rendering_complete(&self, frame_index: u64) -> vk::Semaphore {
        self.rendering_completes[frame_index as usize]
    }
}

//
// Execute
//

unsafe fn execute(
    gpu: &Gpu,
    window_system: &mut WindowSystem,
    renderer: &Renderer,
    _: &str,
) -> Result<()> {
    use winit::event::Event;
    use winit::event::WindowEvent;
    use winit::event_loop::ControlFlow;
    use winit::platform::run_return::EventLoopExtRunReturn;

    let mut frame_index = 0_u64;
    let mut frame_count = 0_u64;
    let mut frame_result = None;
    window_system
        .event_loop
        .run_return(|event, _, control_flow| {
            // Default control flow.
            *control_flow = ControlFlow::Poll;

            // Event handler.
            match event {
                Event::NewEvents(cause) => {
                    info!("Frame {frame_count},{frame_index}: NewEvents: cause={cause:?}");
                }
                Event::WindowEvent { window_id, event } => {
                    info!("Frame {frame_count},{frame_index}: WindowEvent: window={window_id:?}, event={event:?}");
                    match (window_id, event) {
                        (id, WindowEvent::CloseRequested) if id == window_system.window.id() => {
                            frame_result = Some(Ok(()));
                            *control_flow = ControlFlow::ExitWithCode(0);
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { device_id, event } => {
                    info!("Frame {frame_count},{frame_index}: DeviceEvent: device_id={device_id:?}, event={event:?}");
                }
                Event::UserEvent(_) => {
                    info!("Frame {frame_count},{frame_index}: UserEvent");
                }
                Event::Suspended => {
                    info!("Frame {frame_count},{frame_index}: Suspended");
                }
                Event::Resumed => {
                    info!("Frame {frame_count},{frame_index}: Resumed");
                }
                Event::MainEventsCleared => {
                    info!("Frame {frame_count},{frame_index}: MainEventsCleared");
                    window_system.window.request_redraw();
                }
                Event::RedrawRequested(window_id) => {
                    info!("Frame {frame_count},{frame_index}: RedrawRequested: window={window_id:?}");
                    if let result @ Err(_) = redraw(gpu, renderer, frame_index, frame_count) {
                        frame_result = Some(result);
                        *control_flow = ControlFlow::ExitWithCode(1);
                    }
                }
                Event::RedrawEventsCleared => {
                    info!("Frame {frame_count},{frame_index}: RedrawEventsCleared");

                    // Update indices.
                    frame_index = (frame_index + 1) % renderer.swapchain.image_count();
                    frame_count += 1;

                    // Exit after n frames.
                    if frame_count == 10 {
                        frame_result = Some(Ok(()));
                        *control_flow = ControlFlow::ExitWithCode(0);
                    }

                    // Delay for readability.
                    std::thread::sleep(Duration::from_millis(250));
                }
                Event::LoopDestroyed => {
                    info!("Frame {frame_count},{frame_index}: LoopDestroyed");
                }
            }
        });

    frame_result.expect("Event loop should return a result on exit")
}

unsafe fn redraw(
    Gpu { device, .. }: &Gpu,
    Renderer {
        swapchain,
        commands,
        ..
    }: &Renderer,
    frame_index: u64,
    frame_count: u64,
) -> Result<()> {
    // Wait until a command buffer is available.
    device.wait_semaphores(
        &vk::SemaphoreWaitInfo {
            s_type: vk::StructureType::SemaphoreWaitInfo,
            p_next: null(),
            flags: vk::SemaphoreWaitFlagBits::Any.into(),
            semaphore_count: 1,
            p_semaphores: &commands.command_buffer_available(),
            p_values: &frame_count,
        },
        u64::MAX,
    )?;

    // Acquire image.
    let image_index = device.acquire_next_image2_khr(
        &(vk::AcquireNextImageInfoKHR {
            s_type: vk::StructureType::AcquireNextImageInfoKHR,
            p_next: null(),
            swapchain: swapchain.swapchain,
            timeout: u64::MAX,
            semaphore: commands.present_complete(frame_index),
            fence: vk::Fence::null(),
            device_mask: 1,
        }),
    )?;

    // Begin command buffer.
    let command_buffer = commands.command_buffer(frame_index);
    device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;
    device.begin_command_buffer(
        command_buffer,
        &(vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: null(),
            flags: vk::CommandBufferUsageFlagBits::OneTimeSubmit.into(),
            p_inheritance_info: null(),
        }),
    )?;

    // Transition swapchain image.
    device.cmd_pipeline_barrier2(
        command_buffer,
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
                src_access_mask: vk::AccessFlagBits2::None.into(),
                dst_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                dst_access_mask: vk::AccessFlagBits2::ColorAttachmentWrite.into(),
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::AttachmentOptimal,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: swapchain.image(image_index),
                subresource_range: swapchain.image_subresource_range(),
            },
        },
    );

    // Begin rendering.
    let clear_color = {
        use palette::{FromColor, Hsl, Srgb};
        let frame_period = 10;
        let hue = ((frame_count % frame_period) as f32 + 0.5) / frame_period as f32;
        let hsl = Hsl::new(360.0 * hue, 0.75, 0.75);
        let rgb = Srgb::from_color(hsl);
        [rgb.red, rgb.green, rgb.blue, 1.0]
    };
    device.cmd_begin_rendering(
        command_buffer,
        &(vk::RenderingInfo {
            s_type: vk::StructureType::RenderingInfo,
            p_next: null(),
            flags: vk::RenderingFlags::empty(),
            render_area: swapchain.render_area(),
            layer_count: 1,
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachments: &(vk::RenderingAttachmentInfo {
                s_type: vk::StructureType::RenderingAttachmentInfo,
                p_next: null(),
                image_view: swapchain.image_view(image_index),
                image_layout: vk::ImageLayout::AttachmentOptimal,
                resolve_mode: vk::ResolveModeFlagBits::None,
                resolve_image_view: vk::ImageView::null(),
                resolve_image_layout: vk::ImageLayout::Undefined,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: clear_color,
                    },
                },
            }),
            p_depth_attachment: null(),
            p_stencil_attachment: null(),
        }),
    );

    // End rendering.
    device.cmd_end_rendering(command_buffer);

    // Transition swapchain image.
    device.cmd_pipeline_barrier2(
        command_buffer,
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
                dst_stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                dst_access_mask: vk::AccessFlagBits2::None.into(),
                old_layout: vk::ImageLayout::AttachmentOptimal,
                new_layout: vk::ImageLayout::PresentSrcKHR,
                src_queue_family_index: 0,
                dst_queue_family_index: 0,
                image: swapchain.image(image_index),
                subresource_range: swapchain.image_subresource_range(),
            },
        },
    );

    // End command buffer.
    device.end_command_buffer(command_buffer)?;

    // Queue submit.
    device.queue_submit2(
        device.queue,
        1,
        &(vk::SubmitInfo2 {
            s_type: vk::StructureType::SubmitInfo2,
            p_next: null(),
            flags: vk::SubmitFlags::empty(),
            wait_semaphore_info_count: 1,
            p_wait_semaphore_infos: &vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SemaphoreSubmitInfo,
                p_next: null(),
                semaphore: commands.present_complete(frame_index),
                value: 0,
                stage_mask: vk::PipelineStageFlagBits2::ColorAttachmentOutput.into(),
                device_index: 0,
            },
            command_buffer_info_count: 1,
            p_command_buffer_infos: &vk::CommandBufferSubmitInfo {
                s_type: vk::StructureType::CommandBufferSubmitInfo,
                p_next: null(),
                command_buffer,
                device_mask: 0,
            },
            signal_semaphore_info_count: 2,
            p_signal_semaphore_infos: [
                vk::SemaphoreSubmitInfo {
                    s_type: vk::StructureType::SemaphoreSubmitInfo,
                    p_next: null(),
                    semaphore: commands.rendering_complete(frame_index),
                    value: 0,
                    stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
                    device_index: 0,
                },
                vk::SemaphoreSubmitInfo {
                    s_type: vk::StructureType::SemaphoreSubmitInfo,
                    p_next: null(),
                    semaphore: commands.command_buffer_available(),
                    value: frame_count + 1,
                    stage_mask: vk::PipelineStageFlagBits2::AllCommands.into(),
                    device_index: 0,
                },
            ]
            .as_ptr(),
        }),
        vk::Fence::null(),
    )?;

    // Queue present.
    {
        let mut result = MaybeUninit::uninit();
        let present_info_khr = vk::PresentInfoKHR {
            s_type: vk::StructureType::PresentInfoKHR,
            p_next: null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: &commands.rendering_complete(frame_index),
            swapchain_count: 1,
            p_swapchains: &swapchain.swapchain,
            p_image_indices: &image_index,
            p_results: result.as_mut_ptr(),
        };
        device.queue_present_khr(device.queue, &present_info_khr)?;
        let result = result.assume_init();
        ensure!(result == vk::Result::Success);
    }

    Ok(())
}
