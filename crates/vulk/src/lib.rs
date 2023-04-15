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
//! ## Shaders
//! - [`vk::CreateShadersEXT`] Create one or more new shaders
//! - [`vk::CmdBindShadersEXT`] Bind shader objects to a command buffer
//! - [`vk::DestroyShaderEXT`] Destroy a shader object
//! ## Memory Allocation
//! - [`vk::GetPhysicalDeviceMemoryProperties2`] Reports memory information for the specified physical device
//! - [`vk::AllocateMemory`] Allocate device memory
//! - [`vk::FreeMemory`] Free device memory
//! - [`vk::MapMemory2KHR`] Map a memory object into application address space
//! - [`vk::UnmapMemory2KHR`] Unmap a previously mapped memory object
//! ## Resource Creation
//! - [`vk::CreateBuffer`] Create a new buffer object
//! - [`vk::DestroyBuffer`] Destroy a buffer object
//! - [`vk::GetDeviceBufferMemoryRequirements`] Returns the memory requirements for specified Vulkan object
//! - [`vk::BindBufferMemory2`] Bind device memory to buffer objects
//! ## Resource Descriptors
//! - [`vk::CreateDescriptorSetLayout`] Create a new descriptor set layout
//! - [`vk::DestroyDescriptorSetLayout`] Destroy a descriptor set layout object
//! - [`vk::CreatePipelineLayout`] Creates a new pipeline layout object
//! - [`vk::DestroyPipelineLayout`] Destroy a pipeline layout object
//! - [`vk::GetBufferDeviceAddress`] Query an address of a buffer
//! - [`vk::GetDescriptorSetLayoutSizeEXT`] Get the size of a descriptor set layout in memory
//! - [`vk::GetDescriptorEXT`] To get a descriptor to place in a buffer
//! - [`vk::CmdBindDescriptorBuffersEXT`] Binding descriptor buffers to a command buffer
//! - [`vk::CmdSetDescriptorBufferOffsetsEXT`] Setting descriptor buffer offsets in a command buffer
//! ## Dispatching Commands
//! - [`vk::CmdDispatch`] Dispatch compute work items
//! - [`vk::CmdDispatchIndirect`] Dispatch compute work items with indirect parameters
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
    clippy::unreadable_literal
)]

pub mod loader;
pub mod vk;
