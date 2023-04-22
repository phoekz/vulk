//! # `vulk`
//! ## Initialization
//! - [`vk::GetInstanceProcAddr`] Return a function pointer for a command
//! - [`vk::GetDeviceProcAddr`] Return a function pointer for a command
//! - [`vk::CreateInstance`] Create a new Vulkan instance
//! - [`vk::DestroyInstance`] Destroy an instance of Vulkan
//! ## Devices and Queues
//! - [`vk::EnumeratePhysicalDevices`] Enumerates the physical devices accessible to a Vulkan instance
//! - [`vk::GetPhysicalDeviceProperties2`] Returns properties of a physical device
//! - [`vk::GetPhysicalDeviceQueueFamilyProperties2`] Reports properties of the queues of the specified physical device
//! - [`vk::CreateDevice`] Create a new device instance
//! - [`vk::DestroyDevice`] Destroy a logical device
//! - [`vk::GetDeviceQueue2`] Get a queue handle from a device
//! ## Command Buffers
//! - [`vk::CreateCommandPool`] Create a new command pool object
//! - [`vk::ResetCommandPool`] Reset a command pool
//! - [`vk::DestroyCommandPool`] Destroy a command pool object
//! - [`vk::AllocateCommandBuffers`] Allocate command buffers from an existing command pool
//! - [`vk::FreeCommandBuffers`] Free command buffers
//! - [`vk::BeginCommandBuffer`] Start recording a command buffer
//! - [`vk::EndCommandBuffer`] Finish recording a command buffer
//! - [`vk::QueueSubmit2`] Submits command buffers to a queue
//! ## Synchronization and Cache Control
//! - [`vk::CreateSemaphore`] Create a new queue semaphore object
//! - [`vk::DestroySemaphore`] Destroy a semaphore object
//! - [`vk::WaitSemaphores`] Wait for timeline semaphores on the host
//! - [`vk::CmdPipelineBarrier2`] Insert a memory dependency
//! - [`vk::QueueWaitIdle`] Wait for a queue to become idle
//! - [`vk::DeviceWaitIdle`] Wait for a device to become idle
//! - [`vk::GetCalibratedTimestampsEXT`] Query calibrated timestamps
//! ## Render Pass
//! - [`vk::CmdBeginRendering`] Begin a dynamic render pass instance
//! - [`vk::CmdEndRendering`] End a dynamic render pass instance
//! ## Shaders
//! - [`vk::CreateShadersEXT`] Create one or more new shaders
//! - [`vk::CmdBindShadersEXT`] Bind shader objects to a command buffer
//! - [`vk::DestroyShaderEXT`] Destroy a shader object
//! - [`vk::CreateShaderModule`] Creates a new shader module object
//! - [`vk::DestroyShaderModule`] Destroy a shader module
//! ## Pipelines
//! - [`vk::CreateRayTracingPipelinesKHR`] Creates a new ray tracing pipeline object
//! - [`vk::GetRayTracingShaderGroupHandlesKHR`] Query ray tracing pipeline shader group handles
//! - [`vk::DestroyPipeline`] Destroy a pipeline object
//! - [`vk::CmdBindPipeline`] Bind a pipeline object to a command buffer
//! ## Memory Allocation
//! - [`vk::GetPhysicalDeviceMemoryProperties2`] Reports memory information for the specified physical device
//! - [`vk::AllocateMemory`] Allocate device memory
//! - [`vk::FreeMemory`] Free device memory
//! - [`vk::MapMemory2KHR`] Map a memory object into application address space
//! - [`vk::UnmapMemory2KHR`] Unmap a previously mapped memory object
//! ## Resource Creation
//! - [`vk::CreateBuffer`] Create a new buffer object
//! - [`vk::DestroyBuffer`] Destroy a buffer object
//! - [`vk::CreateImage`] Create a new image object
//! - [`vk::DestroyImage`] Destroy an image object
//! - [`vk::CreateImageView`] Create an image view from an existing image
//! - [`vk::DestroyImageView`] Destroy an image view object
//! - [`vk::CreateAccelerationStructureKHR`] Create a new acceleration structure object
//! - [`vk::GetAccelerationStructureBuildSizesKHR`] Retrieve the required size for an acceleration structure
//! - [`vk::DestroyAccelerationStructureKHR`] Destroy an acceleration structure object
//! - [`vk::GetAccelerationStructureDeviceAddressKHR`] Query an address of a acceleration structure
//! - [`vk::GetDeviceBufferMemoryRequirements`] Returns the memory requirements for specified Vulkan object
//! - [`vk::GetDeviceImageMemoryRequirements`] Returns the memory requirements for specified Vulkan object
//! - [`vk::BindBufferMemory2`] Bind device memory to buffer objects
//! - [`vk::BindImageMemory2`] Bind device memory to image objects
//! ## Samplers
//! - [`vk::CreateSampler`] Create a new sampler object
//! - [`vk::DestroySampler`] Destroy a sampler object
//! ## Resource Descriptors
//! - [`vk::CreateDescriptorSetLayout`] Create a new descriptor set layout
//! - [`vk::DestroyDescriptorSetLayout`] Destroy a descriptor set layout object
//! - [`vk::CreatePipelineLayout`] Creates a new pipeline layout object
//! - [`vk::DestroyPipelineLayout`] Destroy a pipeline layout object
//! - [`vk::CmdPushConstants`] Update the values of push constants
//! - [`vk::GetBufferDeviceAddress`] Query an address of a buffer
//! - [`vk::GetDescriptorSetLayoutSizeEXT`] Get the size of a descriptor set layout in memory
//! - [`vk::GetDescriptorSetLayoutBindingOffsetEXT`] Get the offset of a binding within a descriptor set layout
//! - [`vk::GetDescriptorEXT`] To get a descriptor to place in a buffer
//! - [`vk::CmdBindDescriptorBuffersEXT`] Binding descriptor buffers to a command buffer
//! - [`vk::CmdSetDescriptorBufferOffsetsEXT`] Setting descriptor buffer offsets in a command buffer
//! ## Queries
//! - [`vk::CreateQueryPool`] Create a new query pool object
//! - [`vk::DestroyQueryPool`] Destroy a query pool object
//! - [`vk::ResetQueryPool`] Reset queries in a query pool
//! - [`vk::CmdBeginQuery`] Begin a query
//! - [`vk::CmdEndQuery`] Ends a query
//! - [`vk::GetQueryPoolResults`] Copy results of queries in a query pool to a host memory region
//! - [`vk::CmdWriteTimestamp2`] Write a device timestamp into a query object
//! ## Copy Commands
//! - [`vk::CmdCopyBufferToImage2`] Copy data from a buffer into an image
//! - [`vk::CmdCopyImageToBuffer2`] Copy image data into a buffer
//! ## Drawing Commands
//! - [`vk::CmdDrawMeshTasksEXT`] Draw mesh task work items
//! - [`vk::CmdDrawMeshTasksIndirectEXT`] Issue an indirect mesh tasks draw into a command buffer
//! - [`vk::CmdDrawMeshTasksIndirectCountEXT`] Perform an indirect mesh tasks draw with the draw count sourced from a buffer
//! ## Fixed-Function Vertex Post-Processing
//! - [`vk::CmdSetViewportWithCount`] Set the viewport count and viewports dynamically for a command buffer
//! - [`vk::CmdSetScissorWithCount`] Set the scissor count and scissor rectangular bounds dynamically for a command buffer
//! ## Rasterization
//! - [`vk::CmdSetRasterizationSamplesEXT`] Specify the rasterization samples dynamically for a command buffer
//! - [`vk::CmdSetFrontFace`] Set front face orientation dynamically for a command buffer
//! - [`vk::CmdSetCullMode`] Set cull mode dynamically for a command buffer
//! ## Fragment Operations
//! - [`vk::CmdSetDepthTestEnable`] Set depth test enable dynamically for a command buffer
//! - [`vk::CmdSetDepthCompareOp`] Set depth comparison operator dynamically for a command buffer
//! - [`vk::CmdSetDepthWriteEnable`] Set depth write enable dynamically for a command buffer
//! ## The Framebuffer
//! - [`vk::CmdSetColorBlendEnableEXT`] Specify the pname:blendEnable for each attachment dynamically for a command buffer
//! - [`vk::CmdSetColorBlendEquationEXT`] Specify the blend factors and operations dynamically for a command buffer
//! - [`vk::CmdSetColorWriteMaskEXT`] Specify the color write masks for each attachment dynamically for a command buffer
//! ## Dispatching Commands
//! - [`vk::CmdDispatch`] Dispatch compute work items
//! - [`vk::CmdDispatchIndirect`] Dispatch compute work items with indirect parameters
//! ## Acceleration Structures
//! - [`vk::CmdBuildAccelerationStructuresKHR`] Build an acceleration structure
//! ## Ray Tracing
//! - [`vk::CmdTraceRaysKHR`] Initialize a ray tracing dispatch
//! - [`vk::CmdTraceRaysIndirect2KHR`] Initialize an indirect ray tracing dispatch with indirect shader binding tables
//! ## Additional Capabilities
//! - [`vk::GetPhysicalDeviceCalibrateableTimeDomainsEXT`] Query calibrateable time domains
//! ## Debugging
//! - [`vk::CreateDebugUtilsMessengerEXT`] Create a debug messenger object
//! - [`vk::DestroyDebugUtilsMessengerEXT`] Destroy a debug messenger object

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

//
// Modules
//

pub mod loader;
pub mod vk;
#[cfg(test)]
mod tests;

//
// Re-exports
//

pub use loader::Init;
pub use loader::Instance;
pub use loader::Device;

//
// Error
//

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to load dynamic library {0}")]
    LibraryLoad(std::borrow::Cow<'static, str>),
    #[error("failed to load function {0}")]
    FunctionLoad(std::borrow::Cow<'static, str>),
    #[error("vulkan error {0:?}")]
    Vulkan(vk::Result),
}

//
// Utilities
//

pub unsafe fn read_to_vec<F, T>(f: F, s_type: Option<vk::StructureType>) -> Result<Vec<T>, Error>
where
    F: Fn(*mut u32, *mut T) -> Result<(), Error>,
{
    use std::alloc::alloc_zeroed;
    use std::alloc::Layout;
    use std::mem::size_of;
    use std::mem::size_of_val;
    use std::ptr::addr_of;

    // Query the number of elements.
    let mut len_u32 = 0_u32;
    f(&mut len_u32, std::ptr::null_mut())?;
    let len = len_u32 as usize;

    // Allocate.
    let layout = Layout::from_size_align(len * size_of::<T>(), 16).unwrap();
    let ptr = alloc_zeroed(layout);

    // Note: if the output type contains `s_type`, we have to write the
    // structure type on each of the output elements.
    if let Some(s_type) = s_type {
        for i in 0..len {
            let dst = ptr.add(i * size_of::<T>());
            dst.copy_from(addr_of!(s_type).cast(), size_of_val(&s_type));
        }
    }

    // Query elements.
    f(&mut len_u32, ptr.cast())?;

    // Build the Vec.
    let vec = Vec::from_raw_parts(ptr.cast::<T>(), len, len);

    Ok(vec)
}
