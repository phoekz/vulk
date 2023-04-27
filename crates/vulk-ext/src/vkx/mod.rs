use super::*;

//
// Modules
//

mod buffer;
mod debug_utils;
mod descriptor;
mod device;
mod image;
mod instance;
mod memory;
mod physical_device;
mod sampler;
mod semaphore;
mod shader;
mod surface;
mod swapchain;

//
// Re-exports
//

pub use buffer::{
    BufferCreator, BufferDedicatedResource, BufferDedicatedTransfer, BufferOps, BufferResource,
    BufferResourceOps, BufferShaderBindingTable,
};
pub use descriptor::{Descriptor, DescriptorBinding, DescriptorCreateInfo, DescriptorStorage};
pub use device::Device;
pub use image::{ImageCreator, ImageDedicatedResource, ImageOps, ImageResource, ImageViewCreator};
pub use instance::{Instance, InstanceCreateInfo};
pub use memory::{BufferAllocation, BufferAllocations, ImageAllocations};
pub use physical_device::PhysicalDevice;
pub use sampler::{SamplerCreator, SamplerResource};
pub use semaphore::{BinarySemaphore, TimelineSemaphore};
pub use shader::{Shader, ShaderBinary, ShaderCompiler, ShaderCreateInfo, ShaderType};
pub use surface::Surface;
pub use swapchain::Swapchain;

pub(crate) use debug_utils::DebugUtils;

pub mod prelude {
    use super::*;

    pub use buffer::{BufferOps, BufferResourceOps};
    pub use image::ImageOps;
}
