use super::*;

//
// Modules
//

pub mod clear;
pub mod compute;
pub mod cube;
pub mod gui;
pub mod raytracing;
pub mod triangle;
pub mod window;

//
// Aliases
//

pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Mat4 = glam::Mat4;

pub use clear::Demo as ClearDemo;
pub use compute::Demo as ComputeDemo;
pub use cube::Demo as CubeDemo;
pub use gui::Demo as GuiDemo;
pub use raytracing::Demo as RaytracingDemo;
pub use triangle::Demo as TriangleDemo;
#[cfg(target_family = "windows")]
pub use window::Demo as WindowDemo;

//
// Constants
//

pub const DEFAULT_RENDER_TARGET_WIDTH: u32 = 256;
pub const DEFAULT_RENDER_TARGET_HEIGHT: u32 = 256;
pub const DEFAULT_RENDER_TARGET_COLOR_FORMAT: vk::Format = vk::Format::R8g8b8a8Unorm;
pub const DEFAULT_RENDER_TARGET_DEPTH_FORMAT: vk::Format = vk::Format::D32Sfloat;
pub const DEFAULT_RENDER_TARGET_RESOLVE_FORMAT: vk::Format = vk::Format::R8g8b8a8Unorm;
pub const DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE: vk::DeviceSize =
    (DEFAULT_RENDER_TARGET_COLOR_FORMAT.block_size() as vk::DeviceSize)
        * (DEFAULT_RENDER_TARGET_WIDTH as vk::DeviceSize)
        * (DEFAULT_RENDER_TARGET_HEIGHT as vk::DeviceSize);
pub const DEFAULT_RENDER_TARGET_SAMPLES: vk::SampleCountFlagBits = vk::SampleCountFlagBits::Count8;
pub const DEFAULT_RENDER_TARGET_CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

//
// Utilities
//

struct OutputImage {
    buffer: vkx::BufferDedicatedTransfer,
}

impl OutputImage {
    unsafe fn create(physical_device: &vkx::PhysicalDevice, device: &vkx::Device) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            buffer: vkx::BufferDedicatedTransfer::create(
                physical_device,
                device,
                vkx::BufferCreator::new(
                    DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE,
                    vk::BufferUsageFlagBits::TransferDst,
                ),
                vk::MemoryPropertyFlagBits::HostVisible,
            )?,
        })
    }

    unsafe fn destroy(self, device: &vkx::Device) {
        self.buffer.destroy(device);
    }

    unsafe fn write_to_path(&self, path: &Path) -> Result<()> {
        use imagelib::{ImageFormat, RgbaImage};
        let width = DEFAULT_RENDER_TARGET_WIDTH;
        let height = DEFAULT_RENDER_TARGET_HEIGHT;
        let byte_size = DEFAULT_RENDER_TARGET_COLOR_BYTE_SIZE;
        let mut pixels = vec![0_u8; byte_size as _];
        pixels.copy_from_slice(self.buffer.memory().as_slice(byte_size as _));

        let image = RgbaImage::from_raw(width, height, pixels)
            .context("Creating image from output buffer")?;
        image.save_with_format(path, ImageFormat::Png)?;
        info!("Wrote {width}x{height} image to {}", path.display());
        Ok(())
    }
}
