use super::*;

//
// Modules
//

mod buffer;
mod command_buffer;
mod debug_utils;
mod descriptor;
mod device;
mod image;
mod instance;
mod memory;
mod physical_device;
mod query;
mod queue;
mod sampler;
mod semaphore;
mod shader;
mod surface;
mod swapchain;
mod transfer;

//
// Re-exports
//

pub use buffer::{
    BufferCreator, BufferDedicatedResource, BufferDedicatedTransfer, BufferOps, BufferResource,
    BufferResourceOps, BufferShaderBindingTable,
};
pub use command_buffer::CommandBuffer;
pub use descriptor::{Descriptor, DescriptorBinding, DescriptorCreateInfo, DescriptorStorage};
pub use device::Device;
pub use image::{ImageCreator, ImageDedicatedResource, ImageOps, ImageResource, ImageViewCreator};
pub use instance::{Instance, InstanceCreateInfo};
pub use memory::{BufferAllocation, BufferAllocations, ImageAllocations};
pub use physical_device::PhysicalDevice;
pub use query::{StatisticsQuery, TimestampQuery};
pub use queue::queue_submit;
pub use sampler::{SamplerCreator, SamplerResource};
pub use semaphore::{BinarySemaphore, SemaphoreOps, TimelineSemaphore};
pub use shader::{Shader, ShaderBinary, ShaderCompiler, ShaderCreateInfo, ShaderType};
pub use surface::Surface;
pub use swapchain::Swapchain;
pub use transfer::transfer_resources;

pub(crate) use debug_utils::DebugUtils;

pub mod prelude {
    use super::*;

    pub use buffer::{BufferOps, BufferResourceOps};
    pub use image::ImageOps;
    pub use semaphore::SemaphoreOps;
}
