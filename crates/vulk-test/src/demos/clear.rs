use super::*;

//
// Demo
//

pub struct Demo {
    command_buffer: vkx::CommandBuffer,
    command_buffer_done: vkx::TimelineSemaphore,
    timestamps: vkx::TimestampQuery,
    statistics: vkx::StatisticsQuery,
    render_targets: RenderTargets,
    output: OutputImage,
}

impl DemoCallbacks for Demo {
    const NAME: &'static str = "clear";

    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized,
    {
        let command_buffer = vkx::CommandBuffer::create(&gpu.device)?;
        let command_buffer_done = vkx::TimelineSemaphore::create(&gpu.device, 0)?;
        let timestamps = vkx::TimestampQuery::create(&gpu.physical_device, &gpu.device, 2)?;
        let statistics = vkx::StatisticsQuery::create(&gpu.device)?;
        let render_targets = RenderTargets::create(gpu, &())?;
        let output = OutputImage::create(&gpu.physical_device, &gpu.device)?;
        Ok(Self {
            command_buffer,
            command_buffer_done,
            timestamps,
            statistics,
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
        state.statistics.destroy(&gpu.device);
        state.timestamps.destroy(&gpu.device);
        state.command_buffer_done.destroy(&gpu.device);
        state.command_buffer.destroy(&gpu.device);
        Ok(())
    }
}

//
// Render targets
//

struct RenderTargets {
    color: vkx::ImageDedicatedResource,
}

impl GpuResource for RenderTargets {
    type CreateInfo<'a> = ();

    unsafe fn create(gpu: &Gpu, _: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized,
    {
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
            vk::MemoryPropertyFlagBits::DeviceLocal.into(),
        )?;
        Ok(Self { color })
    }

    unsafe fn destroy(self, gpu: &Gpu) {
        self.color.destroy(&gpu.device);
    }
}

//
// Draw
//

unsafe fn draw(
    Gpu { device, .. }: &Gpu,
    Demo {
        command_buffer,
        command_buffer_done,
        timestamps,
        statistics,
        render_targets,
        output,
    }: &Demo,
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
    }

    // Write image.
    output.write_to_path(&work_dir_or_create()?.join(format!("{demo_name}.png")))?;

    Ok(())
}
