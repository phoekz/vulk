//
// Imports
//

use bitflags::bitflags;
use std::ffi::{c_char, c_void};

//
// Defines
//

#[must_use]
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

//
// API Constants
//

#[doc = "**Description**: Length of a physical device name string"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html)"]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;

#[doc = "**Description**: Length of a universally unique device or driver build identifier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_UUID_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_UUID_SIZE.html)"]
pub const UUID_SIZE: u32 = 16;

#[doc = "**Description**: Length of a locally unique device identifier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_LUID_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html)"]
pub const LUID_SIZE: u32 = 8;

#[doc = "**Description**: Maximum length of a layer of extension name string"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_EXTENSION_NAME_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_EXTENSION_NAME_SIZE.html)"]
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;

#[doc = "**Description**: Length of a driver name string"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_DESCRIPTION_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DESCRIPTION_SIZE.html)"]
pub const MAX_DESCRIPTION_SIZE: u32 = 256;

#[doc = "**Description**: Length of an array of memory types"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_MEMORY_TYPES`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_TYPES.html)"]
pub const MAX_MEMORY_TYPES: u32 = 32;

#[doc = "**Description**: Length of an array of memory heaps"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_MEMORY_HEAPS`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_HEAPS.html)"]
pub const MAX_MEMORY_HEAPS: u32 = 16;

#[doc = "**Description**: Maximum level of detail unclamped access sentinel"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_LOD_CLAMP_NONE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LOD_CLAMP_NONE.html)"]
pub const LOD_CLAMP_NONE: f32 = 1000.0;

#[doc = "**Description**: Sentinel for all remaining mipmap levels"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_REMAINING_MIP_LEVELS`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_MIP_LEVELS.html)"]
pub const REMAINING_MIP_LEVELS: u32 = !0;

#[doc = "**Description**: Sentinel for all remaining array layers"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_REMAINING_ARRAY_LAYERS`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_ARRAY_LAYERS.html)"]
pub const REMAINING_ARRAY_LAYERS: u32 = !0;

#[doc = "**Description**: Sentinel for all remaining 3D slices"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_image_sliced_view_of_3d`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_sliced_view_of_3d.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_REMAINING_3D_SLICES_EXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_3D_SLICES_EXT.html)"]
pub const REMAINING_3D_SLICES_EXT: u32 = !0;

#[doc = "**Description**: Sentinel value to use entire remaining array length"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_WHOLE_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_WHOLE_SIZE.html)"]
pub const WHOLE_SIZE: u64 = !0;

#[doc = "**Description**: Unused attachment sentinel"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_ATTACHMENT_UNUSED`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ATTACHMENT_UNUSED.html)"]
pub const ATTACHMENT_UNUSED: u32 = !0;

#[doc = "**Description**: Boolean true value"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_TRUE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_TRUE.html)"]
pub const TRUE: u32 = 1;

#[doc = "**Description**: Boolean false value"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_FALSE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FALSE.html)"]
pub const FALSE: u32 = 0;

#[doc = "**Description**: Ignored queue family index sentinel"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_QUEUE_FAMILY_IGNORED`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_IGNORED.html)"]
pub const QUEUE_FAMILY_IGNORED: u32 = !0;

#[doc = "**Description**: External queue family index sentinel"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_QUEUE_FAMILY_EXTERNAL`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL.html)"]
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;

#[doc = "**Description**: Foreign queue family index sentinel"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_queue_family_foreign`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_queue_family_foreign.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_QUEUE_FAMILY_FOREIGN_EXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html)"]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;

#[doc = "**Description**: Subpass index sentinel expanding synchronization scope outside a subpass"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_SUBPASS_EXTERNAL`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SUBPASS_EXTERNAL.html)"]
pub const SUBPASS_EXTERNAL: u32 = !0;

#[doc = "**Description**: Length of a physical device handle array"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_DEVICE_GROUP_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE.html)"]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;

#[doc = "**Description**: Maximum length of a physical device driver name string"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_DRIVER_NAME_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE.html)"]
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;

#[doc = "**Description**: Length of a physical device driver information string"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_DRIVER_INFO_SIZE`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html)"]
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;

#[doc = "**Description**: Sentinel for an unused shader index"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_ray_tracing_pipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_SHADER_UNUSED_KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html)"]
pub const SHADER_UNUSED_KHR: u32 = !0;

#[doc = "**Description**: Length of an array of global queue priorities"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_global_priority`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_global_priority.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_GLOBAL_PRIORITY_SIZE_KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_KHR.html)"]
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;

#[doc = "**Description**: Maximum length of a shader module identifier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_module_identifier`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_module_identifier.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html)"]
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;

//
// Base types
//

#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Vulkan boolean type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBool32`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBool32.html)"]
pub type Bool32 = u32;

#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Vulkan device memory size and offsets"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceSize.html)"]
pub type DeviceSize = u64;

#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Vulkan device address type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceAddress`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddress.html)"]
pub type DeviceAddress = u64;

//
// Function pointers
//

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Placeholder function pointer type returned by queries"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkVoidFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkVoidFunction.html)"]
pub type PfnVoidFunction = *const c_void;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined memory allocation function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkAllocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html)"]
pub type PfnAllocationFunction = *const c_void;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined memory reallocation function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkReallocationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html)"]
pub type PfnReallocationFunction = *const c_void;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined memory free function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkFreeFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html)"]
pub type PfnFreeFunction = *const c_void;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined memory allocation notification function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkInternalAllocationNotification`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html)"]
pub type PfnInternalAllocationNotification = *const c_void;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined memory free notification function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkInternalFreeNotification`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html)"]
pub type PfnInternalFreeNotification = *const c_void;

#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Application-defined debug messenger callback function"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`PFN_vkDebugUtilsMessengerCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html)"]
pub type PfnDebugUtilsMessengerCallbackEXT = *const c_void;

//
// Handles
//

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to an instance object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstance.html)"]
pub struct Instance(u64);

impl Instance {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Instance").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a physical device object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html)"]
pub struct PhysicalDevice(u64);

impl PhysicalDevice {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PhysicalDevice").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a device object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevice.html)"]
pub struct Device(u64);

impl Device {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Device").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a queue object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueue.html)"]
pub struct Queue(u64);

impl Queue {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Queue").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a command buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html)"]
pub struct CommandBuffer(u64);

impl CommandBuffer {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for CommandBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CommandBuffer").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a command pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html)"]
pub struct CommandPool(u64);

impl CommandPool {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for CommandPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CommandPool").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a fence object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkFence`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html)"]
pub struct Fence(u64);

impl Fence {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Fence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Fence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Fence").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a semaphore object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html)"]
pub struct Semaphore(u64);

impl Semaphore {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Semaphore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Semaphore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Semaphore").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a render pass object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html)"]
pub struct RenderPass(u64);

impl RenderPass {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for RenderPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RenderPass").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a framebuffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html)"]
pub struct Framebuffer(u64);

impl Framebuffer {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Framebuffer").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a shader object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkShaderEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderEXT.html)"]
pub struct ShaderEXT(u64);

impl ShaderEXT {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for ShaderEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for ShaderEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ShaderEXT").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a device memory object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html)"]
pub struct DeviceMemory(u64);

impl DeviceMemory {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for DeviceMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for DeviceMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeviceMemory").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuffer.html)"]
pub struct Buffer(u64);

impl Buffer {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Buffer").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to an image object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImage`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html)"]
pub struct Image(u64);

impl Image {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Image").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to an image view object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageView`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html)"]
pub struct ImageView(u64);

impl ImageView {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for ImageView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for ImageView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImageView").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a sampler object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSampler`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampler.html)"]
pub struct Sampler(u64);

impl Sampler {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Sampler").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a descriptor set layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html)"]
pub struct DescriptorSetLayout(u64);

impl DescriptorSetLayout {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for DescriptorSetLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for DescriptorSetLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DescriptorSetLayout").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a pipeline layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html)"]
pub struct PipelineLayout(u64);

impl PipelineLayout {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for PipelineLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for PipelineLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PipelineLayout").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a query pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html)"]
pub struct QueryPool(u64);

impl QueryPool {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for QueryPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for QueryPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QueryPool").field(&format_args!("{self}")).finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Opaque handle to a debug messenger object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html)"]
pub struct DebugUtilsMessengerEXT(u64);

impl DebugUtilsMessengerEXT {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for DebugUtilsMessengerEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for DebugUtilsMessengerEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DebugUtilsMessengerEXT").field(&format_args!("{self}")).finish()
    }
}

//
// Enumerations
//

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Vulkan command return codes"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResult.html)"]
pub enum Result {
    #[doc = "**Translated from**: `VK_SUCCESS`"]
    Success = 0,
    #[doc = "**Translated from**: `VK_NOT_READY`"]
    NotReady = 1,
    #[doc = "**Translated from**: `VK_TIMEOUT`"]
    Timeout = 2,
    #[doc = "**Translated from**: `VK_EVENT_SET`"]
    EventSet = 3,
    #[doc = "**Translated from**: `VK_EVENT_RESET`"]
    EventReset = 4,
    #[doc = "**Translated from**: `VK_INCOMPLETE`"]
    Incomplete = 5,
    #[doc = "**Translated from**: `VK_ERROR_OUT_OF_HOST_MEMORY`"]
    ErrorOutOfHostMemory = -1,
    #[doc = "**Translated from**: `VK_ERROR_OUT_OF_DEVICE_MEMORY`"]
    ErrorOutOfDeviceMemory = -2,
    #[doc = "**Translated from**: `VK_ERROR_INITIALIZATION_FAILED`"]
    ErrorInitializationFailed = -3,
    #[doc = "**Translated from**: `VK_ERROR_DEVICE_LOST`"]
    ErrorDeviceLost = -4,
    #[doc = "**Translated from**: `VK_ERROR_MEMORY_MAP_FAILED`"]
    ErrorMemoryMapFailed = -5,
    #[doc = "**Translated from**: `VK_ERROR_LAYER_NOT_PRESENT`"]
    ErrorLayerNotPresent = -6,
    #[doc = "**Translated from**: `VK_ERROR_EXTENSION_NOT_PRESENT`"]
    ErrorExtensionNotPresent = -7,
    #[doc = "**Translated from**: `VK_ERROR_FEATURE_NOT_PRESENT`"]
    ErrorFeatureNotPresent = -8,
    #[doc = "**Translated from**: `VK_ERROR_INCOMPATIBLE_DRIVER`"]
    ErrorIncompatibleDriver = -9,
    #[doc = "**Translated from**: `VK_ERROR_TOO_MANY_OBJECTS`"]
    ErrorTooManyObjects = -10,
    #[doc = "**Translated from**: `VK_ERROR_FORMAT_NOT_SUPPORTED`"]
    ErrorFormatNotSupported = -11,
    #[doc = "**Translated from**: `VK_ERROR_FRAGMENTED_POOL`"]
    ErrorFragmentedPool = -12,
    #[doc = "**Translated from**: `VK_ERROR_UNKNOWN`"]
    ErrorUnknown = -13,
    #[doc = "**Translated from**: `VK_ERROR_OUT_OF_POOL_MEMORY`"]
    ErrorOutOfPoolMemory = -1000069000,
    #[doc = "**Translated from**: `VK_ERROR_INVALID_EXTERNAL_HANDLE`"]
    ErrorInvalidExternalHandle = -1000072003,
    #[doc = "**Translated from**: `VK_ERROR_FRAGMENTATION`"]
    ErrorFragmentation = -1000161000,
    #[doc = "**Translated from**: `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`"]
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
    #[doc = "**Translated from**: `VK_PIPELINE_COMPILE_REQUIRED`"]
    PipelineCompileRequired = 1000297000,
    #[doc = "**Translated from**: `VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT`"]
    ErrorIncompatibleShaderBinaryEXT = 1000482000,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Vulkan structure types (pname:sType)"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkStructureType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStructureType.html)"]
pub enum StructureType {
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_APPLICATION_INFO`"]
    ApplicationInfo = 0,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`"]
    InstanceCreateInfo = 1,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`"]
    DeviceQueueCreateInfo = 2,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`"]
    DeviceCreateInfo = 3,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBMIT_INFO`"]
    SubmitInfo = 4,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`"]
    MemoryAllocateInfo = 5,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`"]
    MappedMemoryRange = 6,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`"]
    BindSparseInfo = 7,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`"]
    FenceCreateInfo = 8,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`"]
    SemaphoreCreateInfo = 9,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`"]
    EventCreateInfo = 10,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`"]
    QueryPoolCreateInfo = 11,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`"]
    BufferCreateInfo = 12,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`"]
    BufferViewCreateInfo = 13,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`"]
    ImageCreateInfo = 14,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`"]
    ImageViewCreateInfo = 15,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`"]
    ShaderModuleCreateInfo = 16,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`"]
    PipelineCacheCreateInfo = 17,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`"]
    PipelineShaderStageCreateInfo = 18,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`"]
    PipelineVertexInputStateCreateInfo = 19,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`"]
    PipelineInputAssemblyStateCreateInfo = 20,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`"]
    PipelineTessellationStateCreateInfo = 21,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`"]
    PipelineViewportStateCreateInfo = 22,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`"]
    PipelineRasterizationStateCreateInfo = 23,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`"]
    PipelineMultisampleStateCreateInfo = 24,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`"]
    PipelineDepthStencilStateCreateInfo = 25,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`"]
    PipelineColorBlendStateCreateInfo = 26,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`"]
    PipelineDynamicStateCreateInfo = 27,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`"]
    GraphicsPipelineCreateInfo = 28,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`"]
    ComputePipelineCreateInfo = 29,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`"]
    PipelineLayoutCreateInfo = 30,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`"]
    SamplerCreateInfo = 31,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`"]
    DescriptorSetLayoutCreateInfo = 32,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`"]
    DescriptorPoolCreateInfo = 33,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`"]
    DescriptorSetAllocateInfo = 34,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`"]
    WriteDescriptorSet = 35,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`"]
    CopyDescriptorSet = 36,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`"]
    FramebufferCreateInfo = 37,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`"]
    RenderPassCreateInfo = 38,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`"]
    CommandPoolCreateInfo = 39,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`"]
    CommandBufferAllocateInfo = 40,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`"]
    CommandBufferInheritanceInfo = 41,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`"]
    CommandBufferBeginInfo = 42,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`"]
    RenderPassBeginInfo = 43,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`"]
    BufferMemoryBarrier = 44,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`"]
    ImageMemoryBarrier = 45,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_BARRIER`"]
    MemoryBarrier = 46,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO`"]
    LoaderInstanceCreateInfo = 47,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO`"]
    LoaderDeviceCreateInfo = 48,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`"]
    PhysicalDeviceSubgroupProperties = 1000094000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`"]
    BindBufferMemoryInfo = 1000157000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`"]
    BindImageMemoryInfo = 1000157001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`"]
    PhysicalDevice16bitStorageFeatures = 1000083000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`"]
    MemoryDedicatedRequirements = 1000127000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`"]
    MemoryDedicatedAllocateInfo = 1000127001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`"]
    MemoryAllocateFlagsInfo = 1000060000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`"]
    DeviceGroupRenderPassBeginInfo = 1000060003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`"]
    DeviceGroupCommandBufferBeginInfo = 1000060004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`"]
    DeviceGroupSubmitInfo = 1000060005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`"]
    DeviceGroupBindSparseInfo = 1000060006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`"]
    BindBufferMemoryDeviceGroupInfo = 1000060013,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`"]
    BindImageMemoryDeviceGroupInfo = 1000060014,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`"]
    PhysicalDeviceGroupProperties = 1000070000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`"]
    DeviceGroupDeviceCreateInfo = 1000070001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`"]
    BufferMemoryRequirementsInfo2 = 1000146000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`"]
    ImageMemoryRequirementsInfo2 = 1000146001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`"]
    ImageSparseMemoryRequirementsInfo2 = 1000146002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`"]
    MemoryRequirements2 = 1000146003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`"]
    SparseImageMemoryRequirements2 = 1000146004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`"]
    PhysicalDeviceFeatures2 = 1000059000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`"]
    PhysicalDeviceProperties2 = 1000059001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`"]
    FormatProperties2 = 1000059002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`"]
    ImageFormatProperties2 = 1000059003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`"]
    PhysicalDeviceImageFormatInfo2 = 1000059004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`"]
    QueueFamilyProperties2 = 1000059005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`"]
    PhysicalDeviceMemoryProperties2 = 1000059006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`"]
    SparseImageFormatProperties2 = 1000059007,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`"]
    PhysicalDeviceSparseImageFormatInfo2 = 1000059008,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`"]
    PhysicalDevicePointClippingProperties = 1000117000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`"]
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`"]
    ImageViewUsageCreateInfo = 1000117002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`"]
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`"]
    RenderPassMultiviewCreateInfo = 1000053000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`"]
    PhysicalDeviceMultiviewFeatures = 1000053001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`"]
    PhysicalDeviceMultiviewProperties = 1000053002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`"]
    PhysicalDeviceVariablePointersFeatures = 1000120000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`"]
    ProtectedSubmitInfo = 1000145000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`"]
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`"]
    PhysicalDeviceProtectedMemoryProperties = 1000145002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`"]
    DeviceQueueInfo2 = 1000145003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`"]
    SamplerYcbcrConversionCreateInfo = 1000156000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`"]
    SamplerYcbcrConversionInfo = 1000156001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`"]
    BindImagePlaneMemoryInfo = 1000156002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`"]
    ImagePlaneMemoryRequirementsInfo = 1000156003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`"]
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`"]
    SamplerYcbcrConversionImageFormatProperties = 1000156005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`"]
    DescriptorUpdateTemplateCreateInfo = 1000085000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`"]
    PhysicalDeviceExternalImageFormatInfo = 1000071000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`"]
    ExternalImageFormatProperties = 1000071001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`"]
    PhysicalDeviceExternalBufferInfo = 1000071002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`"]
    ExternalBufferProperties = 1000071003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`"]
    PhysicalDeviceIdProperties = 1000071004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`"]
    ExternalMemoryBufferCreateInfo = 1000072000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`"]
    ExternalMemoryImageCreateInfo = 1000072001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`"]
    ExportMemoryAllocateInfo = 1000072002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`"]
    PhysicalDeviceExternalFenceInfo = 1000112000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`"]
    ExternalFenceProperties = 1000112001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`"]
    ExportFenceCreateInfo = 1000113000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`"]
    ExportSemaphoreCreateInfo = 1000077000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`"]
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`"]
    ExternalSemaphoreProperties = 1000076001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`"]
    PhysicalDeviceMaintenance3Properties = 1000168000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`"]
    DescriptorSetLayoutSupport = 1000168001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`"]
    PhysicalDeviceShaderDrawParametersFeatures = 1000063000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`"]
    PhysicalDeviceVulkan11Features = 49,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`"]
    PhysicalDeviceVulkan11Properties = 50,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`"]
    PhysicalDeviceVulkan12Features = 51,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`"]
    PhysicalDeviceVulkan12Properties = 52,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`"]
    ImageFormatListCreateInfo = 1000147000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`"]
    AttachmentDescription2 = 1000109000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`"]
    AttachmentReference2 = 1000109001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`"]
    SubpassDescription2 = 1000109002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`"]
    SubpassDependency2 = 1000109003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`"]
    RenderPassCreateInfo2 = 1000109004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`"]
    SubpassBeginInfo = 1000109005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`"]
    SubpassEndInfo = 1000109006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`"]
    PhysicalDevice8bitStorageFeatures = 1000177000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`"]
    PhysicalDeviceDriverProperties = 1000196000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`"]
    PhysicalDeviceShaderAtomicInt64Features = 1000180000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`"]
    PhysicalDeviceShaderFloat16Int8Features = 1000082000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`"]
    PhysicalDeviceFloatControlsProperties = 1000197000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`"]
    DescriptorSetLayoutBindingFlagsCreateInfo = 1000161000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`"]
    PhysicalDeviceDescriptorIndexingFeatures = 1000161001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`"]
    PhysicalDeviceDescriptorIndexingProperties = 1000161002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`"]
    DescriptorSetVariableDescriptorCountAllocateInfo = 1000161003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`"]
    DescriptorSetVariableDescriptorCountLayoutSupport = 1000161004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`"]
    PhysicalDeviceDepthStencilResolveProperties = 1000199000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`"]
    SubpassDescriptionDepthStencilResolve = 1000199001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`"]
    PhysicalDeviceScalarBlockLayoutFeatures = 1000221000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`"]
    ImageStencilUsageCreateInfo = 1000246000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`"]
    PhysicalDeviceSamplerFilterMinmaxProperties = 1000130000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`"]
    SamplerReductionModeCreateInfo = 1000130001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`"]
    PhysicalDeviceVulkanMemoryModelFeatures = 1000211000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`"]
    PhysicalDeviceImagelessFramebufferFeatures = 1000108000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`"]
    FramebufferAttachmentsCreateInfo = 1000108001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`"]
    FramebufferAttachmentImageInfo = 1000108002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`"]
    RenderPassAttachmentBeginInfo = 1000108003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`"]
    PhysicalDeviceUniformBufferStandardLayoutFeatures = 1000253000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`"]
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures = 1000175000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`"]
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures = 1000241000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`"]
    AttachmentReferenceStencilLayout = 1000241001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`"]
    AttachmentDescriptionStencilLayout = 1000241002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`"]
    PhysicalDeviceHostQueryResetFeatures = 1000261000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`"]
    PhysicalDeviceTimelineSemaphoreFeatures = 1000207000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`"]
    PhysicalDeviceTimelineSemaphoreProperties = 1000207001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`"]
    SemaphoreTypeCreateInfo = 1000207002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`"]
    TimelineSemaphoreSubmitInfo = 1000207003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`"]
    SemaphoreWaitInfo = 1000207004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`"]
    SemaphoreSignalInfo = 1000207005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`"]
    PhysicalDeviceBufferDeviceAddressFeatures = 1000257000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`"]
    BufferDeviceAddressInfo = 1000244001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`"]
    BufferOpaqueCaptureAddressCreateInfo = 1000257002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`"]
    MemoryOpaqueCaptureAddressAllocateInfo = 1000257003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`"]
    DeviceMemoryOpaqueCaptureAddressInfo = 1000257004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`"]
    PhysicalDeviceVulkan13Features = 53,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`"]
    PhysicalDeviceVulkan13Properties = 54,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`"]
    PipelineCreationFeedbackCreateInfo = 1000192000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`"]
    PhysicalDeviceShaderTerminateInvocationFeatures = 1000215000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`"]
    PhysicalDeviceToolProperties = 1000245000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`"]
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures = 1000276000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`"]
    PhysicalDevicePrivateDataFeatures = 1000295000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`"]
    DevicePrivateDataCreateInfo = 1000295001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`"]
    PrivateDataSlotCreateInfo = 1000295002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`"]
    PhysicalDevicePipelineCreationCacheControlFeatures = 1000297000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`"]
    MemoryBarrier2 = 1000314000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`"]
    BufferMemoryBarrier2 = 1000314001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`"]
    ImageMemoryBarrier2 = 1000314002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`"]
    DependencyInfo = 1000314003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`"]
    SubmitInfo2 = 1000314004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`"]
    SemaphoreSubmitInfo = 1000314005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`"]
    CommandBufferSubmitInfo = 1000314006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`"]
    PhysicalDeviceSynchronization2Features = 1000314007,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`"]
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures = 1000325000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`"]
    PhysicalDeviceImageRobustnessFeatures = 1000335000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`"]
    CopyBufferInfo2 = 1000337000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`"]
    CopyImageInfo2 = 1000337001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`"]
    CopyBufferToImageInfo2 = 1000337002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`"]
    CopyImageToBufferInfo2 = 1000337003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`"]
    BlitImageInfo2 = 1000337004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`"]
    ResolveImageInfo2 = 1000337005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_COPY_2`"]
    BufferCopy2 = 1000337006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_COPY_2`"]
    ImageCopy2 = 1000337007,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`"]
    ImageBlit2 = 1000337008,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`"]
    BufferImageCopy2 = 1000337009,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`"]
    ImageResolve2 = 1000337010,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`"]
    PhysicalDeviceSubgroupSizeControlProperties = 1000225000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`"]
    PipelineShaderStageRequiredSubgroupSizeCreateInfo = 1000225001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`"]
    PhysicalDeviceSubgroupSizeControlFeatures = 1000225002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`"]
    PhysicalDeviceInlineUniformBlockFeatures = 1000138000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`"]
    PhysicalDeviceInlineUniformBlockProperties = 1000138001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`"]
    WriteDescriptorSetInlineUniformBlock = 1000138002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`"]
    DescriptorPoolInlineUniformBlockCreateInfo = 1000138003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`"]
    PhysicalDeviceTextureCompressionAstcHdrFeatures = 1000066000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDERING_INFO`"]
    RenderingInfo = 1000044000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`"]
    RenderingAttachmentInfo = 1000044001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`"]
    PipelineRenderingCreateInfo = 1000044002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`"]
    PhysicalDeviceDynamicRenderingFeatures = 1000044003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`"]
    CommandBufferInheritanceRenderingInfo = 1000044004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`"]
    PhysicalDeviceShaderIntegerDotProductFeatures = 1000280000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`"]
    PhysicalDeviceShaderIntegerDotProductProperties = 1000280001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`"]
    PhysicalDeviceTexelBufferAlignmentProperties = 1000281001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`"]
    FormatProperties3 = 1000360000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`"]
    PhysicalDeviceMaintenance4Features = 1000413000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`"]
    PhysicalDeviceMaintenance4Properties = 1000413001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`"]
    DeviceBufferMemoryRequirements = 1000413002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`"]
    DeviceImageMemoryRequirements = 1000413003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`"]
    DebugUtilsObjectNameInfoEXT = 1000128000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`"]
    DebugUtilsObjectTagInfoEXT = 1000128001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`"]
    DebugUtilsLabelEXT = 1000128002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`"]
    DebugUtilsMessengerCallbackDataEXT = 1000128003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`"]
    DebugUtilsMessengerCreateInfoEXT = 1000128004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`"]
    CalibratedTimestampInfoEXT = 1000184000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`"]
    ValidationFeaturesEXT = 1000247000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR`"]
    MemoryMapInfoKHR = 1000271000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO_KHR`"]
    MemoryUnmapInfoKHR = 1000271001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT`"]
    PhysicalDeviceDescriptorBufferPropertiesEXT = 1000316000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT`"]
    PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT = 1000316001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT`"]
    PhysicalDeviceDescriptorBufferFeaturesEXT = 1000316002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT`"]
    DescriptorAddressInfoEXT = 1000316003,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT`"]
    DescriptorGetInfoEXT = 1000316004,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`"]
    BufferCaptureDescriptorDataInfoEXT = 1000316005,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`"]
    ImageCaptureDescriptorDataInfoEXT = 1000316006,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`"]
    ImageViewCaptureDescriptorDataInfoEXT = 1000316007,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`"]
    SamplerCaptureDescriptorDataInfoEXT = 1000316008,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT`"]
    OpaqueCaptureDescriptorDataCreateInfoEXT = 1000316010,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT`"]
    DescriptorBufferBindingInfoEXT = 1000316011,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT`"]
    DescriptorBufferBindingPushDescriptorBufferHandleEXT = 1000316012,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`"]
    AccelerationStructureCaptureDescriptorDataInfoEXT = 1000316009,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT`"]
    PhysicalDeviceMeshShaderFeaturesEXT = 1000328000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT`"]
    PhysicalDeviceMeshShaderPropertiesEXT = 1000328001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT`"]
    PhysicalDeviceShaderObjectFeaturesEXT = 1000482000,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT`"]
    PhysicalDeviceShaderObjectPropertiesEXT = 1000482001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_SHADER_CREATE_INFO_EXT`"]
    ShaderCreateInfoEXT = 1000482002,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`"]
    VertexInputBindingDescription2EXT = 1000352001,
    #[doc = "**Translated from**: `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`"]
    VertexInputAttributeDescription2EXT = 1000352002,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Specify validation features to enable"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_validation_features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkValidationFeatureEnableEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html)"]
pub enum ValidationFeatureEnableEXT {
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`"]
    GpuAssistedEXT = 0,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT`"]
    GpuAssistedReserveBindingSlotEXT = 1,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT`"]
    BestPracticesEXT = 2,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT`"]
    DebugPrintfEXT = 3,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT`"]
    SynchronizationValidationEXT = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Specify validation features to disable"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_validation_features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkValidationFeatureDisableEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html)"]
pub enum ValidationFeatureDisableEXT {
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_ALL_EXT`"]
    AllEXT = 0,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT`"]
    ShadersEXT = 1,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT`"]
    ThreadSafetyEXT = 2,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT`"]
    ApiParametersEXT = 3,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT`"]
    ObjectLifetimesEXT = 4,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT`"]
    CoreChecksEXT = 5,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT`"]
    UniqueHandlesEXT = 6,
    #[doc = "**Translated from**: `VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT`"]
    ShaderValidationCacheEXT = 7,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Supported physical device types"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html)"]
pub enum PhysicalDeviceType {
    #[doc = "**Translated from**: `VK_PHYSICAL_DEVICE_TYPE_OTHER`"]
    Other = 0,
    #[doc = "**Translated from**: `VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU`"]
    IntegratedGpu = 1,
    #[doc = "**Translated from**: `VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU`"]
    DiscreteGpu = 2,
    #[doc = "**Translated from**: `VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU`"]
    VirtualGpu = 3,
    #[doc = "**Translated from**: `VK_PHYSICAL_DEVICE_TYPE_CPU`"]
    Cpu = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Enumerant specifying a command buffer level"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html)"]
pub enum CommandBufferLevel {
    #[doc = "**Translated from**: `VK_COMMAND_BUFFER_LEVEL_PRIMARY`"]
    Primary = 0,
    #[doc = "**Translated from**: `VK_COMMAND_BUFFER_LEVEL_SECONDARY`"]
    Secondary = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Specifies the type of a semaphore object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreType.html)"]
pub enum SemaphoreType {
    #[doc = "**Translated from**: `VK_SEMAPHORE_TYPE_BINARY`"]
    Binary = 0,
    #[doc = "**Translated from**: `VK_SEMAPHORE_TYPE_TIMELINE`"]
    Timeline = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Supported time domains"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkTimeDomainEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html)"]
pub enum TimeDomainEXT {
    #[doc = "**Translated from**: `VK_TIME_DOMAIN_DEVICE_EXT`"]
    DeviceEXT = 0,
    #[doc = "**Translated from**: `VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT`"]
    ClockMonotonicEXT = 1,
    #[doc = "**Translated from**: `VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT`"]
    ClockMonotonicRawEXT = 2,
    #[doc = "**Translated from**: `VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT`"]
    QueryPerformanceCounterEXT = 3,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Specify how contents of an attachment are treated at the beginning of a subpass"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html)"]
pub enum AttachmentLoadOp {
    #[doc = "**Translated from**: `VK_ATTACHMENT_LOAD_OP_LOAD`"]
    Load = 0,
    #[doc = "**Translated from**: `VK_ATTACHMENT_LOAD_OP_CLEAR`"]
    Clear = 1,
    #[doc = "**Translated from**: `VK_ATTACHMENT_LOAD_OP_DONT_CARE`"]
    DontCare = 2,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Specify how contents of an attachment are treated at the end of a subpass"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html)"]
pub enum AttachmentStoreOp {
    #[doc = "**Translated from**: `VK_ATTACHMENT_STORE_OP_STORE`"]
    Store = 0,
    #[doc = "**Translated from**: `VK_ATTACHMENT_STORE_OP_DONT_CARE`"]
    DontCare = 1,
    #[doc = "**Translated from**: `VK_ATTACHMENT_STORE_OP_NONE`"]
    None = 1000301000,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Indicate a shader code type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkShaderCodeTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCodeTypeEXT.html)"]
pub enum ShaderCodeTypeEXT {
    #[doc = "**Translated from**: `VK_SHADER_CODE_TYPE_BINARY_EXT`"]
    BinaryEXT = 0,
    #[doc = "**Translated from**: `VK_SHADER_CODE_TYPE_SPIRV_EXT`"]
    SpirvEXT = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Pipelines"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the bind point of a pipeline object to a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPipelineBindPoint`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html)"]
pub enum PipelineBindPoint {
    #[doc = "**Translated from**: `VK_PIPELINE_BIND_POINT_GRAPHICS`"]
    Graphics = 0,
    #[doc = "**Translated from**: `VK_PIPELINE_BIND_POINT_COMPUTE`"]
    Compute = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Allocation scope"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSystemAllocationScope`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html)"]
pub enum SystemAllocationScope {
    #[doc = "**Translated from**: `VK_SYSTEM_ALLOCATION_SCOPE_COMMAND`"]
    Command = 0,
    #[doc = "**Translated from**: `VK_SYSTEM_ALLOCATION_SCOPE_OBJECT`"]
    Object = 1,
    #[doc = "**Translated from**: `VK_SYSTEM_ALLOCATION_SCOPE_CACHE`"]
    Cache = 2,
    #[doc = "**Translated from**: `VK_SYSTEM_ALLOCATION_SCOPE_DEVICE`"]
    Device = 3,
    #[doc = "**Translated from**: `VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE`"]
    Instance = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Allocation type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkInternalAllocationType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html)"]
pub enum InternalAllocationType {
    #[doc = "**Translated from**: `VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE`"]
    Executable = 0,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Specifies the type of an image object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageType.html)"]
pub enum ImageType {
    #[doc = "**Translated from**: `VK_IMAGE_TYPE_1D`"]
    Type1d = 0,
    #[doc = "**Translated from**: `VK_IMAGE_TYPE_2D`"]
    Type2d = 1,
    #[doc = "**Translated from**: `VK_IMAGE_TYPE_3D`"]
    Type3d = 2,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Specifies the tiling arrangement of data in an image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageTiling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html)"]
pub enum ImageTiling {
    #[doc = "**Translated from**: `VK_IMAGE_TILING_OPTIMAL`"]
    Optimal = 0,
    #[doc = "**Translated from**: `VK_IMAGE_TILING_LINEAR`"]
    Linear = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Layout of image and image subresources"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html)"]
pub enum ImageLayout {
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_UNDEFINED`"]
    Undefined = 0,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_GENERAL`"]
    General = 1,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`"]
    ColorAttachmentOptimal = 2,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`"]
    DepthStencilAttachmentOptimal = 3,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`"]
    DepthStencilReadOnlyOptimal = 4,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`"]
    ShaderReadOnlyOptimal = 5,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`"]
    TransferSrcOptimal = 6,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`"]
    TransferDstOptimal = 7,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_PREINITIALIZED`"]
    Preinitialized = 8,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`"]
    DepthReadOnlyStencilAttachmentOptimal = 1000117000,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`"]
    DepthAttachmentStencilReadOnlyOptimal = 1000117001,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`"]
    DepthAttachmentOptimal = 1000241000,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`"]
    DepthReadOnlyOptimal = 1000241001,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`"]
    StencilAttachmentOptimal = 1000241002,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`"]
    StencilReadOnlyOptimal = 1000241003,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL`"]
    ReadOnlyOptimal = 1000314000,
    #[doc = "**Translated from**: `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`"]
    AttachmentOptimal = 1000314001,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Image view types"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html)"]
pub enum ImageViewType {
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_1D`"]
    Type1d = 0,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_2D`"]
    Type2d = 1,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_3D`"]
    Type3d = 2,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_CUBE`"]
    TypeCube = 3,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_1D_ARRAY`"]
    Type1dArray = 4,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_2D_ARRAY`"]
    Type2dArray = 5,
    #[doc = "**Translated from**: `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`"]
    TypeCubeArray = 6,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Specify how a component is swizzled"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html)"]
pub enum ComponentSwizzle {
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_IDENTITY`"]
    Identity = 0,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_ZERO`"]
    Zero = 1,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_ONE`"]
    One = 2,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_R`"]
    R = 3,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_G`"]
    G = 4,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_B`"]
    B = 5,
    #[doc = "**Translated from**: `VK_COMPONENT_SWIZZLE_A`"]
    A = 6,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Buffer and image sharing modes"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSharingMode`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html)"]
pub enum SharingMode {
    #[doc = "**Translated from**: `VK_SHARING_MODE_EXCLUSIVE`"]
    Exclusive = 0,
    #[doc = "**Translated from**: `VK_SHARING_MODE_CONCURRENT`"]
    Concurrent = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Specify filters used for texture lookups"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilter.html)"]
pub enum Filter {
    #[doc = "**Translated from**: `VK_FILTER_NEAREST`"]
    Nearest = 0,
    #[doc = "**Translated from**: `VK_FILTER_LINEAR`"]
    Linear = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Specify mipmap mode used for texture lookups"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSamplerMipmapMode`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html)"]
pub enum SamplerMipmapMode {
    #[doc = "**Translated from**: `VK_SAMPLER_MIPMAP_MODE_NEAREST`"]
    Nearest = 0,
    #[doc = "**Translated from**: `VK_SAMPLER_MIPMAP_MODE_LINEAR`"]
    Linear = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Specify behavior of sampling with texture coordinates outside an image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html)"]
pub enum SamplerAddressMode {
    #[doc = "**Translated from**: `VK_SAMPLER_ADDRESS_MODE_REPEAT`"]
    Repeat = 0,
    #[doc = "**Translated from**: `VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT`"]
    MirroredRepeat = 1,
    #[doc = "**Translated from**: `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`"]
    ClampToEdge = 2,
    #[doc = "**Translated from**: `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER`"]
    ClampToBorder = 3,
    #[doc = "**Translated from**: `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE`"]
    MirrorClampToEdge = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Comparison operator for depth"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html)"]
pub enum CompareOp {
    #[doc = "**Translated from**: `VK_COMPARE_OP_NEVER`"]
    Never = 0,
    #[doc = "**Translated from**: `VK_COMPARE_OP_LESS`"]
    Less = 1,
    #[doc = "**Translated from**: `VK_COMPARE_OP_EQUAL`"]
    Equal = 2,
    #[doc = "**Translated from**: `VK_COMPARE_OP_LESS_OR_EQUAL`"]
    LessOrEqual = 3,
    #[doc = "**Translated from**: `VK_COMPARE_OP_GREATER`"]
    Greater = 4,
    #[doc = "**Translated from**: `VK_COMPARE_OP_NOT_EQUAL`"]
    NotEqual = 5,
    #[doc = "**Translated from**: `VK_COMPARE_OP_GREATER_OR_EQUAL`"]
    GreaterOrEqual = 6,
    #[doc = "**Translated from**: `VK_COMPARE_OP_ALWAYS`"]
    Always = 7,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Specify border color used for texture lookups"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html)"]
pub enum BorderColor {
    #[doc = "**Translated from**: `VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK`"]
    FloatTransparentBlack = 0,
    #[doc = "**Translated from**: `VK_BORDER_COLOR_INT_TRANSPARENT_BLACK`"]
    IntTransparentBlack = 1,
    #[doc = "**Translated from**: `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`"]
    FloatOpaqueBlack = 2,
    #[doc = "**Translated from**: `VK_BORDER_COLOR_INT_OPAQUE_BLACK`"]
    IntOpaqueBlack = 3,
    #[doc = "**Translated from**: `VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE`"]
    FloatOpaqueWhite = 4,
    #[doc = "**Translated from**: `VK_BORDER_COLOR_INT_OPAQUE_WHITE`"]
    IntOpaqueWhite = 5,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Specifies the type of a descriptor in a descriptor set"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html)"]
pub enum DescriptorType {
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_SAMPLER`"]
    Sampler = 0,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`"]
    CombinedImageSampler = 1,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`"]
    SampledImage = 2,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`"]
    StorageImage = 3,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`"]
    UniformTexelBuffer = 4,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`"]
    StorageTexelBuffer = 5,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`"]
    UniformBuffer = 6,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`"]
    StorageBuffer = 7,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`"]
    UniformBufferDynamic = 8,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`"]
    StorageBufferDynamic = 9,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`"]
    InputAttachment = 10,
    #[doc = "**Translated from**: `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`"]
    InlineUniformBlock = 1000138000,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the type of queries managed by a query pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryType.html)"]
pub enum QueryType {
    #[doc = "**Translated from**: `VK_QUERY_TYPE_OCCLUSION`"]
    Occlusion = 0,
    #[doc = "**Translated from**: `VK_QUERY_TYPE_PIPELINE_STATISTICS`"]
    PipelineStatistics = 1,
    #[doc = "**Translated from**: `VK_QUERY_TYPE_TIMESTAMP`"]
    Timestamp = 2,
    #[doc = "**Translated from**: `VK_QUERY_TYPE_MESH_PRIMITIVES_GENERATED_EXT`"]
    MeshPrimitivesGeneratedEXT = 1000328000,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Rasterization"]
#[doc = "<br>"]
#[doc = "**Description**: Interpret polygon front-facing orientation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html)"]
pub enum FrontFace {
    #[doc = "**Translated from**: `VK_FRONT_FACE_COUNTER_CLOCKWISE`"]
    CounterClockwise = 0,
    #[doc = "**Translated from**: `VK_FRONT_FACE_CLOCKWISE`"]
    Clockwise = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Framebuffer blending factors"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html)"]
pub enum BlendFactor {
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ZERO`"]
    Zero = 0,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE`"]
    One = 1,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_SRC_COLOR`"]
    SrcColor = 2,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR`"]
    OneMinusSrcColor = 3,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_DST_COLOR`"]
    DstColor = 4,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR`"]
    OneMinusDstColor = 5,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_SRC_ALPHA`"]
    SrcAlpha = 6,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA`"]
    OneMinusSrcAlpha = 7,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_DST_ALPHA`"]
    DstAlpha = 8,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA`"]
    OneMinusDstAlpha = 9,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_CONSTANT_COLOR`"]
    ConstantColor = 10,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR`"]
    OneMinusConstantColor = 11,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_CONSTANT_ALPHA`"]
    ConstantAlpha = 12,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA`"]
    OneMinusConstantAlpha = 13,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_SRC_ALPHA_SATURATE`"]
    SrcAlphaSaturate = 14,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_SRC1_COLOR`"]
    Src1Color = 15,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`"]
    OneMinusSrc1Color = 16,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_SRC1_ALPHA`"]
    Src1Alpha = 17,
    #[doc = "**Translated from**: `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`"]
    OneMinusSrc1Alpha = 18,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Framebuffer blending operations"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html)"]
pub enum BlendOp {
    #[doc = "**Translated from**: `VK_BLEND_OP_ADD`"]
    Add = 0,
    #[doc = "**Translated from**: `VK_BLEND_OP_SUBTRACT`"]
    Subtract = 1,
    #[doc = "**Translated from**: `VK_BLEND_OP_REVERSE_SUBTRACT`"]
    ReverseSubtract = 2,
    #[doc = "**Translated from**: `VK_BLEND_OP_MIN`"]
    Min = 3,
    #[doc = "**Translated from**: `VK_BLEND_OP_MAX`"]
    Max = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Formats"]
#[doc = "<br>"]
#[doc = "**Description**: Available image formats"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormat.html)"]
pub enum Format {
    #[doc = "**Translated from**: `VK_FORMAT_UNDEFINED`"]
    Undefined = 0,
    #[doc = "**Translated from**: `VK_FORMAT_R4G4_UNORM_PACK8`"]
    R4g4UnormPack8 = 1,
    #[doc = "**Translated from**: `VK_FORMAT_R4G4B4A4_UNORM_PACK16`"]
    R4g4b4a4UnormPack16 = 2,
    #[doc = "**Translated from**: `VK_FORMAT_B4G4R4A4_UNORM_PACK16`"]
    B4g4r4a4UnormPack16 = 3,
    #[doc = "**Translated from**: `VK_FORMAT_R5G6B5_UNORM_PACK16`"]
    R5g6b5UnormPack16 = 4,
    #[doc = "**Translated from**: `VK_FORMAT_B5G6R5_UNORM_PACK16`"]
    B5g6r5UnormPack16 = 5,
    #[doc = "**Translated from**: `VK_FORMAT_R5G5B5A1_UNORM_PACK16`"]
    R5g5b5a1UnormPack16 = 6,
    #[doc = "**Translated from**: `VK_FORMAT_B5G5R5A1_UNORM_PACK16`"]
    B5g5r5a1UnormPack16 = 7,
    #[doc = "**Translated from**: `VK_FORMAT_A1R5G5B5_UNORM_PACK16`"]
    A1r5g5b5UnormPack16 = 8,
    #[doc = "**Translated from**: `VK_FORMAT_R8_UNORM`"]
    R8Unorm = 9,
    #[doc = "**Translated from**: `VK_FORMAT_R8_SNORM`"]
    R8Snorm = 10,
    #[doc = "**Translated from**: `VK_FORMAT_R8_USCALED`"]
    R8Uscaled = 11,
    #[doc = "**Translated from**: `VK_FORMAT_R8_SSCALED`"]
    R8Sscaled = 12,
    #[doc = "**Translated from**: `VK_FORMAT_R8_UINT`"]
    R8Uint = 13,
    #[doc = "**Translated from**: `VK_FORMAT_R8_SINT`"]
    R8Sint = 14,
    #[doc = "**Translated from**: `VK_FORMAT_R8_SRGB`"]
    R8Srgb = 15,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_UNORM`"]
    R8g8Unorm = 16,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_SNORM`"]
    R8g8Snorm = 17,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_USCALED`"]
    R8g8Uscaled = 18,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_SSCALED`"]
    R8g8Sscaled = 19,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_UINT`"]
    R8g8Uint = 20,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_SINT`"]
    R8g8Sint = 21,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8_SRGB`"]
    R8g8Srgb = 22,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_UNORM`"]
    R8g8b8Unorm = 23,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_SNORM`"]
    R8g8b8Snorm = 24,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_USCALED`"]
    R8g8b8Uscaled = 25,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_SSCALED`"]
    R8g8b8Sscaled = 26,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_UINT`"]
    R8g8b8Uint = 27,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_SINT`"]
    R8g8b8Sint = 28,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8_SRGB`"]
    R8g8b8Srgb = 29,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_UNORM`"]
    B8g8r8Unorm = 30,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_SNORM`"]
    B8g8r8Snorm = 31,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_USCALED`"]
    B8g8r8Uscaled = 32,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_SSCALED`"]
    B8g8r8Sscaled = 33,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_UINT`"]
    B8g8r8Uint = 34,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_SINT`"]
    B8g8r8Sint = 35,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8_SRGB`"]
    B8g8r8Srgb = 36,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_UNORM`"]
    R8g8b8a8Unorm = 37,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_SNORM`"]
    R8g8b8a8Snorm = 38,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_USCALED`"]
    R8g8b8a8Uscaled = 39,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_SSCALED`"]
    R8g8b8a8Sscaled = 40,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_UINT`"]
    R8g8b8a8Uint = 41,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_SINT`"]
    R8g8b8a8Sint = 42,
    #[doc = "**Translated from**: `VK_FORMAT_R8G8B8A8_SRGB`"]
    R8g8b8a8Srgb = 43,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_UNORM`"]
    B8g8r8a8Unorm = 44,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_SNORM`"]
    B8g8r8a8Snorm = 45,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_USCALED`"]
    B8g8r8a8Uscaled = 46,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_SSCALED`"]
    B8g8r8a8Sscaled = 47,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_UINT`"]
    B8g8r8a8Uint = 48,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_SINT`"]
    B8g8r8a8Sint = 49,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8A8_SRGB`"]
    B8g8r8a8Srgb = 50,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_UNORM_PACK32`"]
    A8b8g8r8UnormPack32 = 51,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_SNORM_PACK32`"]
    A8b8g8r8SnormPack32 = 52,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_USCALED_PACK32`"]
    A8b8g8r8UscaledPack32 = 53,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_SSCALED_PACK32`"]
    A8b8g8r8SscaledPack32 = 54,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_UINT_PACK32`"]
    A8b8g8r8UintPack32 = 55,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_SINT_PACK32`"]
    A8b8g8r8SintPack32 = 56,
    #[doc = "**Translated from**: `VK_FORMAT_A8B8G8R8_SRGB_PACK32`"]
    A8b8g8r8SrgbPack32 = 57,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_UNORM_PACK32`"]
    A2r10g10b10UnormPack32 = 58,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_SNORM_PACK32`"]
    A2r10g10b10SnormPack32 = 59,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_USCALED_PACK32`"]
    A2r10g10b10UscaledPack32 = 60,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_SSCALED_PACK32`"]
    A2r10g10b10SscaledPack32 = 61,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_UINT_PACK32`"]
    A2r10g10b10UintPack32 = 62,
    #[doc = "**Translated from**: `VK_FORMAT_A2R10G10B10_SINT_PACK32`"]
    A2r10g10b10SintPack32 = 63,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_UNORM_PACK32`"]
    A2b10g10r10UnormPack32 = 64,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_SNORM_PACK32`"]
    A2b10g10r10SnormPack32 = 65,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_USCALED_PACK32`"]
    A2b10g10r10UscaledPack32 = 66,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_SSCALED_PACK32`"]
    A2b10g10r10SscaledPack32 = 67,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_UINT_PACK32`"]
    A2b10g10r10UintPack32 = 68,
    #[doc = "**Translated from**: `VK_FORMAT_A2B10G10R10_SINT_PACK32`"]
    A2b10g10r10SintPack32 = 69,
    #[doc = "**Translated from**: `VK_FORMAT_R16_UNORM`"]
    R16Unorm = 70,
    #[doc = "**Translated from**: `VK_FORMAT_R16_SNORM`"]
    R16Snorm = 71,
    #[doc = "**Translated from**: `VK_FORMAT_R16_USCALED`"]
    R16Uscaled = 72,
    #[doc = "**Translated from**: `VK_FORMAT_R16_SSCALED`"]
    R16Sscaled = 73,
    #[doc = "**Translated from**: `VK_FORMAT_R16_UINT`"]
    R16Uint = 74,
    #[doc = "**Translated from**: `VK_FORMAT_R16_SINT`"]
    R16Sint = 75,
    #[doc = "**Translated from**: `VK_FORMAT_R16_SFLOAT`"]
    R16Sfloat = 76,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_UNORM`"]
    R16g16Unorm = 77,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_SNORM`"]
    R16g16Snorm = 78,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_USCALED`"]
    R16g16Uscaled = 79,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_SSCALED`"]
    R16g16Sscaled = 80,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_UINT`"]
    R16g16Uint = 81,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_SINT`"]
    R16g16Sint = 82,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16_SFLOAT`"]
    R16g16Sfloat = 83,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_UNORM`"]
    R16g16b16Unorm = 84,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_SNORM`"]
    R16g16b16Snorm = 85,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_USCALED`"]
    R16g16b16Uscaled = 86,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_SSCALED`"]
    R16g16b16Sscaled = 87,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_UINT`"]
    R16g16b16Uint = 88,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_SINT`"]
    R16g16b16Sint = 89,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16_SFLOAT`"]
    R16g16b16Sfloat = 90,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_UNORM`"]
    R16g16b16a16Unorm = 91,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_SNORM`"]
    R16g16b16a16Snorm = 92,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_USCALED`"]
    R16g16b16a16Uscaled = 93,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_SSCALED`"]
    R16g16b16a16Sscaled = 94,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_UINT`"]
    R16g16b16a16Uint = 95,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_SINT`"]
    R16g16b16a16Sint = 96,
    #[doc = "**Translated from**: `VK_FORMAT_R16G16B16A16_SFLOAT`"]
    R16g16b16a16Sfloat = 97,
    #[doc = "**Translated from**: `VK_FORMAT_R32_UINT`"]
    R32Uint = 98,
    #[doc = "**Translated from**: `VK_FORMAT_R32_SINT`"]
    R32Sint = 99,
    #[doc = "**Translated from**: `VK_FORMAT_R32_SFLOAT`"]
    R32Sfloat = 100,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32_UINT`"]
    R32g32Uint = 101,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32_SINT`"]
    R32g32Sint = 102,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32_SFLOAT`"]
    R32g32Sfloat = 103,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32_UINT`"]
    R32g32b32Uint = 104,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32_SINT`"]
    R32g32b32Sint = 105,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32_SFLOAT`"]
    R32g32b32Sfloat = 106,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32A32_UINT`"]
    R32g32b32a32Uint = 107,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32A32_SINT`"]
    R32g32b32a32Sint = 108,
    #[doc = "**Translated from**: `VK_FORMAT_R32G32B32A32_SFLOAT`"]
    R32g32b32a32Sfloat = 109,
    #[doc = "**Translated from**: `VK_FORMAT_R64_UINT`"]
    R64Uint = 110,
    #[doc = "**Translated from**: `VK_FORMAT_R64_SINT`"]
    R64Sint = 111,
    #[doc = "**Translated from**: `VK_FORMAT_R64_SFLOAT`"]
    R64Sfloat = 112,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64_UINT`"]
    R64g64Uint = 113,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64_SINT`"]
    R64g64Sint = 114,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64_SFLOAT`"]
    R64g64Sfloat = 115,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64_UINT`"]
    R64g64b64Uint = 116,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64_SINT`"]
    R64g64b64Sint = 117,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64_SFLOAT`"]
    R64g64b64Sfloat = 118,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64A64_UINT`"]
    R64g64b64a64Uint = 119,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64A64_SINT`"]
    R64g64b64a64Sint = 120,
    #[doc = "**Translated from**: `VK_FORMAT_R64G64B64A64_SFLOAT`"]
    R64g64b64a64Sfloat = 121,
    #[doc = "**Translated from**: `VK_FORMAT_B10G11R11_UFLOAT_PACK32`"]
    B10g11r11UfloatPack32 = 122,
    #[doc = "**Translated from**: `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`"]
    E5b9g9r9UfloatPack32 = 123,
    #[doc = "**Translated from**: `VK_FORMAT_D16_UNORM`"]
    D16Unorm = 124,
    #[doc = "**Translated from**: `VK_FORMAT_X8_D24_UNORM_PACK32`"]
    X8D24UnormPack32 = 125,
    #[doc = "**Translated from**: `VK_FORMAT_D32_SFLOAT`"]
    D32Sfloat = 126,
    #[doc = "**Translated from**: `VK_FORMAT_S8_UINT`"]
    S8Uint = 127,
    #[doc = "**Translated from**: `VK_FORMAT_D16_UNORM_S8_UINT`"]
    D16UnormS8Uint = 128,
    #[doc = "**Translated from**: `VK_FORMAT_D24_UNORM_S8_UINT`"]
    D24UnormS8Uint = 129,
    #[doc = "**Translated from**: `VK_FORMAT_D32_SFLOAT_S8_UINT`"]
    D32SfloatS8Uint = 130,
    #[doc = "**Translated from**: `VK_FORMAT_BC1_RGB_UNORM_BLOCK`"]
    Bc1RgbUnormBlock = 131,
    #[doc = "**Translated from**: `VK_FORMAT_BC1_RGB_SRGB_BLOCK`"]
    Bc1RgbSrgbBlock = 132,
    #[doc = "**Translated from**: `VK_FORMAT_BC1_RGBA_UNORM_BLOCK`"]
    Bc1RgbaUnormBlock = 133,
    #[doc = "**Translated from**: `VK_FORMAT_BC1_RGBA_SRGB_BLOCK`"]
    Bc1RgbaSrgbBlock = 134,
    #[doc = "**Translated from**: `VK_FORMAT_BC2_UNORM_BLOCK`"]
    Bc2UnormBlock = 135,
    #[doc = "**Translated from**: `VK_FORMAT_BC2_SRGB_BLOCK`"]
    Bc2SrgbBlock = 136,
    #[doc = "**Translated from**: `VK_FORMAT_BC3_UNORM_BLOCK`"]
    Bc3UnormBlock = 137,
    #[doc = "**Translated from**: `VK_FORMAT_BC3_SRGB_BLOCK`"]
    Bc3SrgbBlock = 138,
    #[doc = "**Translated from**: `VK_FORMAT_BC4_UNORM_BLOCK`"]
    Bc4UnormBlock = 139,
    #[doc = "**Translated from**: `VK_FORMAT_BC4_SNORM_BLOCK`"]
    Bc4SnormBlock = 140,
    #[doc = "**Translated from**: `VK_FORMAT_BC5_UNORM_BLOCK`"]
    Bc5UnormBlock = 141,
    #[doc = "**Translated from**: `VK_FORMAT_BC5_SNORM_BLOCK`"]
    Bc5SnormBlock = 142,
    #[doc = "**Translated from**: `VK_FORMAT_BC6H_UFLOAT_BLOCK`"]
    Bc6hUfloatBlock = 143,
    #[doc = "**Translated from**: `VK_FORMAT_BC6H_SFLOAT_BLOCK`"]
    Bc6hSfloatBlock = 144,
    #[doc = "**Translated from**: `VK_FORMAT_BC7_UNORM_BLOCK`"]
    Bc7UnormBlock = 145,
    #[doc = "**Translated from**: `VK_FORMAT_BC7_SRGB_BLOCK`"]
    Bc7SrgbBlock = 146,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`"]
    Etc2R8g8b8UnormBlock = 147,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`"]
    Etc2R8g8b8SrgbBlock = 148,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`"]
    Etc2R8g8b8a1UnormBlock = 149,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`"]
    Etc2R8g8b8a1SrgbBlock = 150,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`"]
    Etc2R8g8b8a8UnormBlock = 151,
    #[doc = "**Translated from**: `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`"]
    Etc2R8g8b8a8SrgbBlock = 152,
    #[doc = "**Translated from**: `VK_FORMAT_EAC_R11_UNORM_BLOCK`"]
    EacR11UnormBlock = 153,
    #[doc = "**Translated from**: `VK_FORMAT_EAC_R11_SNORM_BLOCK`"]
    EacR11SnormBlock = 154,
    #[doc = "**Translated from**: `VK_FORMAT_EAC_R11G11_UNORM_BLOCK`"]
    EacR11g11UnormBlock = 155,
    #[doc = "**Translated from**: `VK_FORMAT_EAC_R11G11_SNORM_BLOCK`"]
    EacR11g11SnormBlock = 156,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_4x4_UNORM_BLOCK`"]
    Astc4x4UnormBlock = 157,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_4x4_SRGB_BLOCK`"]
    Astc4x4SrgbBlock = 158,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x4_UNORM_BLOCK`"]
    Astc5x4UnormBlock = 159,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x4_SRGB_BLOCK`"]
    Astc5x4SrgbBlock = 160,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x5_UNORM_BLOCK`"]
    Astc5x5UnormBlock = 161,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x5_SRGB_BLOCK`"]
    Astc5x5SrgbBlock = 162,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x5_UNORM_BLOCK`"]
    Astc6x5UnormBlock = 163,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x5_SRGB_BLOCK`"]
    Astc6x5SrgbBlock = 164,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x6_UNORM_BLOCK`"]
    Astc6x6UnormBlock = 165,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x6_SRGB_BLOCK`"]
    Astc6x6SrgbBlock = 166,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x5_UNORM_BLOCK`"]
    Astc8x5UnormBlock = 167,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x5_SRGB_BLOCK`"]
    Astc8x5SrgbBlock = 168,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x6_UNORM_BLOCK`"]
    Astc8x6UnormBlock = 169,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x6_SRGB_BLOCK`"]
    Astc8x6SrgbBlock = 170,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x8_UNORM_BLOCK`"]
    Astc8x8UnormBlock = 171,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x8_SRGB_BLOCK`"]
    Astc8x8SrgbBlock = 172,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x5_UNORM_BLOCK`"]
    Astc10x5UnormBlock = 173,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x5_SRGB_BLOCK`"]
    Astc10x5SrgbBlock = 174,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x6_UNORM_BLOCK`"]
    Astc10x6UnormBlock = 175,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x6_SRGB_BLOCK`"]
    Astc10x6SrgbBlock = 176,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x8_UNORM_BLOCK`"]
    Astc10x8UnormBlock = 177,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x8_SRGB_BLOCK`"]
    Astc10x8SrgbBlock = 178,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x10_UNORM_BLOCK`"]
    Astc10x10UnormBlock = 179,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x10_SRGB_BLOCK`"]
    Astc10x10SrgbBlock = 180,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x10_UNORM_BLOCK`"]
    Astc12x10UnormBlock = 181,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x10_SRGB_BLOCK`"]
    Astc12x10SrgbBlock = 182,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x12_UNORM_BLOCK`"]
    Astc12x12UnormBlock = 183,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x12_SRGB_BLOCK`"]
    Astc12x12SrgbBlock = 184,
    #[doc = "**Translated from**: `VK_FORMAT_G8B8G8R8_422_UNORM`"]
    G8b8g8r8422Unorm = 1000156000,
    #[doc = "**Translated from**: `VK_FORMAT_B8G8R8G8_422_UNORM`"]
    B8g8r8g8422Unorm = 1000156001,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM`"]
    G8B8R83plane420Unorm = 1000156002,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM`"]
    G8B8r82plane420Unorm = 1000156003,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM`"]
    G8B8R83plane422Unorm = 1000156004,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8R8_2PLANE_422_UNORM`"]
    G8B8r82plane422Unorm = 1000156005,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM`"]
    G8B8R83plane444Unorm = 1000156006,
    #[doc = "**Translated from**: `VK_FORMAT_R10X6_UNORM_PACK16`"]
    R10x6UnormPack16 = 1000156007,
    #[doc = "**Translated from**: `VK_FORMAT_R10X6G10X6_UNORM_2PACK16`"]
    R10x6g10x6Unorm2pack16 = 1000156008,
    #[doc = "**Translated from**: `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`"]
    R10x6g10x6b10x6a10x6Unorm4pack16 = 1000156009,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16`"]
    G10x6b10x6g10x6r10x6422Unorm4pack16 = 1000156010,
    #[doc = "**Translated from**: `VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16`"]
    B10x6g10x6r10x6g10x6422Unorm4pack16 = 1000156011,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16`"]
    G10x6B10x6R10x63plane420Unorm3pack16 = 1000156012,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16`"]
    G10x6B10x6r10x62plane420Unorm3pack16 = 1000156013,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16`"]
    G10x6B10x6R10x63plane422Unorm3pack16 = 1000156014,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16`"]
    G10x6B10x6r10x62plane422Unorm3pack16 = 1000156015,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16`"]
    G10x6B10x6R10x63plane444Unorm3pack16 = 1000156016,
    #[doc = "**Translated from**: `VK_FORMAT_R12X4_UNORM_PACK16`"]
    R12x4UnormPack16 = 1000156017,
    #[doc = "**Translated from**: `VK_FORMAT_R12X4G12X4_UNORM_2PACK16`"]
    R12x4g12x4Unorm2pack16 = 1000156018,
    #[doc = "**Translated from**: `VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16`"]
    R12x4g12x4b12x4a12x4Unorm4pack16 = 1000156019,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16`"]
    G12x4b12x4g12x4r12x4422Unorm4pack16 = 1000156020,
    #[doc = "**Translated from**: `VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16`"]
    B12x4g12x4r12x4g12x4422Unorm4pack16 = 1000156021,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16`"]
    G12x4B12x4R12x43plane420Unorm3pack16 = 1000156022,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16`"]
    G12x4B12x4r12x42plane420Unorm3pack16 = 1000156023,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16`"]
    G12x4B12x4R12x43plane422Unorm3pack16 = 1000156024,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16`"]
    G12x4B12x4r12x42plane422Unorm3pack16 = 1000156025,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16`"]
    G12x4B12x4R12x43plane444Unorm3pack16 = 1000156026,
    #[doc = "**Translated from**: `VK_FORMAT_G16B16G16R16_422_UNORM`"]
    G16b16g16r16422Unorm = 1000156027,
    #[doc = "**Translated from**: `VK_FORMAT_B16G16R16G16_422_UNORM`"]
    B16g16r16g16422Unorm = 1000156028,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM`"]
    G16B16R163plane420Unorm = 1000156029,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16R16_2PLANE_420_UNORM`"]
    G16B16r162plane420Unorm = 1000156030,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM`"]
    G16B16R163plane422Unorm = 1000156031,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16R16_2PLANE_422_UNORM`"]
    G16B16r162plane422Unorm = 1000156032,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM`"]
    G16B16R163plane444Unorm = 1000156033,
    #[doc = "**Translated from**: `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM`"]
    G8B8r82plane444Unorm = 1000330000,
    #[doc = "**Translated from**: `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`"]
    G10x6B10x6r10x62plane444Unorm3pack16 = 1000330001,
    #[doc = "**Translated from**: `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`"]
    G12x4B12x4r12x42plane444Unorm3pack16 = 1000330002,
    #[doc = "**Translated from**: `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM`"]
    G16B16r162plane444Unorm = 1000330003,
    #[doc = "**Translated from**: `VK_FORMAT_A4R4G4B4_UNORM_PACK16`"]
    A4r4g4b4UnormPack16 = 1000340000,
    #[doc = "**Translated from**: `VK_FORMAT_A4B4G4R4_UNORM_PACK16`"]
    A4b4g4r4UnormPack16 = 1000340001,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`"]
    Astc4x4SfloatBlock = 1000066000,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`"]
    Astc5x4SfloatBlock = 1000066001,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`"]
    Astc5x5SfloatBlock = 1000066002,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`"]
    Astc6x5SfloatBlock = 1000066003,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`"]
    Astc6x6SfloatBlock = 1000066004,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`"]
    Astc8x5SfloatBlock = 1000066005,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`"]
    Astc8x6SfloatBlock = 1000066006,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`"]
    Astc8x8SfloatBlock = 1000066007,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`"]
    Astc10x5SfloatBlock = 1000066008,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`"]
    Astc10x6SfloatBlock = 1000066009,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`"]
    Astc10x8SfloatBlock = 1000066010,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`"]
    Astc10x10SfloatBlock = 1000066011,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`"]
    Astc12x10SfloatBlock = 1000066012,
    #[doc = "**Translated from**: `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`"]
    Astc12x12SfloatBlock = 1000066013,
}

impl Format {
    #[must_use]
    pub fn aspect_mask(self) -> ImageAspectFlags {
        #[allow(clippy::match_same_arms)]
        match self {
            Format::Undefined => ImageAspectFlags::empty(),
            Format::R4g4UnormPack8 => ImageAspectFlags::COLOR,
            Format::R4g4b4a4UnormPack16 => ImageAspectFlags::COLOR,
            Format::B4g4r4a4UnormPack16 => ImageAspectFlags::COLOR,
            Format::R5g6b5UnormPack16 => ImageAspectFlags::COLOR,
            Format::B5g6r5UnormPack16 => ImageAspectFlags::COLOR,
            Format::R5g5b5a1UnormPack16 => ImageAspectFlags::COLOR,
            Format::B5g5r5a1UnormPack16 => ImageAspectFlags::COLOR,
            Format::A1r5g5b5UnormPack16 => ImageAspectFlags::COLOR,
            Format::R8Unorm => ImageAspectFlags::COLOR,
            Format::R8Snorm => ImageAspectFlags::COLOR,
            Format::R8Uscaled => ImageAspectFlags::COLOR,
            Format::R8Sscaled => ImageAspectFlags::COLOR,
            Format::R8Uint => ImageAspectFlags::COLOR,
            Format::R8Sint => ImageAspectFlags::COLOR,
            Format::R8Srgb => ImageAspectFlags::COLOR,
            Format::R8g8Unorm => ImageAspectFlags::COLOR,
            Format::R8g8Snorm => ImageAspectFlags::COLOR,
            Format::R8g8Uscaled => ImageAspectFlags::COLOR,
            Format::R8g8Sscaled => ImageAspectFlags::COLOR,
            Format::R8g8Uint => ImageAspectFlags::COLOR,
            Format::R8g8Sint => ImageAspectFlags::COLOR,
            Format::R8g8Srgb => ImageAspectFlags::COLOR,
            Format::R8g8b8Unorm => ImageAspectFlags::COLOR,
            Format::R8g8b8Snorm => ImageAspectFlags::COLOR,
            Format::R8g8b8Uscaled => ImageAspectFlags::COLOR,
            Format::R8g8b8Sscaled => ImageAspectFlags::COLOR,
            Format::R8g8b8Uint => ImageAspectFlags::COLOR,
            Format::R8g8b8Sint => ImageAspectFlags::COLOR,
            Format::R8g8b8Srgb => ImageAspectFlags::COLOR,
            Format::B8g8r8Unorm => ImageAspectFlags::COLOR,
            Format::B8g8r8Snorm => ImageAspectFlags::COLOR,
            Format::B8g8r8Uscaled => ImageAspectFlags::COLOR,
            Format::B8g8r8Sscaled => ImageAspectFlags::COLOR,
            Format::B8g8r8Uint => ImageAspectFlags::COLOR,
            Format::B8g8r8Sint => ImageAspectFlags::COLOR,
            Format::B8g8r8Srgb => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Unorm => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Snorm => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Uscaled => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Sscaled => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Uint => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Sint => ImageAspectFlags::COLOR,
            Format::R8g8b8a8Srgb => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Unorm => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Snorm => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Uscaled => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Sscaled => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Uint => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Sint => ImageAspectFlags::COLOR,
            Format::B8g8r8a8Srgb => ImageAspectFlags::COLOR,
            Format::A8b8g8r8UnormPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8SnormPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8UscaledPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8SscaledPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8UintPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8SintPack32 => ImageAspectFlags::COLOR,
            Format::A8b8g8r8SrgbPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10UnormPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10SnormPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10UscaledPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10SscaledPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10UintPack32 => ImageAspectFlags::COLOR,
            Format::A2r10g10b10SintPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10UnormPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10SnormPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10UscaledPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10SscaledPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10UintPack32 => ImageAspectFlags::COLOR,
            Format::A2b10g10r10SintPack32 => ImageAspectFlags::COLOR,
            Format::R16Unorm => ImageAspectFlags::COLOR,
            Format::R16Snorm => ImageAspectFlags::COLOR,
            Format::R16Uscaled => ImageAspectFlags::COLOR,
            Format::R16Sscaled => ImageAspectFlags::COLOR,
            Format::R16Uint => ImageAspectFlags::COLOR,
            Format::R16Sint => ImageAspectFlags::COLOR,
            Format::R16Sfloat => ImageAspectFlags::COLOR,
            Format::R16g16Unorm => ImageAspectFlags::COLOR,
            Format::R16g16Snorm => ImageAspectFlags::COLOR,
            Format::R16g16Uscaled => ImageAspectFlags::COLOR,
            Format::R16g16Sscaled => ImageAspectFlags::COLOR,
            Format::R16g16Uint => ImageAspectFlags::COLOR,
            Format::R16g16Sint => ImageAspectFlags::COLOR,
            Format::R16g16Sfloat => ImageAspectFlags::COLOR,
            Format::R16g16b16Unorm => ImageAspectFlags::COLOR,
            Format::R16g16b16Snorm => ImageAspectFlags::COLOR,
            Format::R16g16b16Uscaled => ImageAspectFlags::COLOR,
            Format::R16g16b16Sscaled => ImageAspectFlags::COLOR,
            Format::R16g16b16Uint => ImageAspectFlags::COLOR,
            Format::R16g16b16Sint => ImageAspectFlags::COLOR,
            Format::R16g16b16Sfloat => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Unorm => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Snorm => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Uscaled => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Sscaled => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Uint => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Sint => ImageAspectFlags::COLOR,
            Format::R16g16b16a16Sfloat => ImageAspectFlags::COLOR,
            Format::R32Uint => ImageAspectFlags::COLOR,
            Format::R32Sint => ImageAspectFlags::COLOR,
            Format::R32Sfloat => ImageAspectFlags::COLOR,
            Format::R32g32Uint => ImageAspectFlags::COLOR,
            Format::R32g32Sint => ImageAspectFlags::COLOR,
            Format::R32g32Sfloat => ImageAspectFlags::COLOR,
            Format::R32g32b32Uint => ImageAspectFlags::COLOR,
            Format::R32g32b32Sint => ImageAspectFlags::COLOR,
            Format::R32g32b32Sfloat => ImageAspectFlags::COLOR,
            Format::R32g32b32a32Uint => ImageAspectFlags::COLOR,
            Format::R32g32b32a32Sint => ImageAspectFlags::COLOR,
            Format::R32g32b32a32Sfloat => ImageAspectFlags::COLOR,
            Format::R64Uint => ImageAspectFlags::COLOR,
            Format::R64Sint => ImageAspectFlags::COLOR,
            Format::R64Sfloat => ImageAspectFlags::COLOR,
            Format::R64g64Uint => ImageAspectFlags::COLOR,
            Format::R64g64Sint => ImageAspectFlags::COLOR,
            Format::R64g64Sfloat => ImageAspectFlags::COLOR,
            Format::R64g64b64Uint => ImageAspectFlags::COLOR,
            Format::R64g64b64Sint => ImageAspectFlags::COLOR,
            Format::R64g64b64Sfloat => ImageAspectFlags::COLOR,
            Format::R64g64b64a64Uint => ImageAspectFlags::COLOR,
            Format::R64g64b64a64Sint => ImageAspectFlags::COLOR,
            Format::R64g64b64a64Sfloat => ImageAspectFlags::COLOR,
            Format::B10g11r11UfloatPack32 => ImageAspectFlags::COLOR,
            Format::E5b9g9r9UfloatPack32 => ImageAspectFlags::COLOR,
            Format::D16Unorm => ImageAspectFlags::DEPTH,
            Format::X8D24UnormPack32 => ImageAspectFlags::DEPTH,
            Format::D32Sfloat => ImageAspectFlags::DEPTH,
            Format::S8Uint => ImageAspectFlags::STENCIL,
            Format::D16UnormS8Uint => ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL,
            Format::D24UnormS8Uint => ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL,
            Format::D32SfloatS8Uint => ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL,
            Format::Bc1RgbUnormBlock => ImageAspectFlags::COLOR,
            Format::Bc1RgbSrgbBlock => ImageAspectFlags::COLOR,
            Format::Bc1RgbaUnormBlock => ImageAspectFlags::COLOR,
            Format::Bc1RgbaSrgbBlock => ImageAspectFlags::COLOR,
            Format::Bc2UnormBlock => ImageAspectFlags::COLOR,
            Format::Bc2SrgbBlock => ImageAspectFlags::COLOR,
            Format::Bc3UnormBlock => ImageAspectFlags::COLOR,
            Format::Bc3SrgbBlock => ImageAspectFlags::COLOR,
            Format::Bc4UnormBlock => ImageAspectFlags::COLOR,
            Format::Bc4SnormBlock => ImageAspectFlags::COLOR,
            Format::Bc5UnormBlock => ImageAspectFlags::COLOR,
            Format::Bc5SnormBlock => ImageAspectFlags::COLOR,
            Format::Bc6hUfloatBlock => ImageAspectFlags::COLOR,
            Format::Bc6hSfloatBlock => ImageAspectFlags::COLOR,
            Format::Bc7UnormBlock => ImageAspectFlags::COLOR,
            Format::Bc7SrgbBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8UnormBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8SrgbBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8a1UnormBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8a1SrgbBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8a8UnormBlock => ImageAspectFlags::COLOR,
            Format::Etc2R8g8b8a8SrgbBlock => ImageAspectFlags::COLOR,
            Format::EacR11UnormBlock => ImageAspectFlags::COLOR,
            Format::EacR11SnormBlock => ImageAspectFlags::COLOR,
            Format::EacR11g11UnormBlock => ImageAspectFlags::COLOR,
            Format::EacR11g11SnormBlock => ImageAspectFlags::COLOR,
            Format::Astc4x4UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc4x4SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc5x4UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc5x4SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc5x5UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc5x5SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc6x5UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc6x5SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc6x6UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc6x6SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc8x5UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc8x5SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc8x6UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc8x6SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc8x8UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc8x8SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc10x5UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc10x5SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc10x6UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc10x6SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc10x8UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc10x8SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc10x10UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc10x10SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc12x10UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc12x10SrgbBlock => ImageAspectFlags::COLOR,
            Format::Astc12x12UnormBlock => ImageAspectFlags::COLOR,
            Format::Astc12x12SrgbBlock => ImageAspectFlags::COLOR,
            Format::G8b8g8r8422Unorm => ImageAspectFlags::COLOR,
            Format::B8g8r8g8422Unorm => ImageAspectFlags::COLOR,
            Format::G8B8R83plane420Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G8B8r82plane420Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G8B8R83plane422Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G8B8r82plane422Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G8B8R83plane444Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::R10x6UnormPack16 => ImageAspectFlags::COLOR,
            Format::R10x6g10x6Unorm2pack16 => ImageAspectFlags::COLOR,
            Format::R10x6g10x6b10x6a10x6Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::G10x6b10x6g10x6r10x6422Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::B10x6g10x6r10x6g10x6422Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::G10x6B10x6R10x63plane420Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G10x6B10x6r10x62plane420Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G10x6B10x6R10x63plane422Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G10x6B10x6r10x62plane422Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G10x6B10x6R10x63plane444Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::R12x4UnormPack16 => ImageAspectFlags::COLOR,
            Format::R12x4g12x4Unorm2pack16 => ImageAspectFlags::COLOR,
            Format::R12x4g12x4b12x4a12x4Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::G12x4b12x4g12x4r12x4422Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::B12x4g12x4r12x4g12x4422Unorm4pack16 => ImageAspectFlags::COLOR,
            Format::G12x4B12x4R12x43plane420Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G12x4B12x4r12x42plane420Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G12x4B12x4R12x43plane422Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G12x4B12x4r12x42plane422Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G12x4B12x4R12x43plane444Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G16b16g16r16422Unorm => ImageAspectFlags::COLOR,
            Format::B16g16r16g16422Unorm => ImageAspectFlags::COLOR,
            Format::G16B16R163plane420Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G16B16r162plane420Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G16B16R163plane422Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G16B16r162plane422Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G16B16R163plane444Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1 | ImageAspectFlags::PLANE_2,
            Format::G8B8r82plane444Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G10x6B10x6r10x62plane444Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G12x4B12x4r12x42plane444Unorm3pack16 => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::G16B16r162plane444Unorm => ImageAspectFlags::COLOR | ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
            Format::A4r4g4b4UnormPack16 => ImageAspectFlags::COLOR,
            Format::A4b4g4r4UnormPack16 => ImageAspectFlags::COLOR,
            Format::Astc4x4SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc5x4SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc5x5SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc6x5SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc6x6SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc8x5SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc8x6SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc8x8SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc10x5SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc10x6SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc10x8SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc10x10SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc12x10SfloatBlock => ImageAspectFlags::COLOR,
            Format::Astc12x12SfloatBlock => ImageAspectFlags::COLOR,
        }
    }

    #[must_use]
    pub const fn block_size(self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Format::Undefined => 0,
            Format::R4g4UnormPack8 => 1,
            Format::R4g4b4a4UnormPack16 => 2,
            Format::B4g4r4a4UnormPack16 => 2,
            Format::R5g6b5UnormPack16 => 2,
            Format::B5g6r5UnormPack16 => 2,
            Format::R5g5b5a1UnormPack16 => 2,
            Format::B5g5r5a1UnormPack16 => 2,
            Format::A1r5g5b5UnormPack16 => 2,
            Format::R8Unorm => 1,
            Format::R8Snorm => 1,
            Format::R8Uscaled => 1,
            Format::R8Sscaled => 1,
            Format::R8Uint => 1,
            Format::R8Sint => 1,
            Format::R8Srgb => 1,
            Format::R8g8Unorm => 2,
            Format::R8g8Snorm => 2,
            Format::R8g8Uscaled => 2,
            Format::R8g8Sscaled => 2,
            Format::R8g8Uint => 2,
            Format::R8g8Sint => 2,
            Format::R8g8Srgb => 2,
            Format::R8g8b8Unorm => 3,
            Format::R8g8b8Snorm => 3,
            Format::R8g8b8Uscaled => 3,
            Format::R8g8b8Sscaled => 3,
            Format::R8g8b8Uint => 3,
            Format::R8g8b8Sint => 3,
            Format::R8g8b8Srgb => 3,
            Format::B8g8r8Unorm => 3,
            Format::B8g8r8Snorm => 3,
            Format::B8g8r8Uscaled => 3,
            Format::B8g8r8Sscaled => 3,
            Format::B8g8r8Uint => 3,
            Format::B8g8r8Sint => 3,
            Format::B8g8r8Srgb => 3,
            Format::R8g8b8a8Unorm => 4,
            Format::R8g8b8a8Snorm => 4,
            Format::R8g8b8a8Uscaled => 4,
            Format::R8g8b8a8Sscaled => 4,
            Format::R8g8b8a8Uint => 4,
            Format::R8g8b8a8Sint => 4,
            Format::R8g8b8a8Srgb => 4,
            Format::B8g8r8a8Unorm => 4,
            Format::B8g8r8a8Snorm => 4,
            Format::B8g8r8a8Uscaled => 4,
            Format::B8g8r8a8Sscaled => 4,
            Format::B8g8r8a8Uint => 4,
            Format::B8g8r8a8Sint => 4,
            Format::B8g8r8a8Srgb => 4,
            Format::A8b8g8r8UnormPack32 => 4,
            Format::A8b8g8r8SnormPack32 => 4,
            Format::A8b8g8r8UscaledPack32 => 4,
            Format::A8b8g8r8SscaledPack32 => 4,
            Format::A8b8g8r8UintPack32 => 4,
            Format::A8b8g8r8SintPack32 => 4,
            Format::A8b8g8r8SrgbPack32 => 4,
            Format::A2r10g10b10UnormPack32 => 4,
            Format::A2r10g10b10SnormPack32 => 4,
            Format::A2r10g10b10UscaledPack32 => 4,
            Format::A2r10g10b10SscaledPack32 => 4,
            Format::A2r10g10b10UintPack32 => 4,
            Format::A2r10g10b10SintPack32 => 4,
            Format::A2b10g10r10UnormPack32 => 4,
            Format::A2b10g10r10SnormPack32 => 4,
            Format::A2b10g10r10UscaledPack32 => 4,
            Format::A2b10g10r10SscaledPack32 => 4,
            Format::A2b10g10r10UintPack32 => 4,
            Format::A2b10g10r10SintPack32 => 4,
            Format::R16Unorm => 2,
            Format::R16Snorm => 2,
            Format::R16Uscaled => 2,
            Format::R16Sscaled => 2,
            Format::R16Uint => 2,
            Format::R16Sint => 2,
            Format::R16Sfloat => 2,
            Format::R16g16Unorm => 4,
            Format::R16g16Snorm => 4,
            Format::R16g16Uscaled => 4,
            Format::R16g16Sscaled => 4,
            Format::R16g16Uint => 4,
            Format::R16g16Sint => 4,
            Format::R16g16Sfloat => 4,
            Format::R16g16b16Unorm => 6,
            Format::R16g16b16Snorm => 6,
            Format::R16g16b16Uscaled => 6,
            Format::R16g16b16Sscaled => 6,
            Format::R16g16b16Uint => 6,
            Format::R16g16b16Sint => 6,
            Format::R16g16b16Sfloat => 6,
            Format::R16g16b16a16Unorm => 8,
            Format::R16g16b16a16Snorm => 8,
            Format::R16g16b16a16Uscaled => 8,
            Format::R16g16b16a16Sscaled => 8,
            Format::R16g16b16a16Uint => 8,
            Format::R16g16b16a16Sint => 8,
            Format::R16g16b16a16Sfloat => 8,
            Format::R32Uint => 4,
            Format::R32Sint => 4,
            Format::R32Sfloat => 4,
            Format::R32g32Uint => 8,
            Format::R32g32Sint => 8,
            Format::R32g32Sfloat => 8,
            Format::R32g32b32Uint => 12,
            Format::R32g32b32Sint => 12,
            Format::R32g32b32Sfloat => 12,
            Format::R32g32b32a32Uint => 16,
            Format::R32g32b32a32Sint => 16,
            Format::R32g32b32a32Sfloat => 16,
            Format::R64Uint => 8,
            Format::R64Sint => 8,
            Format::R64Sfloat => 8,
            Format::R64g64Uint => 16,
            Format::R64g64Sint => 16,
            Format::R64g64Sfloat => 16,
            Format::R64g64b64Uint => 24,
            Format::R64g64b64Sint => 24,
            Format::R64g64b64Sfloat => 24,
            Format::R64g64b64a64Uint => 32,
            Format::R64g64b64a64Sint => 32,
            Format::R64g64b64a64Sfloat => 32,
            Format::B10g11r11UfloatPack32 => 4,
            Format::E5b9g9r9UfloatPack32 => 4,
            Format::D16Unorm => 2,
            Format::X8D24UnormPack32 => 4,
            Format::D32Sfloat => 4,
            Format::S8Uint => 1,
            Format::D16UnormS8Uint => 3,
            Format::D24UnormS8Uint => 4,
            Format::D32SfloatS8Uint => 5,
            Format::Bc1RgbUnormBlock => 8,
            Format::Bc1RgbSrgbBlock => 8,
            Format::Bc1RgbaUnormBlock => 8,
            Format::Bc1RgbaSrgbBlock => 8,
            Format::Bc2UnormBlock => 16,
            Format::Bc2SrgbBlock => 16,
            Format::Bc3UnormBlock => 16,
            Format::Bc3SrgbBlock => 16,
            Format::Bc4UnormBlock => 8,
            Format::Bc4SnormBlock => 8,
            Format::Bc5UnormBlock => 16,
            Format::Bc5SnormBlock => 16,
            Format::Bc6hUfloatBlock => 16,
            Format::Bc6hSfloatBlock => 16,
            Format::Bc7UnormBlock => 16,
            Format::Bc7SrgbBlock => 16,
            Format::Etc2R8g8b8UnormBlock => 8,
            Format::Etc2R8g8b8SrgbBlock => 8,
            Format::Etc2R8g8b8a1UnormBlock => 8,
            Format::Etc2R8g8b8a1SrgbBlock => 8,
            Format::Etc2R8g8b8a8UnormBlock => 16,
            Format::Etc2R8g8b8a8SrgbBlock => 16,
            Format::EacR11UnormBlock => 8,
            Format::EacR11SnormBlock => 8,
            Format::EacR11g11UnormBlock => 16,
            Format::EacR11g11SnormBlock => 16,
            Format::Astc4x4UnormBlock => 16,
            Format::Astc4x4SrgbBlock => 16,
            Format::Astc5x4UnormBlock => 16,
            Format::Astc5x4SrgbBlock => 16,
            Format::Astc5x5UnormBlock => 16,
            Format::Astc5x5SrgbBlock => 16,
            Format::Astc6x5UnormBlock => 16,
            Format::Astc6x5SrgbBlock => 16,
            Format::Astc6x6UnormBlock => 16,
            Format::Astc6x6SrgbBlock => 16,
            Format::Astc8x5UnormBlock => 16,
            Format::Astc8x5SrgbBlock => 16,
            Format::Astc8x6UnormBlock => 16,
            Format::Astc8x6SrgbBlock => 16,
            Format::Astc8x8UnormBlock => 16,
            Format::Astc8x8SrgbBlock => 16,
            Format::Astc10x5UnormBlock => 16,
            Format::Astc10x5SrgbBlock => 16,
            Format::Astc10x6UnormBlock => 16,
            Format::Astc10x6SrgbBlock => 16,
            Format::Astc10x8UnormBlock => 16,
            Format::Astc10x8SrgbBlock => 16,
            Format::Astc10x10UnormBlock => 16,
            Format::Astc10x10SrgbBlock => 16,
            Format::Astc12x10UnormBlock => 16,
            Format::Astc12x10SrgbBlock => 16,
            Format::Astc12x12UnormBlock => 16,
            Format::Astc12x12SrgbBlock => 16,
            Format::G8b8g8r8422Unorm => 4,
            Format::B8g8r8g8422Unorm => 4,
            Format::G8B8R83plane420Unorm => 3,
            Format::G8B8r82plane420Unorm => 3,
            Format::G8B8R83plane422Unorm => 3,
            Format::G8B8r82plane422Unorm => 3,
            Format::G8B8R83plane444Unorm => 3,
            Format::R10x6UnormPack16 => 2,
            Format::R10x6g10x6Unorm2pack16 => 4,
            Format::R10x6g10x6b10x6a10x6Unorm4pack16 => 8,
            Format::G10x6b10x6g10x6r10x6422Unorm4pack16 => 8,
            Format::B10x6g10x6r10x6g10x6422Unorm4pack16 => 8,
            Format::G10x6B10x6R10x63plane420Unorm3pack16 => 6,
            Format::G10x6B10x6r10x62plane420Unorm3pack16 => 6,
            Format::G10x6B10x6R10x63plane422Unorm3pack16 => 6,
            Format::G10x6B10x6r10x62plane422Unorm3pack16 => 6,
            Format::G10x6B10x6R10x63plane444Unorm3pack16 => 6,
            Format::R12x4UnormPack16 => 2,
            Format::R12x4g12x4Unorm2pack16 => 4,
            Format::R12x4g12x4b12x4a12x4Unorm4pack16 => 8,
            Format::G12x4b12x4g12x4r12x4422Unorm4pack16 => 8,
            Format::B12x4g12x4r12x4g12x4422Unorm4pack16 => 8,
            Format::G12x4B12x4R12x43plane420Unorm3pack16 => 6,
            Format::G12x4B12x4r12x42plane420Unorm3pack16 => 6,
            Format::G12x4B12x4R12x43plane422Unorm3pack16 => 6,
            Format::G12x4B12x4r12x42plane422Unorm3pack16 => 6,
            Format::G12x4B12x4R12x43plane444Unorm3pack16 => 6,
            Format::G16b16g16r16422Unorm => 8,
            Format::B16g16r16g16422Unorm => 8,
            Format::G16B16R163plane420Unorm => 6,
            Format::G16B16r162plane420Unorm => 6,
            Format::G16B16R163plane422Unorm => 6,
            Format::G16B16r162plane422Unorm => 6,
            Format::G16B16R163plane444Unorm => 6,
            Format::G8B8r82plane444Unorm => 3,
            Format::G10x6B10x6r10x62plane444Unorm3pack16 => 6,
            Format::G12x4B12x4r12x42plane444Unorm3pack16 => 6,
            Format::G16B16r162plane444Unorm => 6,
            Format::A4r4g4b4UnormPack16 => 2,
            Format::A4b4g4r4UnormPack16 => 2,
            Format::Astc4x4SfloatBlock => 16,
            Format::Astc5x4SfloatBlock => 16,
            Format::Astc5x5SfloatBlock => 16,
            Format::Astc6x5SfloatBlock => 16,
            Format::Astc6x6SfloatBlock => 16,
            Format::Astc8x5SfloatBlock => 16,
            Format::Astc8x6SfloatBlock => 16,
            Format::Astc8x8SfloatBlock => 16,
            Format::Astc10x5SfloatBlock => 16,
            Format::Astc10x6SfloatBlock => 16,
            Format::Astc10x8SfloatBlock => 16,
            Format::Astc10x10SfloatBlock => 16,
            Format::Astc12x10SfloatBlock => 16,
            Format::Astc12x12SfloatBlock => 16,
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Specify an enumeration to track object handle types"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkObjectType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkObjectType.html)"]
pub enum ObjectType {
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_UNKNOWN`"]
    Unknown = 0,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_INSTANCE`"]
    Instance = 1,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_PHYSICAL_DEVICE`"]
    PhysicalDevice = 2,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DEVICE`"]
    Device = 3,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_QUEUE`"]
    Queue = 4,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_SEMAPHORE`"]
    Semaphore = 5,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_COMMAND_BUFFER`"]
    CommandBuffer = 6,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_FENCE`"]
    Fence = 7,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DEVICE_MEMORY`"]
    DeviceMemory = 8,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_BUFFER`"]
    Buffer = 9,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_IMAGE`"]
    Image = 10,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_EVENT`"]
    Event = 11,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_QUERY_POOL`"]
    QueryPool = 12,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_BUFFER_VIEW`"]
    BufferView = 13,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_IMAGE_VIEW`"]
    ImageView = 14,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_SHADER_MODULE`"]
    ShaderModule = 15,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_PIPELINE_CACHE`"]
    PipelineCache = 16,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_PIPELINE_LAYOUT`"]
    PipelineLayout = 17,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_RENDER_PASS`"]
    RenderPass = 18,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_PIPELINE`"]
    Pipeline = 19,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT`"]
    DescriptorSetLayout = 20,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_SAMPLER`"]
    Sampler = 21,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DESCRIPTOR_POOL`"]
    DescriptorPool = 22,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DESCRIPTOR_SET`"]
    DescriptorSet = 23,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_FRAMEBUFFER`"]
    Framebuffer = 24,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_COMMAND_POOL`"]
    CommandPool = 25,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION`"]
    SamplerYcbcrConversion = 1000156000,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE`"]
    DescriptorUpdateTemplate = 1000085000,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_PRIVATE_DATA_SLOT`"]
    PrivateDataSlot = 1000295000,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT`"]
    DebugUtilsMessengerEXT = 1000128000,
    #[doc = "**Translated from**: `VK_OBJECT_TYPE_SHADER_EXT`"]
    ShaderEXT = 1000482000,
}

//
// Bitmasks
//

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Initialization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkInstanceCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkInstanceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)"]
    pub struct InstanceCreateFlags: u32 {

    }
}

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying behavior of the instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkInstanceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)"]
pub type InstanceCreateFlagBits = InstanceCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkQueueFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueueFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)"]
    pub struct QueueFlags: u32 {
        #[doc = "Translated from: `VK_QUEUE_GRAPHICS_BIT`"]
        const GRAPHICS = 0b1;
        #[doc = "Translated from: `VK_QUEUE_COMPUTE_BIT`"]
        const COMPUTE = 0b10;
        #[doc = "Translated from: `VK_QUEUE_TRANSFER_BIT`"]
        const TRANSFER = 0b100;
        #[doc = "Translated from: `VK_QUEUE_SPARSE_BINDING_BIT`"]
        const SPARSE_BINDING = 0b1000;
        #[doc = "Translated from: `VK_QUEUE_PROTECTED_BIT`"]
        const PROTECTED = 0b10000;
    }
}

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying capabilities of queues in a queue family"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)"]
pub type QueueFlagBits = QueueFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html)"]
    pub struct DeviceCreateFlags: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkDeviceQueueCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDeviceQueueCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)"]
    pub struct DeviceQueueCreateFlags: u32 {
        #[doc = "Translated from: `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT`"]
        const PROTECTED = 0b1;
    }
}

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying behavior of the queue"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceQueueCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)"]
pub type DeviceQueueCreateFlagBits = DeviceQueueCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkCommandPoolCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html)"]
    pub struct CommandPoolCreateFlags: u32 {
        #[doc = "Translated from: `VK_COMMAND_POOL_CREATE_TRANSIENT_BIT`"]
        const TRANSIENT = 0b1;
        #[doc = "Translated from: `VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT`"]
        const RESET_COMMAND_BUFFER = 0b10;
        #[doc = "Translated from: `VK_COMMAND_POOL_CREATE_PROTECTED_BIT`"]
        const PROTECTED = 0b100;
    }
}

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying usage behavior for a command pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html)"]
pub type CommandPoolCreateFlagBits = CommandPoolCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkCommandPoolResetFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html)"]
    pub struct CommandPoolResetFlags: u32 {
        #[doc = "Translated from: `VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT`"]
        const RELEASE_RESOURCES = 0b1;
    }
}

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask controlling behavior of a command pool reset"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html)"]
pub type CommandPoolResetFlagBits = CommandPoolResetFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkCommandBufferUsageFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html)"]
    pub struct CommandBufferUsageFlags: u32 {
        #[doc = "Translated from: `VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT`"]
        const ONE_TIME_SUBMIT = 0b1;
        #[doc = "Translated from: `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`"]
        const RENDER_PASS_CONTINUE = 0b10;
        #[doc = "Translated from: `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`"]
        const SIMULTANEOUS_USE = 0b100;
    }
}

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying usage behavior for command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html)"]
pub type CommandBufferUsageFlagBits = CommandBufferUsageFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkSubmitFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSubmitFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSubmitFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html)"]
    pub struct SubmitFlags: u32 {
        #[doc = "Translated from: `VK_SUBMIT_PROTECTED_BIT`"]
        const PROTECTED = 0b1;
    }
}

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying behavior of a submission"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSubmitFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html)"]
pub type SubmitFlagBits = SubmitFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: 64-bit mask of pipeline stage flags"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkPipelineStageFlags2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlags2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkPipelineStageFlagBits2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html)"]
    pub struct PipelineStageFlags2: u64 {
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_NONE`"]
        const NONE = 0;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT`"]
        const TOP_OF_PIPE = 0b1;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`"]
        const DRAW_INDIRECT = 0b10;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`"]
        const VERTEX_INPUT = 0b100;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT`"]
        const VERTEX_SHADER = 0b1000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT`"]
        const TESSELLATION_CONTROL_SHADER = 0b10000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`"]
        const TESSELLATION_EVALUATION_SHADER = 0b100000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`"]
        const GEOMETRY_SHADER = 0b1000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`"]
        const FRAGMENT_SHADER = 0b10000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`"]
        const EARLY_FRAGMENT_TESTS = 0b100000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`"]
        const LATE_FRAGMENT_TESTS = 0b1000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT`"]
        const COLOR_ATTACHMENT_OUTPUT = 0b10000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT`"]
        const COMPUTE_SHADER = 0b100000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`"]
        const ALL_TRANSFER = 0b1000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT`"]
        const BOTTOM_OF_PIPE = 0b10000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_HOST_BIT`"]
        const HOST = 0b100000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`"]
        const ALL_GRAPHICS = 0b1000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`"]
        const ALL_COMMANDS = 0b10000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_COPY_BIT`"]
        const COPY = 0b100000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_RESOLVE_BIT`"]
        const RESOLVE = 0b1000000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_BLIT_BIT`"]
        const BLIT = 0b10000000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_CLEAR_BIT`"]
        const CLEAR = 0b100000000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`"]
        const INDEX_INPUT = 0b1000000000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`"]
        const VERTEX_ATTRIBUTE_INPUT = 0b10000000000000000000000000000000000000;
        #[doc = "Translated from: `VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT`"]
        const PRE_RASTERIZATION_SHADERS = 0b100000000000000000000000000000000000000;
    }
}

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Pipeline stage flags for VkPipelineStageFlags2"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPipelineStageFlagBits2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html)"]
pub type PipelineStageFlagBits2 = PipelineStageFlags2;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: 64-bit mask of access flags"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkAccessFlags2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlags2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkAccessFlagBits2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html)"]
    pub struct AccessFlags2: u64 {
        #[doc = "Translated from: `VK_ACCESS_2_NONE`"]
        const NONE = 0;
        #[doc = "Translated from: `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`"]
        const INDIRECT_COMMAND_READ = 0b1;
        #[doc = "Translated from: `VK_ACCESS_2_INDEX_READ_BIT`"]
        const INDEX_READ = 0b10;
        #[doc = "Translated from: `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`"]
        const VERTEX_ATTRIBUTE_READ = 0b100;
        #[doc = "Translated from: `VK_ACCESS_2_UNIFORM_READ_BIT`"]
        const UNIFORM_READ = 0b1000;
        #[doc = "Translated from: `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`"]
        const INPUT_ATTACHMENT_READ = 0b10000;
        #[doc = "Translated from: `VK_ACCESS_2_SHADER_READ_BIT`"]
        const SHADER_READ = 0b100000;
        #[doc = "Translated from: `VK_ACCESS_2_SHADER_WRITE_BIT`"]
        const SHADER_WRITE = 0b1000000;
        #[doc = "Translated from: `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`"]
        const COLOR_ATTACHMENT_READ = 0b10000000;
        #[doc = "Translated from: `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`"]
        const COLOR_ATTACHMENT_WRITE = 0b100000000;
        #[doc = "Translated from: `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`"]
        const DEPTH_STENCIL_ATTACHMENT_READ = 0b1000000000;
        #[doc = "Translated from: `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`"]
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 0b10000000000;
        #[doc = "Translated from: `VK_ACCESS_2_TRANSFER_READ_BIT`"]
        const TRANSFER_READ = 0b100000000000;
        #[doc = "Translated from: `VK_ACCESS_2_TRANSFER_WRITE_BIT`"]
        const TRANSFER_WRITE = 0b1000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_HOST_READ_BIT`"]
        const HOST_READ = 0b10000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_HOST_WRITE_BIT`"]
        const HOST_WRITE = 0b100000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_MEMORY_READ_BIT`"]
        const MEMORY_READ = 0b1000000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_MEMORY_WRITE_BIT`"]
        const MEMORY_WRITE = 0b10000000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`"]
        const SHADER_SAMPLED_READ = 0b100000000000000000000000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`"]
        const SHADER_STORAGE_READ = 0b1000000000000000000000000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`"]
        const SHADER_STORAGE_WRITE = 0b10000000000000000000000000000000000;
        #[doc = "Translated from: `VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_READ_EXT = 0b100000000000000000000000000000000000000000;
    }
}

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Access flags for VkAccessFlags2"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkAccessFlagBits2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html)"]
pub type AccessFlagBits2 = AccessFlags2;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlags.html)"]
    pub struct SemaphoreCreateFlags: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkSemaphoreWaitFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSemaphoreWaitFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSemaphoreWaitFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html)"]
    pub struct SemaphoreWaitFlags: u32 {
        #[doc = "Translated from: `VK_SEMAPHORE_WAIT_ANY_BIT`"]
        const ANY = 0b1;
    }
}

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional parameters of a semaphore wait operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreWaitFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html)"]
pub type SemaphoreWaitFlagBits = SemaphoreWaitFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkDependencyFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDependencyFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html)"]
    pub struct DependencyFlags: u32 {
        #[doc = "Translated from: `VK_DEPENDENCY_BY_REGION_BIT`"]
        const BY_REGION = 0b1;
        #[doc = "Translated from: `VK_DEPENDENCY_DEVICE_GROUP_BIT`"]
        const DEVICE_GROUP = 0b100;
        #[doc = "Translated from: `VK_DEPENDENCY_VIEW_LOCAL_BIT`"]
        const VIEW_LOCAL = 0b10;
    }
}

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying how execution and memory dependencies are formed"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html)"]
pub type DependencyFlagBits = DependencyFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Render Pass"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkRenderingFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkRenderingFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkRenderingFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html)"]
    pub struct RenderingFlags: u32 {
        #[doc = "Translated from: `VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT`"]
        const CONTENTS_SECONDARY_COMMAND_BUFFERS = 0b1;
        #[doc = "Translated from: `VK_RENDERING_SUSPENDING_BIT`"]
        const SUSPENDING = 0b10;
        #[doc = "Translated from: `VK_RENDERING_RESUMING_BIT`"]
        const RESUMING = 0b100;
    }
}

#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional properties of a dynamic render pass instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkRenderingFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html)"]
pub type RenderingFlagBits = RenderingFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Render Pass"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkResolveModeFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkResolveModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkResolveModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html)"]
    pub struct ResolveModeFlags: u32 {
        #[doc = "Translated from: `VK_RESOLVE_MODE_NONE`"]
        const NONE = 0;
        #[doc = "Translated from: `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`"]
        const SAMPLE_ZERO = 0b1;
        #[doc = "Translated from: `VK_RESOLVE_MODE_AVERAGE_BIT`"]
        const AVERAGE = 0b10;
        #[doc = "Translated from: `VK_RESOLVE_MODE_MIN_BIT`"]
        const MIN = 0b100;
        #[doc = "Translated from: `VK_RESOLVE_MODE_MAX_BIT`"]
        const MAX = 0b1000;
    }
}

#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask indicating supported depth and stencil resolve modes"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkResolveModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html)"]
pub type ResolveModeFlagBits = ResolveModeFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkShaderCreateFlagBitsEXT"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkShaderCreateFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCreateFlagsEXT.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkShaderCreateFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCreateFlagBitsEXT.html)"]
    pub struct ShaderCreateFlagsEXT: u32 {
        #[doc = "Translated from: `VK_SHADER_CREATE_LINK_STAGE_BIT_EXT`"]
        const LINK_STAGE_EXT = 0b1;
        #[doc = "Translated from: `VK_SHADER_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT`"]
        const ALLOW_VARYING_SUBGROUP_SIZE_EXT = 0b10;
        #[doc = "Translated from: `VK_SHADER_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT`"]
        const REQUIRE_FULL_SUBGROUPS_EXT = 0b100;
        #[doc = "Translated from: `VK_SHADER_CREATE_NO_TASK_SHADER_BIT_EXT`"]
        const NO_TASK_SHADER_EXT = 0b1000;
        #[doc = "Translated from: `VK_SHADER_CREATE_DISPATCH_BASE_BIT_EXT`"]
        const DISPATCH_BASE_EXT = 0b10000;
        #[doc = "Translated from: `VK_SHADER_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_EXT`"]
        const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT = 0b100000;
        #[doc = "Translated from: `VK_SHADER_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`"]
        const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = 0b1000000;
    }
}

#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask controlling how a shader object is created"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkShaderCreateFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCreateFlagBitsEXT.html)"]
pub type ShaderCreateFlagBitsEXT = ShaderCreateFlagsEXT;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Pipelines"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkShaderStageFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkShaderStageFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html)"]
    pub struct ShaderStageFlags: u32 {
        #[doc = "Translated from: `VK_SHADER_STAGE_VERTEX_BIT`"]
        const VERTEX = 0b1;
        #[doc = "Translated from: `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`"]
        const TESSELLATION_CONTROL = 0b10;
        #[doc = "Translated from: `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`"]
        const TESSELLATION_EVALUATION = 0b100;
        #[doc = "Translated from: `VK_SHADER_STAGE_GEOMETRY_BIT`"]
        const GEOMETRY = 0b1000;
        #[doc = "Translated from: `VK_SHADER_STAGE_FRAGMENT_BIT`"]
        const FRAGMENT = 0b10000;
        #[doc = "Translated from: `VK_SHADER_STAGE_COMPUTE_BIT`"]
        const COMPUTE = 0b100000;
        #[doc = "Translated from: `VK_SHADER_STAGE_ALL_GRAPHICS`"]
        const ALL_GRAPHICS = 0x1f;
        #[doc = "Translated from: `VK_SHADER_STAGE_ALL`"]
        const ALL = 0x7fffffff;
        #[doc = "Translated from: `VK_SHADER_STAGE_TASK_BIT_EXT`"]
        const TASK_EXT = 0b1000000;
        #[doc = "Translated from: `VK_SHADER_STAGE_MESH_BIT_EXT`"]
        const MESH_EXT = 0b10000000;
    }
}

#[doc = "**Chapter**: Pipelines"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying a pipeline stage"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html)"]
pub type ShaderStageFlagBits = ShaderStageFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkMemoryHeapFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryHeapFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html)"]
    pub struct MemoryHeapFlags: u32 {
        #[doc = "Translated from: `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT`"]
        const DEVICE_LOCAL = 0b1;
        #[doc = "Translated from: `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`"]
        const MULTI_INSTANCE = 0b10;
    }
}

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying attribute flags for a heap"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html)"]
pub type MemoryHeapFlagBits = MemoryHeapFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkMemoryPropertyFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryPropertyFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html)"]
    pub struct MemoryPropertyFlags: u32 {
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`"]
        const DEVICE_LOCAL = 0b1;
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`"]
        const HOST_VISIBLE = 0b10;
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`"]
        const HOST_COHERENT = 0b100;
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`"]
        const HOST_CACHED = 0b1000;
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`"]
        const LAZILY_ALLOCATED = 0b10000;
        #[doc = "Translated from: `VK_MEMORY_PROPERTY_PROTECTED_BIT`"]
        const PROTECTED = 0b100000;
    }
}

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying properties for a memory type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html)"]
pub type MemoryPropertyFlagBits = MemoryPropertyFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkMemoryAllocateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryAllocateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryAllocateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html)"]
    pub struct MemoryAllocateFlags: u32 {
        #[doc = "Translated from: `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT`"]
        const DEVICE_MASK = 0b1;
        #[doc = "Translated from: `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`"]
        const DEVICE_ADDRESS = 0b10;
        #[doc = "Translated from: `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`"]
        const DEVICE_ADDRESS_CAPTURE_REPLAY = 0b100;
    }
}

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying flags for a device memory allocation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryAllocateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html)"]
pub type MemoryAllocateFlagBits = MemoryAllocateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html)"]
    pub struct MemoryMapFlags: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkMemoryUnmapFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryUnmapFlagsKHR.html)"]
    pub struct MemoryUnmapFlagsKHR: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkBufferUsageFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkBufferUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html)"]
    pub struct BufferUsageFlags: u32 {
        #[doc = "Translated from: `VK_BUFFER_USAGE_TRANSFER_SRC_BIT`"]
        const TRANSFER_SRC = 0b1;
        #[doc = "Translated from: `VK_BUFFER_USAGE_TRANSFER_DST_BIT`"]
        const TRANSFER_DST = 0b10;
        #[doc = "Translated from: `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT`"]
        const UNIFORM_TEXEL_BUFFER = 0b100;
        #[doc = "Translated from: `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`"]
        const STORAGE_TEXEL_BUFFER = 0b1000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT`"]
        const UNIFORM_BUFFER = 0b10000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT`"]
        const STORAGE_BUFFER = 0b100000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_INDEX_BUFFER_BIT`"]
        const INDEX_BUFFER = 0b1000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT`"]
        const VERTEX_BUFFER = 0b10000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT`"]
        const INDIRECT_BUFFER = 0b100000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`"]
        const SHADER_DEVICE_ADDRESS = 0b100000000000000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT`"]
        const SAMPLER_DESCRIPTOR_BUFFER_EXT = 0b1000000000000000000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT`"]
        const RESOURCE_DESCRIPTOR_BUFFER_EXT = 0b10000000000000000000000;
        #[doc = "Translated from: `VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT`"]
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = 0b100000000000000000000000000;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying allowed usage of a buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html)"]
pub type BufferUsageFlagBits = BufferUsageFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkBufferCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkBufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html)"]
    pub struct BufferCreateFlags: u32 {
        #[doc = "Translated from: `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`"]
        const SPARSE_BINDING = 0b1;
        #[doc = "Translated from: `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`"]
        const SPARSE_RESIDENCY = 0b10;
        #[doc = "Translated from: `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT`"]
        const SPARSE_ALIASED = 0b100;
        #[doc = "Translated from: `VK_BUFFER_CREATE_PROTECTED_BIT`"]
        const PROTECTED = 0b1000;
        #[doc = "Translated from: `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`"]
        const DEVICE_ADDRESS_CAPTURE_REPLAY = 0b10000;
        #[doc = "Translated from: `VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 0b100000;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional parameters of a buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html)"]
pub type BufferCreateFlagBits = BufferCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkImageUsageFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageUsageFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html)"]
    pub struct ImageUsageFlags: u32 {
        #[doc = "Translated from: `VK_IMAGE_USAGE_TRANSFER_SRC_BIT`"]
        const TRANSFER_SRC = 0b1;
        #[doc = "Translated from: `VK_IMAGE_USAGE_TRANSFER_DST_BIT`"]
        const TRANSFER_DST = 0b10;
        #[doc = "Translated from: `VK_IMAGE_USAGE_SAMPLED_BIT`"]
        const SAMPLED = 0b100;
        #[doc = "Translated from: `VK_IMAGE_USAGE_STORAGE_BIT`"]
        const STORAGE = 0b1000;
        #[doc = "Translated from: `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`"]
        const COLOR_ATTACHMENT = 0b10000;
        #[doc = "Translated from: `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`"]
        const DEPTH_STENCIL_ATTACHMENT = 0b100000;
        #[doc = "Translated from: `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`"]
        const TRANSIENT_ATTACHMENT = 0b1000000;
        #[doc = "Translated from: `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`"]
        const INPUT_ATTACHMENT = 0b10000000;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying intended usage of an image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html)"]
pub type ImageUsageFlagBits = ImageUsageFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkImageCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html)"]
    pub struct ImageCreateFlags: u32 {
        #[doc = "Translated from: `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`"]
        const SPARSE_BINDING = 0b1;
        #[doc = "Translated from: `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`"]
        const SPARSE_RESIDENCY = 0b10;
        #[doc = "Translated from: `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`"]
        const SPARSE_ALIASED = 0b100;
        #[doc = "Translated from: `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`"]
        const MUTABLE_FORMAT = 0b1000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT`"]
        const CUBE_COMPATIBLE = 0b10000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_ALIAS_BIT`"]
        const ALIAS = 0b10000000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`"]
        const SPLIT_INSTANCE_BIND_REGIONS = 0b1000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`"]
        const NUM_2D_ARRAY_COMPATIBLE = 0b100000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT`"]
        const BLOCK_TEXEL_VIEW_COMPATIBLE = 0b10000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT`"]
        const EXTENDED_USAGE = 0b100000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_PROTECTED_BIT`"]
        const PROTECTED = 0b100000000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_DISJOINT_BIT`"]
        const DISJOINT = 0b1000000000;
        #[doc = "Translated from: `VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 0b10000000000000000;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional parameters of an image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html)"]
pub type ImageCreateFlagBits = ImageCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageViewCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html)"]
    pub struct ImageViewCreateFlags: u32 {
        #[doc = "Translated from: `VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 0b100;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional parameters of an image view"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageViewCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html)"]
pub type ImageViewCreateFlagBits = ImageViewCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkImageAspectFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageAspectFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)"]
    pub struct ImageAspectFlags: u32 {
        #[doc = "Translated from: `VK_IMAGE_ASPECT_COLOR_BIT`"]
        const COLOR = 0b1;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_DEPTH_BIT`"]
        const DEPTH = 0b10;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_STENCIL_BIT`"]
        const STENCIL = 0b100;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_METADATA_BIT`"]
        const METADATA = 0b1000;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_PLANE_0_BIT`"]
        const PLANE_0 = 0b10000;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_PLANE_1_BIT`"]
        const PLANE_1 = 0b100000;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_PLANE_2_BIT`"]
        const PLANE_2 = 0b1000000;
        #[doc = "Translated from: `VK_IMAGE_ASPECT_NONE`"]
        const NONE = 0;
    }
}

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying which aspects of an image are included in a view"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)"]
pub type ImageAspectFlagBits = ImageAspectFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Samplers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSamplerCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html)"]
    pub struct SamplerCreateFlags: u32 {
        #[doc = "Translated from: `VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = 0b1000;
    }
}

#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying additional parameters of sampler"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSamplerCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html)"]
pub type SamplerCreateFlagBits = SamplerCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkDescriptorSetLayoutCreateFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDescriptorSetLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html)"]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        #[doc = "Translated from: `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`"]
        const UPDATE_AFTER_BIND_POOL = 0b10;
        #[doc = "Translated from: `VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT`"]
        const DESCRIPTOR_BUFFER_EXT = 0b10000;
        #[doc = "Translated from: `VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT`"]
        const EMBEDDED_IMMUTABLE_SAMPLERS_EXT = 0b100000;
    }
}

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying descriptor set layout properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html)"]
pub type DescriptorSetLayoutCreateFlagBits = DescriptorSetLayoutCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of pipeline layout creation flag bits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkPipelineLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html)"]
    pub struct PipelineLayoutCreateFlags: u32 {

    }
}

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Pipeline layout creation flag bits"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_graphics_pipeline_library`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_graphics_pipeline_library.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPipelineLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html)"]
pub type PipelineLayoutCreateFlagBits = PipelineLayoutCreateFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html)"]
    pub struct QueryPoolCreateFlags: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkQueryControlFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryControlFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html)"]
    pub struct QueryControlFlags: u32 {
        #[doc = "Translated from: `VK_QUERY_CONTROL_PRECISE_BIT`"]
        const PRECISE = 0b1;
    }
}

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying constraints on a query"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html)"]
pub type QueryControlFlagBits = QueryControlFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkQueryResultFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryResultFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html)"]
    pub struct QueryResultFlags: u32 {
        #[doc = "Translated from: `VK_QUERY_RESULT_64_BIT`"]
        const NUM_64 = 0b1;
        #[doc = "Translated from: `VK_QUERY_RESULT_WAIT_BIT`"]
        const WAIT = 0b10;
        #[doc = "Translated from: `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT`"]
        const WITH_AVAILABILITY = 0b100;
        #[doc = "Translated from: `VK_QUERY_RESULT_PARTIAL_BIT`"]
        const PARTIAL = 0b1000;
    }
}

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying how and when query results are returned"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html)"]
pub type QueryResultFlagBits = QueryResultFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkQueryPipelineStatisticFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryPipelineStatisticFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html)"]
    pub struct QueryPipelineStatisticFlags: u32 {
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT`"]
        const INPUT_ASSEMBLY_VERTICES = 0b1;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT`"]
        const INPUT_ASSEMBLY_PRIMITIVES = 0b10;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT`"]
        const VERTEX_SHADER_INVOCATIONS = 0b100;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT`"]
        const GEOMETRY_SHADER_INVOCATIONS = 0b1000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT`"]
        const GEOMETRY_SHADER_PRIMITIVES = 0b10000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT`"]
        const CLIPPING_INVOCATIONS = 0b100000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT`"]
        const CLIPPING_PRIMITIVES = 0b1000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT`"]
        const FRAGMENT_SHADER_INVOCATIONS = 0b10000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT`"]
        const TESSELLATION_CONTROL_SHADER_PATCHES = 0b100000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT`"]
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 0b1000000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT`"]
        const COMPUTE_SHADER_INVOCATIONS = 0b10000000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT`"]
        const TASK_SHADER_INVOCATIONS_EXT = 0b100000000000;
        #[doc = "Translated from: `VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT`"]
        const MESH_SHADER_INVOCATIONS_EXT = 0b1000000000000;
    }
}

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying queried pipeline statistics"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html)"]
pub type QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Rasterization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkCullModeFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCullModeFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html)"]
    pub struct CullModeFlags: u32 {
        #[doc = "Translated from: `VK_CULL_MODE_NONE`"]
        const NONE = 0;
        #[doc = "Translated from: `VK_CULL_MODE_FRONT_BIT`"]
        const FRONT = 0b1;
        #[doc = "Translated from: `VK_CULL_MODE_BACK_BIT`"]
        const BACK = 0b10;
        #[doc = "Translated from: `VK_CULL_MODE_FRONT_AND_BACK`"]
        const FRONT_AND_BACK = 0x3;
    }
}

#[doc = "**Chapter**: Rasterization"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask controlling triangle culling"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html)"]
pub type CullModeFlagBits = CullModeFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: The Framebuffer"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkColorComponentFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkColorComponentFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html)"]
    pub struct ColorComponentFlags: u32 {
        #[doc = "Translated from: `VK_COLOR_COMPONENT_R_BIT`"]
        const R = 0b1;
        #[doc = "Translated from: `VK_COLOR_COMPONENT_G_BIT`"]
        const G = 0b10;
        #[doc = "Translated from: `VK_COLOR_COMPONENT_B_BIT`"]
        const B = 0b100;
        #[doc = "Translated from: `VK_COLOR_COMPONENT_A_BIT`"]
        const A = 0b1000;
    }
}

#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask controlling which components are written to the framebuffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html)"]
pub type ColorComponentFlagBits = ColorComponentFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Limits"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkSampleCountFlagBits"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSampleCountFlags`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlags.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)"]
    pub struct SampleCountFlags: u32 {
        #[doc = "Translated from: `VK_SAMPLE_COUNT_1_BIT`"]
        const NUM_1 = 0b1;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_2_BIT`"]
        const NUM_2 = 0b10;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_4_BIT`"]
        const NUM_4 = 0b100;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_8_BIT`"]
        const NUM_8 = 0b1000;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_16_BIT`"]
        const NUM_16 = 0b10000;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_32_BIT`"]
        const NUM_32 = 0b100000;
        #[doc = "Translated from: `VK_SAMPLE_COUNT_64_BIT`"]
        const NUM_64 = 0b1000000;
    }
}

#[doc = "**Chapter**: Limits"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying sample counts supported for an image used for storage operations"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)"]
pub type SampleCountFlagBits = SampleCountFlags;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessengerCreateFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html)"]
    pub struct DebugUtilsMessengerCreateFlagsEXT: u32 {

    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkDebugUtilsMessageSeverityFlagBitsEXT"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessageSeverityFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagsEXT.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessageSeverityFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)"]
    pub struct DebugUtilsMessageSeverityFlagsEXT: u32 {
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT`"]
        const VERBOSE_EXT = 0b1;
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT`"]
        const INFO_EXT = 0b10000;
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT`"]
        const WARNING_EXT = 0b100000000;
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT`"]
        const ERROR_EXT = 0b1000000000000;
    }
}

#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying which severities of events cause a debug messenger callback"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsMessageSeverityFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)"]
pub type DebugUtilsMessageSeverityFlagBitsEXT = DebugUtilsMessageSeverityFlagsEXT;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bitmask of VkDebugUtilsMessageTypeFlagBitsEXT"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessageTypeFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagsEXT.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessageTypeFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)"]
    pub struct DebugUtilsMessageTypeFlagsEXT: u32 {
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT`"]
        const GENERAL_EXT = 0b1;
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT`"]
        const VALIDATION_EXT = 0b10;
        #[doc = "Translated from: `VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT`"]
        const PERFORMANCE_EXT = 0b100;
    }
}

#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Bitmask specifying which types of events cause a debug messenger callback"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsMessageTypeFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)"]
pub type DebugUtilsMessageTypeFlagBitsEXT = DebugUtilsMessageTypeFlagsEXT;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reserved for future use"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`VkDebugUtilsMessengerCallbackDataFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html)"]
    pub struct DebugUtilsMessengerCallbackDataFlagsEXT: u32 {

    }
}

//
// Structures
//

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a two-dimensional offset"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkOffset2D`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let offset_2d = vk::Offset2D {
    x: todo!("i32"),
    y: todo!("i32"),
};
```"#]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a three-dimensional offset"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkOffset3D`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let offset_3d = vk::Offset3D {
    x: todo!("i32"),
    y: todo!("i32"),
    z: todo!("i32"),
};
```"#]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a two-dimensional extent"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkExtent2D`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let extent_2d = vk::Extent2D {
    width: todo!("u32"),
    height: todo!("u32"),
};
```"#]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a three-dimensional extent"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkExtent3D`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let extent_3d = vk::Extent3D {
    width: todo!("u32"),
    height: todo!("u32"),
    depth: todo!("u32"),
};
```"#]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fundamentals"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a two-dimensional subregion"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkRect2D`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let rect_2d = vk::Rect2D {
    offset: todo!("vk::Offset2D"),
    extent: todo!("vk::Extent2D"),
};
```"#]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkInstanceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkValidationFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkDebugUtilsMessengerCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let instance_create_info = vk::InstanceCreateInfo {
    s_type: vk::StructureType::InstanceCreateInfo,
    p_next: null(),
    flags: todo!("vk::InstanceCreateFlags"),
    p_application_info: todo!("*const vk::ApplicationInfo"),
    enabled_layer_count: todo!("u32"),
    pp_enabled_layer_names: todo!("*const *const c_char"),
    enabled_extension_count: todo!("u32"),
    pp_enabled_extension_names: todo!("*const *const c_char"),
};
```"#]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Specify validation features to enable or disable for a Vulkan instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_validation_features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkValidationFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let validation_features_ext = vk::ValidationFeaturesEXT {
    s_type: vk::StructureType::ValidationFeaturesEXT,
    p_next: null(),
    enabled_validation_feature_count: todo!("u32"),
    p_enabled_validation_features: todo!("*const vk::ValidationFeatureEnableEXT"),
    disabled_validation_feature_count: todo!("u32"),
    p_disabled_validation_features: todo!("*const vk::ValidationFeatureDisableEXT"),
};
```"#]
pub struct ValidationFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying application information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkApplicationInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let application_info = vk::ApplicationInfo {
    s_type: vk::StructureType::ApplicationInfo,
    p_next: null(),
    p_application_name: todo!("*const c_char"),
    application_version: todo!("u32"),
    p_engine_name: todo!("*const c_char"),
    engine_version: todo!("u32"),
    api_version: todo!("u32"),
};
```"#]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying physical device properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_properties = vk::PhysicalDeviceProperties {
    api_version: todo!("u32"),
    driver_version: todo!("u32"),
    vendor_id: todo!("u32"),
    device_id: todo!("u32"),
    device_type: todo!("vk::PhysicalDeviceType"),
    device_name: todo!("[c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as _]"),
    pipeline_cache_uuid: todo!("[u8; UUID_SIZE as _]"),
    limits: todo!("vk::PhysicalDeviceLimits"),
    sparse_properties: todo!("vk::PhysicalDeviceSparseProperties"),
};
```"#]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as _],
    pub pipeline_cache_uuid: [u8; UUID_SIZE as _],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying physical device properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceMeshShaderPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceDescriptorBufferPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_properties2 = vk::PhysicalDeviceProperties2 {
    s_type: vk::StructureType::PhysicalDeviceProperties2,
    p_next: null_mut(),
    properties: todo!("vk::PhysicalDeviceProperties"),
};
```"#]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure providing information about a queue family"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let queue_family_properties = vk::QueueFamilyProperties {
    queue_flags: todo!("vk::QueueFlags"),
    queue_count: todo!("u32"),
    timestamp_valid_bits: todo!("u32"),
    min_image_transfer_granularity: todo!("vk::Extent3D"),
};
```"#]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure providing information about a queue family"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueueFamilyProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let queue_family_properties2 = vk::QueueFamilyProperties2 {
    s_type: vk::StructureType::QueueFamilyProperties2,
    p_next: null_mut(),
    queue_family_properties: todo!("vk::QueueFamilyProperties"),
};
```"#]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceFeatures2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan11Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan12Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan13Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceMeshShaderFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceDescriptorBufferFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceShaderObjectFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderObjectFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let device_create_info = vk::DeviceCreateInfo {
    s_type: vk::StructureType::DeviceCreateInfo,
    p_next: null(),
    flags: todo!("vk::DeviceCreateFlags"),
    queue_create_info_count: todo!("u32"),
    p_queue_create_infos: todo!("*const vk::DeviceQueueCreateInfo"),
    enabled_layer_count: todo!("u32"),
    pp_enabled_layer_names: todo!("*const *const c_char"),
    enabled_extension_count: todo!("u32"),
    pp_enabled_extension_names: todo!("*const *const c_char"),
    p_enabled_features: todo!("*const vk::PhysicalDeviceFeatures"),
};
```"#]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created device queue"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceQueueCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let device_queue_create_info = vk::DeviceQueueCreateInfo {
    s_type: vk::StructureType::DeviceQueueCreateInfo,
    p_next: null(),
    flags: todo!("vk::DeviceQueueCreateFlags"),
    queue_family_index: todo!("u32"),
    queue_count: todo!("u32"),
    p_queue_priorities: todo!("*const f32"),
};
```"#]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the parameters used for device queue creation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceQueueInfo2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let device_queue_info2 = vk::DeviceQueueInfo2 {
    s_type: vk::StructureType::DeviceQueueInfo2,
    p_next: null(),
    flags: todo!("vk::DeviceQueueCreateFlags"),
    queue_family_index: todo!("u32"),
    queue_index: todo!("u32"),
};
```"#]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created command pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let command_pool_create_info = vk::CommandPoolCreateInfo {
    s_type: vk::StructureType::CommandPoolCreateInfo,
    p_next: null(),
    flags: todo!("vk::CommandPoolCreateFlags"),
    queue_family_index: todo!("u32"),
};
```"#]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the allocation parameters for command buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
    s_type: vk::StructureType::CommandBufferAllocateInfo,
    p_next: null(),
    command_pool: todo!("vk::CommandPool"),
    level: todo!("vk::CommandBufferLevel"),
    command_buffer_count: todo!("u32"),
};
```"#]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a command buffer begin operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let command_buffer_begin_info = vk::CommandBufferBeginInfo {
    s_type: vk::StructureType::CommandBufferBeginInfo,
    p_next: null(),
    flags: todo!("vk::CommandBufferUsageFlags"),
    p_inheritance_info: todo!("*const vk::CommandBufferInheritanceInfo"),
};
```"#]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying command buffer inheritance information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferInheritanceInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let command_buffer_inheritance_info = vk::CommandBufferInheritanceInfo {
    s_type: vk::StructureType::CommandBufferInheritanceInfo,
    p_next: null(),
    render_pass: todo!("vk::RenderPass"),
    subpass: todo!("u32"),
    framebuffer: todo!("vk::Framebuffer"),
    occlusion_query_enable: todo!("vk::Bool32"),
    query_flags: todo!("vk::QueryControlFlags"),
    pipeline_statistics: todo!("vk::QueryPipelineStatisticFlags"),
};
```"#]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a queue submit operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSubmitInfo2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let submit_info2 = vk::SubmitInfo2 {
    s_type: vk::StructureType::SubmitInfo2,
    p_next: null(),
    flags: todo!("vk::SubmitFlags"),
    wait_semaphore_info_count: todo!("u32"),
    p_wait_semaphore_infos: todo!("*const vk::SemaphoreSubmitInfo"),
    command_buffer_info_count: todo!("u32"),
    p_command_buffer_infos: todo!("*const vk::CommandBufferSubmitInfo"),
    signal_semaphore_info_count: todo!("u32"),
    p_signal_semaphore_infos: todo!("*const vk::SemaphoreSubmitInfo"),
};
```"#]
pub struct SubmitInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubmitFlags,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const CommandBufferSubmitInfo,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a semaphore signal or wait operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let semaphore_submit_info = vk::SemaphoreSubmitInfo {
    s_type: vk::StructureType::SemaphoreSubmitInfo,
    p_next: null(),
    semaphore: todo!("vk::Semaphore"),
    value: todo!("u64"),
    stage_mask: todo!("vk::PipelineStageFlags2"),
    device_index: todo!("u32"),
};
```"#]
pub struct SemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2,
    pub device_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a command buffer submission"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCommandBufferSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let command_buffer_submit_info = vk::CommandBufferSubmitInfo {
    s_type: vk::StructureType::CommandBufferSubmitInfo,
    p_next: null(),
    command_buffer: todo!("vk::CommandBuffer"),
    device_mask: todo!("u32"),
};
```"#]
pub struct CommandBufferSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_buffer: CommandBuffer,
    pub device_mask: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created semaphore"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkSemaphoreTypeCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let semaphore_create_info = vk::SemaphoreCreateInfo {
    s_type: vk::StructureType::SemaphoreCreateInfo,
    p_next: null(),
    flags: todo!("vk::SemaphoreCreateFlags"),
};
```"#]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the type of a newly created semaphore"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreTypeCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let semaphore_type_create_info = vk::SemaphoreTypeCreateInfo {
    s_type: vk::StructureType::SemaphoreTypeCreateInfo,
    p_next: null(),
    semaphore_type: todo!("vk::SemaphoreType"),
    initial_value: todo!("u64"),
};
```"#]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure containing information about the semaphore wait condition"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSemaphoreWaitInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let semaphore_wait_info = vk::SemaphoreWaitInfo {
    s_type: vk::StructureType::SemaphoreWaitInfo,
    p_next: null(),
    flags: todo!("vk::SemaphoreWaitFlags"),
    semaphore_count: todo!("u32"),
    p_semaphores: todo!("*const vk::Semaphore"),
    p_values: todo!("*const u64"),
};
```"#]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const Semaphore,
    pub p_values: *const u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying dependency information for a synchronization command"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDependencyInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let dependency_info = vk::DependencyInfo {
    s_type: vk::StructureType::DependencyInfo,
    p_next: null(),
    dependency_flags: todo!("vk::DependencyFlags"),
    memory_barrier_count: todo!("u32"),
    p_memory_barriers: todo!("*const vk::MemoryBarrier2"),
    buffer_memory_barrier_count: todo!("u32"),
    p_buffer_memory_barriers: todo!("*const vk::BufferMemoryBarrier2"),
    image_memory_barrier_count: todo!("u32"),
    p_image_memory_barriers: todo!("*const vk::ImageMemoryBarrier2"),
};
```"#]
pub struct DependencyInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dependency_flags: DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const MemoryBarrier2,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const BufferMemoryBarrier2,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const ImageMemoryBarrier2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a global memory barrier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryBarrier2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_barrier2 = vk::MemoryBarrier2 {
    s_type: vk::StructureType::MemoryBarrier2,
    p_next: null(),
    src_stage_mask: todo!("vk::PipelineStageFlags2"),
    src_access_mask: todo!("vk::AccessFlags2"),
    dst_stage_mask: todo!("vk::PipelineStageFlags2"),
    dst_access_mask: todo!("vk::AccessFlags2"),
};
```"#]
pub struct MemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a buffer memory barrier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferMemoryBarrier2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let buffer_memory_barrier2 = vk::BufferMemoryBarrier2 {
    s_type: vk::StructureType::BufferMemoryBarrier2,
    p_next: null(),
    src_stage_mask: todo!("vk::PipelineStageFlags2"),
    src_access_mask: todo!("vk::AccessFlags2"),
    dst_stage_mask: todo!("vk::PipelineStageFlags2"),
    dst_access_mask: todo!("vk::AccessFlags2"),
    src_queue_family_index: todo!("u32"),
    dst_queue_family_index: todo!("u32"),
    buffer: todo!("vk::Buffer"),
    offset: todo!("vk::DeviceSize"),
    size: todo!("vk::DeviceSize"),
};
```"#]
pub struct BufferMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying an image memory barrier"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageMemoryBarrier2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let image_memory_barrier2 = vk::ImageMemoryBarrier2 {
    s_type: vk::StructureType::ImageMemoryBarrier2,
    p_next: null(),
    src_stage_mask: todo!("vk::PipelineStageFlags2"),
    src_access_mask: todo!("vk::AccessFlags2"),
    dst_stage_mask: todo!("vk::PipelineStageFlags2"),
    dst_access_mask: todo!("vk::AccessFlags2"),
    old_layout: todo!("vk::ImageLayout"),
    new_layout: todo!("vk::ImageLayout"),
    src_queue_family_index: todo!("u32"),
    dst_queue_family_index: todo!("u32"),
    image: todo!("vk::Image"),
    subresource_range: todo!("vk::ImageSubresourceRange"),
};
```"#]
pub struct ImageMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the input parameters of a calibrated timestamp query"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCalibratedTimestampInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let calibrated_timestamp_info_ext = vk::CalibratedTimestampInfoEXT {
    s_type: vk::StructureType::CalibratedTimestampInfoEXT,
    p_next: null(),
    time_domain: todo!("vk::TimeDomainEXT"),
};
```"#]
pub struct CalibratedTimestampInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub time_domain: TimeDomainEXT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying render pass instance begin info"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkRenderingInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let rendering_info = vk::RenderingInfo {
    s_type: vk::StructureType::RenderingInfo,
    p_next: null(),
    flags: todo!("vk::RenderingFlags"),
    render_area: todo!("vk::Rect2D"),
    layer_count: todo!("u32"),
    view_mask: todo!("u32"),
    color_attachment_count: todo!("u32"),
    p_color_attachments: todo!("*const vk::RenderingAttachmentInfo"),
    p_depth_attachment: todo!("*const vk::RenderingAttachmentInfo"),
    p_stencil_attachment: todo!("*const vk::RenderingAttachmentInfo"),
};
```"#]
pub struct RenderingInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingFlags,
    pub render_area: Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const RenderingAttachmentInfo,
    pub p_depth_attachment: *const RenderingAttachmentInfo,
    pub p_stencil_attachment: *const RenderingAttachmentInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying attachment information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkRenderingAttachmentInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let rendering_attachment_info = vk::RenderingAttachmentInfo {
    s_type: vk::StructureType::RenderingAttachmentInfo,
    p_next: null(),
    image_view: todo!("vk::ImageView"),
    image_layout: todo!("vk::ImageLayout"),
    resolve_mode: todo!("vk::ResolveModeFlagBits"),
    resolve_image_view: todo!("vk::ImageView"),
    resolve_image_layout: todo!("vk::ImageLayout"),
    load_op: todo!("vk::AttachmentLoadOp"),
    store_op: todo!("vk::AttachmentStoreOp"),
    clear_value: todo!("vk::ClearValue"),
};
```"#]
pub struct RenderingAttachmentInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub resolve_mode: ResolveModeFlagBits,
    pub resolve_image_view: ImageView,
    pub resolve_image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub clear_value: ClearValue,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created shader"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkShaderCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCreateInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let shader_create_info_ext = vk::ShaderCreateInfoEXT {
    s_type: vk::StructureType::ShaderCreateInfoEXT,
    p_next: null(),
    flags: todo!("vk::ShaderCreateFlagsEXT"),
    stage: todo!("vk::ShaderStageFlagBits"),
    next_stage: todo!("vk::ShaderStageFlags"),
    code_type: todo!("vk::ShaderCodeTypeEXT"),
    code_size: todo!("usize"),
    p_code: todo!("*const c_void"),
    p_name: todo!("*const c_char"),
    set_layout_count: todo!("u32"),
    p_set_layouts: todo!("*const vk::DescriptorSetLayout"),
    push_constant_range_count: todo!("u32"),
    p_push_constant_ranges: todo!("*const vk::PushConstantRange"),
    p_specialization_info: todo!("*const vk::SpecializationInfo"),
};
```"#]
pub struct ShaderCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderCreateFlagsEXT,
    pub stage: ShaderStageFlagBits,
    pub next_stage: ShaderStageFlags,
    pub code_type: ShaderCodeTypeEXT,
    pub code_size: usize,
    pub p_code: *const c_void,
    pub p_name: *const c_char,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub p_specialization_info: *const SpecializationInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Pipelines"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying specialization information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSpecializationInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let specialization_info = vk::SpecializationInfo {
    map_entry_count: todo!("u32"),
    p_map_entries: todo!("*const vk::SpecializationMapEntry"),
    data_size: todo!("usize"),
    p_data: todo!("*const c_void"),
};
```"#]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Pipelines"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a specialization map entry"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSpecializationMapEntry`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let specialization_map_entry = vk::SpecializationMapEntry {
    constant_id: todo!("u32"),
    offset: todo!("u32"),
    size: todo!("usize"),
};
```"#]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure containing callback function pointers for memory allocation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkAllocationCallbacks`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let allocation_callbacks = vk::AllocationCallbacks {
    p_user_data: todo!("*mut c_void"),
    pfn_allocation: todo!("vk::PfnAllocationFunction"),
    pfn_reallocation: todo!("vk::PfnReallocationFunction"),
    pfn_free: todo!("vk::PfnFreeFunction"),
    pfn_internal_allocation: todo!("vk::PfnInternalAllocationNotification"),
    pfn_internal_free: todo!("vk::PfnInternalFreeNotification"),
};
```"#]
pub struct AllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PfnAllocationFunction,
    pub pfn_reallocation: PfnReallocationFunction,
    pub pfn_free: PfnFreeFunction,
    pub pfn_internal_allocation: PfnInternalAllocationNotification,
    pub pfn_internal_free: PfnInternalFreeNotification,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying physical device memory properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_memory_properties = vk::PhysicalDeviceMemoryProperties {
    memory_type_count: todo!("u32"),
    memory_types: todo!("[vk::MemoryType; MAX_MEMORY_TYPES as _]"),
    memory_heap_count: todo!("u32"),
    memory_heaps: todo!("[vk::MemoryHeap; MAX_MEMORY_HEAPS as _]"),
};
```"#]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES as _],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as _],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying physical device memory properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceMemoryProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_memory_properties2 = vk::PhysicalDeviceMemoryProperties2 {
    s_type: vk::StructureType::PhysicalDeviceMemoryProperties2,
    p_next: null_mut(),
    memory_properties: todo!("vk::PhysicalDeviceMemoryProperties"),
};
```"#]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a memory heap"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryHeap`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_heap = vk::MemoryHeap {
    size: todo!("vk::DeviceSize"),
    flags: todo!("vk::MemoryHeapFlags"),
};
```"#]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying memory type"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryType`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_type = vk::MemoryType {
    property_flags: todo!("vk::MemoryPropertyFlags"),
    heap_index: todo!("u32"),
};
```"#]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure containing parameters of a memory allocation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkMemoryAllocateFlagsInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_allocate_info = vk::MemoryAllocateInfo {
    s_type: vk::StructureType::MemoryAllocateInfo,
    p_next: null(),
    allocation_size: todo!("vk::DeviceSize"),
    memory_type_index: todo!("u32"),
};
```"#]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure controlling how many instances of memory will be allocated"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryAllocateFlagsInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_allocate_flags_info = vk::MemoryAllocateFlagsInfo {
    s_type: vk::StructureType::MemoryAllocateFlagsInfo,
    p_next: null(),
    flags: todo!("vk::MemoryAllocateFlags"),
    device_mask: todo!("u32"),
};
```"#]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure containing parameters of a memory map operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryMapInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryMapInfoKHR.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_map_info_khr = vk::MemoryMapInfoKHR {
    s_type: vk::StructureType::MemoryMapInfoKHR,
    p_next: null(),
    flags: todo!("vk::MemoryMapFlags"),
    memory: todo!("vk::DeviceMemory"),
    offset: todo!("vk::DeviceSize"),
    size: todo!("vk::DeviceSize"),
};
```"#]
pub struct MemoryMapInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryMapFlags,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure containing parameters of a memory unmap operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryUnmapInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryUnmapInfoKHR.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_unmap_info_khr = vk::MemoryUnmapInfoKHR {
    s_type: vk::StructureType::MemoryUnmapInfoKHR,
    p_next: null(),
    flags: todo!("vk::MemoryUnmapFlagsKHR"),
    memory: todo!("vk::DeviceMemory"),
};
```"#]
pub struct MemoryUnmapInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryUnmapFlagsKHR,
    pub memory: DeviceMemory,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the parameters of a newly created buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let buffer_create_info = vk::BufferCreateInfo {
    s_type: vk::StructureType::BufferCreateInfo,
    p_next: null(),
    flags: todo!("vk::BufferCreateFlags"),
    size: todo!("vk::DeviceSize"),
    usage: todo!("vk::BufferUsageFlags"),
    sharing_mode: todo!("vk::SharingMode"),
    queue_family_index_count: todo!("u32"),
    p_queue_family_indices: todo!("*const u32"),
};
```"#]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the parameters of a newly created image object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let image_create_info = vk::ImageCreateInfo {
    s_type: vk::StructureType::ImageCreateInfo,
    p_next: null(),
    flags: todo!("vk::ImageCreateFlags"),
    image_type: todo!("vk::ImageType"),
    format: todo!("vk::Format"),
    extent: todo!("vk::Extent3D"),
    mip_levels: todo!("u32"),
    array_layers: todo!("u32"),
    samples: todo!("vk::SampleCountFlagBits"),
    tiling: todo!("vk::ImageTiling"),
    usage: todo!("vk::ImageUsageFlags"),
    sharing_mode: todo!("vk::SharingMode"),
    queue_family_index_count: todo!("u32"),
    p_queue_family_indices: todo!("*const u32"),
    initial_layout: todo!("vk::ImageLayout"),
};
```"#]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created image view"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let image_view_create_info = vk::ImageViewCreateInfo {
    s_type: vk::StructureType::ImageViewCreateInfo,
    p_next: null(),
    flags: todo!("vk::ImageViewCreateFlags"),
    image: todo!("vk::Image"),
    view_type: todo!("vk::ImageViewType"),
    format: todo!("vk::Format"),
    components: todo!("vk::ComponentMapping"),
    subresource_range: todo!("vk::ImageSubresourceRange"),
};
```"#]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying an image subresource range"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageSubresourceRange`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let image_subresource_range = vk::ImageSubresourceRange {
    aspect_mask: todo!("vk::ImageAspectFlags"),
    base_mip_level: todo!("u32"),
    level_count: todo!("u32"),
    base_array_layer: todo!("u32"),
    layer_count: todo!("u32"),
};
```"#]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a color component mapping"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkComponentMapping`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let component_mapping = vk::ComponentMapping {
    r: todo!("vk::ComponentSwizzle"),
    g: todo!("vk::ComponentSwizzle"),
    b: todo!("vk::ComponentSwizzle"),
    a: todo!("vk::ComponentSwizzle"),
};
```"#]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying memory requirements"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_requirements = vk::MemoryRequirements {
    size: todo!("vk::DeviceSize"),
    alignment: todo!("vk::DeviceSize"),
    memory_type_bits: todo!("u32"),
};
```"#]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: (None)"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let device_buffer_memory_requirements = vk::DeviceBufferMemoryRequirements {
    s_type: vk::StructureType::DeviceBufferMemoryRequirements,
    p_next: null(),
    p_create_info: todo!("*const vk::BufferCreateInfo"),
};
```"#]
pub struct DeviceBufferMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const BufferCreateInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: (None)"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDeviceImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let device_image_memory_requirements = vk::DeviceImageMemoryRequirements {
    s_type: vk::StructureType::DeviceImageMemoryRequirements,
    p_next: null(),
    p_create_info: todo!("*const vk::ImageCreateInfo"),
    plane_aspect: todo!("vk::ImageAspectFlagBits"),
};
```"#]
pub struct DeviceImageMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const ImageCreateInfo,
    pub plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying memory requirements"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkMemoryRequirements2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let memory_requirements2 = vk::MemoryRequirements2 {
    s_type: vk::StructureType::MemoryRequirements2,
    p_next: null_mut(),
    memory_requirements: todo!("vk::MemoryRequirements"),
};
```"#]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying how to bind a buffer to memory"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBindBufferMemoryInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let bind_buffer_memory_info = vk::BindBufferMemoryInfo {
    s_type: vk::StructureType::BindBufferMemoryInfo,
    p_next: null(),
    buffer: todo!("vk::Buffer"),
    memory: todo!("vk::DeviceMemory"),
    memory_offset: todo!("vk::DeviceSize"),
};
```"#]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying how to bind an image to memory"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBindImageMemoryInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let bind_image_memory_info = vk::BindImageMemoryInfo {
    s_type: vk::StructureType::BindImageMemoryInfo,
    p_next: null(),
    image: todo!("vk::Image"),
    memory: todo!("vk::DeviceMemory"),
    memory_offset: todo!("vk::DeviceSize"),
};
```"#]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created sampler"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkSamplerCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let sampler_create_info = vk::SamplerCreateInfo {
    s_type: vk::StructureType::SamplerCreateInfo,
    p_next: null(),
    flags: todo!("vk::SamplerCreateFlags"),
    mag_filter: todo!("vk::Filter"),
    min_filter: todo!("vk::Filter"),
    mipmap_mode: todo!("vk::SamplerMipmapMode"),
    address_mode_u: todo!("vk::SamplerAddressMode"),
    address_mode_v: todo!("vk::SamplerAddressMode"),
    address_mode_w: todo!("vk::SamplerAddressMode"),
    mip_lod_bias: todo!("f32"),
    anisotropy_enable: todo!("vk::Bool32"),
    max_anisotropy: todo!("f32"),
    compare_enable: todo!("vk::Bool32"),
    compare_op: todo!("vk::CompareOp"),
    min_lod: todo!("f32"),
    max_lod: todo!("f32"),
    border_color: todo!("vk::BorderColor"),
    unnormalized_coordinates: todo!("vk::Bool32"),
};
```"#]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created descriptor set layout"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorSetLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
    s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
    p_next: null(),
    flags: todo!("vk::DescriptorSetLayoutCreateFlags"),
    binding_count: todo!("u32"),
    p_bindings: todo!("*const vk::DescriptorSetLayoutBinding"),
};
```"#]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a descriptor set layout binding"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorSetLayoutBinding`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding {
    binding: todo!("u32"),
    descriptor_type: todo!("vk::DescriptorType"),
    descriptor_count: todo!("u32"),
    stage_flags: todo!("vk::ShaderStageFlags"),
    p_immutable_samplers: todo!("*const vk::Sampler"),
};
```"#]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the parameters of a newly created pipeline layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPipelineLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
    s_type: vk::StructureType::PipelineLayoutCreateInfo,
    p_next: null(),
    flags: todo!("vk::PipelineLayoutCreateFlags"),
    set_layout_count: todo!("u32"),
    p_set_layouts: todo!("*const vk::DescriptorSetLayout"),
    push_constant_range_count: todo!("u32"),
    p_push_constant_ranges: todo!("*const vk::PushConstantRange"),
};
```"#]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a push constant range"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPushConstantRange`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let push_constant_range = vk::PushConstantRange {
    stage_flags: todo!("vk::ShaderStageFlags"),
    offset: todo!("u32"),
    size: todo!("u32"),
};
```"#]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying descriptor image information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorImageInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_image_info = vk::DescriptorImageInfo {
    sampler: todo!("vk::Sampler"),
    image_view: todo!("vk::ImageView"),
    image_layout: todo!("vk::ImageLayout"),
};
```"#]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the buffer to query an address for"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferDeviceAddressInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let buffer_device_address_info = vk::BufferDeviceAddressInfo {
    s_type: vk::StructureType::BufferDeviceAddressInfo,
    p_next: null(),
    buffer: todo!("vk::Buffer"),
};
```"#]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of descriptor to get"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorGetInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorGetInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_get_info_ext = vk::DescriptorGetInfoEXT {
    s_type: vk::StructureType::DescriptorGetInfoEXT,
    p_next: null(),
    ty: todo!("vk::DescriptorType"),
    data: todo!("vk::DescriptorDataEXT"),
};
```"#]
pub struct DescriptorGetInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: DescriptorType,
    pub data: DescriptorDataEXT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying descriptor buffer address info"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorAddressInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorAddressInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_address_info_ext = vk::DescriptorAddressInfoEXT {
    s_type: vk::StructureType::DescriptorAddressInfoEXT,
    p_next: null_mut(),
    address: todo!("vk::DeviceAddress"),
    range: todo!("vk::DeviceSize"),
    format: todo!("vk::Format"),
};
```"#]
pub struct DescriptorAddressInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address: DeviceAddress,
    pub range: DeviceSize,
    pub format: Format,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying descriptor buffer binding information"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorBufferBindingInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferBindingInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let descriptor_buffer_binding_info_ext = vk::DescriptorBufferBindingInfoEXT {
    s_type: vk::StructureType::DescriptorBufferBindingInfoEXT,
    p_next: null_mut(),
    address: todo!("vk::DeviceAddress"),
    usage: todo!("vk::BufferUsageFlags"),
};
```"#]
pub struct DescriptorBufferBindingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address: DeviceAddress,
    pub usage: BufferUsageFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created query pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkQueryPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let query_pool_create_info = vk::QueryPoolCreateInfo {
    s_type: vk::StructureType::QueryPoolCreateInfo,
    p_next: null(),
    flags: todo!("vk::QueryPoolCreateFlags"),
    query_type: todo!("vk::QueryType"),
    query_count: todo!("u32"),
    pipeline_statistics: todo!("vk::QueryPipelineStatisticFlags"),
};
```"#]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Clear Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a clear depth stencil value"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkClearDepthStencilValue`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let clear_depth_stencil_value = vk::ClearDepthStencilValue {
    depth: todo!("f32"),
    stencil: todo!("u32"),
};
```"#]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying an image subresource layers"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkImageSubresourceLayers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let image_subresource_layers = vk::ImageSubresourceLayers {
    aspect_mask: todo!("vk::ImageAspectFlags"),
    mip_level: todo!("u32"),
    base_array_layer: todo!("u32"),
    layer_count: todo!("u32"),
};
```"#]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a buffer to image copy command"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCopyBufferToImageInfo2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let copy_buffer_to_image_info2 = vk::CopyBufferToImageInfo2 {
    s_type: vk::StructureType::CopyBufferToImageInfo2,
    p_next: null(),
    src_buffer: todo!("vk::Buffer"),
    dst_image: todo!("vk::Image"),
    dst_image_layout: todo!("vk::ImageLayout"),
    region_count: todo!("u32"),
    p_regions: todo!("*const vk::BufferImageCopy2"),
};
```"#]
pub struct CopyBufferToImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_buffer: Buffer,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of an image to buffer copy command"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkCopyImageToBufferInfo2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let copy_image_to_buffer_info2 = vk::CopyImageToBufferInfo2 {
    s_type: vk::StructureType::CopyImageToBufferInfo2,
    p_next: null(),
    src_image: todo!("vk::Image"),
    src_image_layout: todo!("vk::ImageLayout"),
    dst_buffer: todo!("vk::Buffer"),
    region_count: todo!("u32"),
    p_regions: todo!("*const vk::BufferImageCopy2"),
};
```"#]
pub struct CopyImageToBufferInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_buffer: Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a buffer image copy operation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkBufferImageCopy2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let buffer_image_copy2 = vk::BufferImageCopy2 {
    s_type: vk::StructureType::BufferImageCopy2,
    p_next: null(),
    buffer_offset: todo!("vk::DeviceSize"),
    buffer_row_length: todo!("u32"),
    buffer_image_height: todo!("u32"),
    image_subresource: todo!("vk::ImageSubresourceLayers"),
    image_offset: todo!("vk::Offset3D"),
    image_extent: todo!("vk::Extent3D"),
};
```"#]
pub struct BufferImageCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Fixed-Function Vertex Post-Processing"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a viewport"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkViewport`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewport.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let viewport = vk::Viewport {
    x: todo!("f32"),
    y: todo!("f32"),
    width: todo!("f32"),
    height: todo!("f32"),
    min_depth: todo!("f32"),
    max_depth: todo!("f32"),
};
```"#]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying the color blend factors and operations for an attachment"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkColorBlendEquationEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorBlendEquationEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let color_blend_equation_ext = vk::ColorBlendEquationEXT {
    src_color_blend_factor: todo!("vk::BlendFactor"),
    dst_color_blend_factor: todo!("vk::BlendFactor"),
    color_blend_op: todo!("vk::BlendOp"),
    src_alpha_blend_factor: todo!("vk::BlendFactor"),
    dst_alpha_blend_factor: todo!("vk::BlendFactor"),
    alpha_blend_op: todo!("vk::BlendOp"),
};
```"#]
pub struct ColorBlendEquationEXT {
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Sparse Resources"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying physical device sparse memory properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceSparseProperties`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_sparse_properties = vk::PhysicalDeviceSparseProperties {
    residency_standard_2d_block_shape: todo!("vk::Bool32"),
    residency_standard_2d_multisample_block_shape: todo!("vk::Bool32"),
    residency_standard_3d_block_shape: todo!("vk::Bool32"),
    residency_aligned_mip_size: todo!("vk::Bool32"),
    residency_non_resident_strict: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the fine-grained features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceFeatures2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan11Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan12Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceVulkan13Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceMeshShaderFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceDescriptorBufferFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Extendable by**: [`VkPhysicalDeviceShaderObjectFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderObjectFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_features2 = vk::PhysicalDeviceFeatures2 {
    s_type: vk::StructureType::PhysicalDeviceFeatures2,
    p_next: null_mut(),
    features: todo!("vk::PhysicalDeviceFeatures"),
};
```"#]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the fine-grained features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_features = vk::PhysicalDeviceFeatures {
    robust_buffer_access: todo!("vk::Bool32"),
    full_draw_index_uint32: todo!("vk::Bool32"),
    image_cube_array: todo!("vk::Bool32"),
    independent_blend: todo!("vk::Bool32"),
    geometry_shader: todo!("vk::Bool32"),
    tessellation_shader: todo!("vk::Bool32"),
    sample_rate_shading: todo!("vk::Bool32"),
    dual_src_blend: todo!("vk::Bool32"),
    logic_op: todo!("vk::Bool32"),
    multi_draw_indirect: todo!("vk::Bool32"),
    draw_indirect_first_instance: todo!("vk::Bool32"),
    depth_clamp: todo!("vk::Bool32"),
    depth_bias_clamp: todo!("vk::Bool32"),
    fill_mode_non_solid: todo!("vk::Bool32"),
    depth_bounds: todo!("vk::Bool32"),
    wide_lines: todo!("vk::Bool32"),
    large_points: todo!("vk::Bool32"),
    alpha_to_one: todo!("vk::Bool32"),
    multi_viewport: todo!("vk::Bool32"),
    sampler_anisotropy: todo!("vk::Bool32"),
    texture_compression_etc2: todo!("vk::Bool32"),
    texture_compression_astc_ldr: todo!("vk::Bool32"),
    texture_compression_bc: todo!("vk::Bool32"),
    occlusion_query_precise: todo!("vk::Bool32"),
    pipeline_statistics_query: todo!("vk::Bool32"),
    vertex_pipeline_stores_and_atomics: todo!("vk::Bool32"),
    fragment_stores_and_atomics: todo!("vk::Bool32"),
    shader_tessellation_and_geometry_point_size: todo!("vk::Bool32"),
    shader_image_gather_extended: todo!("vk::Bool32"),
    shader_storage_image_extended_formats: todo!("vk::Bool32"),
    shader_storage_image_multisample: todo!("vk::Bool32"),
    shader_storage_image_read_without_format: todo!("vk::Bool32"),
    shader_storage_image_write_without_format: todo!("vk::Bool32"),
    shader_uniform_buffer_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_sampled_image_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_storage_buffer_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_storage_image_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_clip_distance: todo!("vk::Bool32"),
    shader_cull_distance: todo!("vk::Bool32"),
    shader_float64: todo!("vk::Bool32"),
    shader_int64: todo!("vk::Bool32"),
    shader_int16: todo!("vk::Bool32"),
    shader_resource_residency: todo!("vk::Bool32"),
    shader_resource_min_lod: todo!("vk::Bool32"),
    sparse_binding: todo!("vk::Bool32"),
    sparse_residency_buffer: todo!("vk::Bool32"),
    sparse_residency_image_2d: todo!("vk::Bool32"),
    sparse_residency_image_3d: todo!("vk::Bool32"),
    sparse_residency2_samples: todo!("vk::Bool32"),
    sparse_residency4_samples: todo!("vk::Bool32"),
    sparse_residency8_samples: todo!("vk::Bool32"),
    sparse_residency16_samples: todo!("vk::Bool32"),
    sparse_residency_aliased: todo!("vk::Bool32"),
    variable_multisample_rate: todo!("vk::Bool32"),
    inherited_queries: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image_2d: Bool32,
    pub sparse_residency_image_3d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the Vulkan 1.1 features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceVulkan11Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_vulkan11_features = vk::PhysicalDeviceVulkan11Features {
    s_type: vk::StructureType::PhysicalDeviceVulkan11Features,
    p_next: null_mut(),
    storage_buffer16_bit_access: todo!("vk::Bool32"),
    uniform_and_storage_buffer16_bit_access: todo!("vk::Bool32"),
    storage_push_constant16: todo!("vk::Bool32"),
    storage_input_output16: todo!("vk::Bool32"),
    multiview: todo!("vk::Bool32"),
    multiview_geometry_shader: todo!("vk::Bool32"),
    multiview_tessellation_shader: todo!("vk::Bool32"),
    variable_pointers_storage_buffer: todo!("vk::Bool32"),
    variable_pointers: todo!("vk::Bool32"),
    protected_memory: todo!("vk::Bool32"),
    sampler_ycbcr_conversion: todo!("vk::Bool32"),
    shader_draw_parameters: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub protected_memory: Bool32,
    pub sampler_ycbcr_conversion: Bool32,
    pub shader_draw_parameters: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the Vulkan 1.2 features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceVulkan12Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_vulkan12_features = vk::PhysicalDeviceVulkan12Features {
    s_type: vk::StructureType::PhysicalDeviceVulkan12Features,
    p_next: null_mut(),
    sampler_mirror_clamp_to_edge: todo!("vk::Bool32"),
    draw_indirect_count: todo!("vk::Bool32"),
    storage_buffer8_bit_access: todo!("vk::Bool32"),
    uniform_and_storage_buffer8_bit_access: todo!("vk::Bool32"),
    storage_push_constant8: todo!("vk::Bool32"),
    shader_buffer_int64_atomics: todo!("vk::Bool32"),
    shader_shared_int64_atomics: todo!("vk::Bool32"),
    shader_float16: todo!("vk::Bool32"),
    shader_int8: todo!("vk::Bool32"),
    descriptor_indexing: todo!("vk::Bool32"),
    shader_input_attachment_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_uniform_texel_buffer_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_storage_texel_buffer_array_dynamic_indexing: todo!("vk::Bool32"),
    shader_uniform_buffer_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_sampled_image_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_storage_buffer_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_storage_image_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_input_attachment_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_uniform_texel_buffer_array_non_uniform_indexing: todo!("vk::Bool32"),
    shader_storage_texel_buffer_array_non_uniform_indexing: todo!("vk::Bool32"),
    descriptor_binding_uniform_buffer_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_sampled_image_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_storage_image_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_storage_buffer_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_uniform_texel_buffer_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_storage_texel_buffer_update_after_bind: todo!("vk::Bool32"),
    descriptor_binding_update_unused_while_pending: todo!("vk::Bool32"),
    descriptor_binding_partially_bound: todo!("vk::Bool32"),
    descriptor_binding_variable_descriptor_count: todo!("vk::Bool32"),
    runtime_descriptor_array: todo!("vk::Bool32"),
    sampler_filter_minmax: todo!("vk::Bool32"),
    scalar_block_layout: todo!("vk::Bool32"),
    imageless_framebuffer: todo!("vk::Bool32"),
    uniform_buffer_standard_layout: todo!("vk::Bool32"),
    shader_subgroup_extended_types: todo!("vk::Bool32"),
    separate_depth_stencil_layouts: todo!("vk::Bool32"),
    host_query_reset: todo!("vk::Bool32"),
    timeline_semaphore: todo!("vk::Bool32"),
    buffer_device_address: todo!("vk::Bool32"),
    buffer_device_address_capture_replay: todo!("vk::Bool32"),
    buffer_device_address_multi_device: todo!("vk::Bool32"),
    vulkan_memory_model: todo!("vk::Bool32"),
    vulkan_memory_model_device_scope: todo!("vk::Bool32"),
    vulkan_memory_model_availability_visibility_chains: todo!("vk::Bool32"),
    shader_output_viewport_index: todo!("vk::Bool32"),
    shader_output_layer: todo!("vk::Bool32"),
    subgroup_broadcast_dynamic_id: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
    pub descriptor_indexing: Bool32,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the Vulkan 1.3 features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceVulkan13Features`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_vulkan13_features = vk::PhysicalDeviceVulkan13Features {
    s_type: vk::StructureType::PhysicalDeviceVulkan13Features,
    p_next: null_mut(),
    robust_image_access: todo!("vk::Bool32"),
    inline_uniform_block: todo!("vk::Bool32"),
    descriptor_binding_inline_uniform_block_update_after_bind: todo!("vk::Bool32"),
    pipeline_creation_cache_control: todo!("vk::Bool32"),
    private_data: todo!("vk::Bool32"),
    shader_demote_to_helper_invocation: todo!("vk::Bool32"),
    shader_terminate_invocation: todo!("vk::Bool32"),
    subgroup_size_control: todo!("vk::Bool32"),
    compute_full_subgroups: todo!("vk::Bool32"),
    synchronization2: todo!("vk::Bool32"),
    texture_compression_astc_hdr: todo!("vk::Bool32"),
    shader_zero_initialize_workgroup_memory: todo!("vk::Bool32"),
    dynamic_rendering: todo!("vk::Bool32"),
    shader_integer_dot_product: todo!("vk::Bool32"),
    maintenance4: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceVulkan13Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_image_access: Bool32,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    pub pipeline_creation_cache_control: Bool32,
    pub private_data: Bool32,
    pub shader_demote_to_helper_invocation: Bool32,
    pub shader_terminate_invocation: Bool32,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
    pub synchronization2: Bool32,
    pub texture_compression_astc_hdr: Bool32,
    pub shader_zero_initialize_workgroup_memory: Bool32,
    pub dynamic_rendering: Bool32,
    pub shader_integer_dot_product: Bool32,
    pub maintenance4: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing mesh shading features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceMeshShaderFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_mesh_shader_features_ext = vk::PhysicalDeviceMeshShaderFeaturesEXT {
    s_type: vk::StructureType::PhysicalDeviceMeshShaderFeaturesEXT,
    p_next: null_mut(),
    task_shader: todo!("vk::Bool32"),
    mesh_shader: todo!("vk::Bool32"),
    multiview_mesh_shader: todo!("vk::Bool32"),
    primitive_fragment_shading_rate_mesh_shader: todo!("vk::Bool32"),
    mesh_shader_queries: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceMeshShaderFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
    pub multiview_mesh_shader: Bool32,
    pub primitive_fragment_shading_rate_mesh_shader: Bool32,
    pub mesh_shader_queries: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing the descriptor buffer features that can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceDescriptorBufferFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_descriptor_buffer_features_ext = vk::PhysicalDeviceDescriptorBufferFeaturesEXT {
    s_type: vk::StructureType::PhysicalDeviceDescriptorBufferFeaturesEXT,
    p_next: null_mut(),
    descriptor_buffer: todo!("vk::Bool32"),
    descriptor_buffer_capture_replay: todo!("vk::Bool32"),
    descriptor_buffer_image_layout_ignored: todo!("vk::Bool32"),
    descriptor_buffer_push_descriptors: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceDescriptorBufferFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer: Bool32,
    pub descriptor_buffer_capture_replay: Bool32,
    pub descriptor_buffer_image_layout_ignored: Bool32,
    pub descriptor_buffer_push_descriptors: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Features"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing whether shader objects can be supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceShaderObjectFeaturesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderObjectFeaturesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_shader_object_features_ext = vk::PhysicalDeviceShaderObjectFeaturesEXT {
    s_type: vk::StructureType::PhysicalDeviceShaderObjectFeaturesEXT,
    p_next: null_mut(),
    shader_object: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceShaderObjectFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_object: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Limits"]
#[doc = "<br>"]
#[doc = "**Description**: Structure reporting implementation-dependent physical device limits"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceLimits`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_limits = vk::PhysicalDeviceLimits {
    max_image_dimension_1d: todo!("u32"),
    max_image_dimension_2d: todo!("u32"),
    max_image_dimension_3d: todo!("u32"),
    max_image_dimension_cube: todo!("u32"),
    max_image_array_layers: todo!("u32"),
    max_texel_buffer_elements: todo!("u32"),
    max_uniform_buffer_range: todo!("u32"),
    max_storage_buffer_range: todo!("u32"),
    max_push_constants_size: todo!("u32"),
    max_memory_allocation_count: todo!("u32"),
    max_sampler_allocation_count: todo!("u32"),
    buffer_image_granularity: todo!("vk::DeviceSize"),
    sparse_address_space_size: todo!("vk::DeviceSize"),
    max_bound_descriptor_sets: todo!("u32"),
    max_per_stage_descriptor_samplers: todo!("u32"),
    max_per_stage_descriptor_uniform_buffers: todo!("u32"),
    max_per_stage_descriptor_storage_buffers: todo!("u32"),
    max_per_stage_descriptor_sampled_images: todo!("u32"),
    max_per_stage_descriptor_storage_images: todo!("u32"),
    max_per_stage_descriptor_input_attachments: todo!("u32"),
    max_per_stage_resources: todo!("u32"),
    max_descriptor_set_samplers: todo!("u32"),
    max_descriptor_set_uniform_buffers: todo!("u32"),
    max_descriptor_set_uniform_buffers_dynamic: todo!("u32"),
    max_descriptor_set_storage_buffers: todo!("u32"),
    max_descriptor_set_storage_buffers_dynamic: todo!("u32"),
    max_descriptor_set_sampled_images: todo!("u32"),
    max_descriptor_set_storage_images: todo!("u32"),
    max_descriptor_set_input_attachments: todo!("u32"),
    max_vertex_input_attributes: todo!("u32"),
    max_vertex_input_bindings: todo!("u32"),
    max_vertex_input_attribute_offset: todo!("u32"),
    max_vertex_input_binding_stride: todo!("u32"),
    max_vertex_output_components: todo!("u32"),
    max_tessellation_generation_level: todo!("u32"),
    max_tessellation_patch_size: todo!("u32"),
    max_tessellation_control_per_vertex_input_components: todo!("u32"),
    max_tessellation_control_per_vertex_output_components: todo!("u32"),
    max_tessellation_control_per_patch_output_components: todo!("u32"),
    max_tessellation_control_total_output_components: todo!("u32"),
    max_tessellation_evaluation_input_components: todo!("u32"),
    max_tessellation_evaluation_output_components: todo!("u32"),
    max_geometry_shader_invocations: todo!("u32"),
    max_geometry_input_components: todo!("u32"),
    max_geometry_output_components: todo!("u32"),
    max_geometry_output_vertices: todo!("u32"),
    max_geometry_total_output_components: todo!("u32"),
    max_fragment_input_components: todo!("u32"),
    max_fragment_output_attachments: todo!("u32"),
    max_fragment_dual_src_attachments: todo!("u32"),
    max_fragment_combined_output_resources: todo!("u32"),
    max_compute_shared_memory_size: todo!("u32"),
    max_compute_work_group_count: todo!("[u32; 3]"),
    max_compute_work_group_invocations: todo!("u32"),
    max_compute_work_group_size: todo!("[u32; 3]"),
    sub_pixel_precision_bits: todo!("u32"),
    sub_texel_precision_bits: todo!("u32"),
    mipmap_precision_bits: todo!("u32"),
    max_draw_indexed_index_value: todo!("u32"),
    max_draw_indirect_count: todo!("u32"),
    max_sampler_lod_bias: todo!("f32"),
    max_sampler_anisotropy: todo!("f32"),
    max_viewports: todo!("u32"),
    max_viewport_dimensions: todo!("[u32; 2]"),
    viewport_bounds_range: todo!("[f32; 2]"),
    viewport_sub_pixel_bits: todo!("u32"),
    min_memory_map_alignment: todo!("usize"),
    min_texel_buffer_offset_alignment: todo!("vk::DeviceSize"),
    min_uniform_buffer_offset_alignment: todo!("vk::DeviceSize"),
    min_storage_buffer_offset_alignment: todo!("vk::DeviceSize"),
    min_texel_offset: todo!("i32"),
    max_texel_offset: todo!("u32"),
    min_texel_gather_offset: todo!("i32"),
    max_texel_gather_offset: todo!("u32"),
    min_interpolation_offset: todo!("f32"),
    max_interpolation_offset: todo!("f32"),
    sub_pixel_interpolation_offset_bits: todo!("u32"),
    max_framebuffer_width: todo!("u32"),
    max_framebuffer_height: todo!("u32"),
    max_framebuffer_layers: todo!("u32"),
    framebuffer_color_sample_counts: todo!("vk::SampleCountFlags"),
    framebuffer_depth_sample_counts: todo!("vk::SampleCountFlags"),
    framebuffer_stencil_sample_counts: todo!("vk::SampleCountFlags"),
    framebuffer_no_attachments_sample_counts: todo!("vk::SampleCountFlags"),
    max_color_attachments: todo!("u32"),
    sampled_image_color_sample_counts: todo!("vk::SampleCountFlags"),
    sampled_image_integer_sample_counts: todo!("vk::SampleCountFlags"),
    sampled_image_depth_sample_counts: todo!("vk::SampleCountFlags"),
    sampled_image_stencil_sample_counts: todo!("vk::SampleCountFlags"),
    storage_image_sample_counts: todo!("vk::SampleCountFlags"),
    max_sample_mask_words: todo!("u32"),
    timestamp_compute_and_graphics: todo!("vk::Bool32"),
    timestamp_period: todo!("f32"),
    max_clip_distances: todo!("u32"),
    max_cull_distances: todo!("u32"),
    max_combined_clip_and_cull_distances: todo!("u32"),
    discrete_queue_priorities: todo!("u32"),
    point_size_range: todo!("[f32; 2]"),
    line_width_range: todo!("[f32; 2]"),
    point_size_granularity: todo!("f32"),
    line_width_granularity: todo!("f32"),
    strict_lines: todo!("vk::Bool32"),
    standard_sample_locations: todo!("vk::Bool32"),
    optimal_buffer_copy_offset_alignment: todo!("vk::DeviceSize"),
    optimal_buffer_copy_row_pitch_alignment: todo!("vk::DeviceSize"),
    non_coherent_atom_size: todo!("vk::DeviceSize"),
};
```"#]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Limits"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing mesh shading properties"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceMeshShaderPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_mesh_shader_properties_ext = vk::PhysicalDeviceMeshShaderPropertiesEXT {
    s_type: vk::StructureType::PhysicalDeviceMeshShaderPropertiesEXT,
    p_next: null_mut(),
    max_task_work_group_total_count: todo!("u32"),
    max_task_work_group_count: todo!("[u32; 3]"),
    max_task_work_group_invocations: todo!("u32"),
    max_task_work_group_size: todo!("[u32; 3]"),
    max_task_payload_size: todo!("u32"),
    max_task_shared_memory_size: todo!("u32"),
    max_task_payload_and_shared_memory_size: todo!("u32"),
    max_mesh_work_group_total_count: todo!("u32"),
    max_mesh_work_group_count: todo!("[u32; 3]"),
    max_mesh_work_group_invocations: todo!("u32"),
    max_mesh_work_group_size: todo!("[u32; 3]"),
    max_mesh_shared_memory_size: todo!("u32"),
    max_mesh_payload_and_shared_memory_size: todo!("u32"),
    max_mesh_output_memory_size: todo!("u32"),
    max_mesh_payload_and_output_memory_size: todo!("u32"),
    max_mesh_output_components: todo!("u32"),
    max_mesh_output_vertices: todo!("u32"),
    max_mesh_output_primitives: todo!("u32"),
    max_mesh_output_layers: todo!("u32"),
    max_mesh_multiview_view_count: todo!("u32"),
    mesh_output_per_vertex_granularity: todo!("u32"),
    mesh_output_per_primitive_granularity: todo!("u32"),
    max_preferred_task_work_group_invocations: todo!("u32"),
    max_preferred_mesh_work_group_invocations: todo!("u32"),
    prefers_local_invocation_vertex_output: todo!("vk::Bool32"),
    prefers_local_invocation_primitive_output: todo!("vk::Bool32"),
    prefers_compact_vertex_output: todo!("vk::Bool32"),
    prefers_compact_primitive_output: todo!("vk::Bool32"),
};
```"#]
pub struct PhysicalDeviceMeshShaderPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_task_work_group_total_count: u32,
    pub max_task_work_group_count: [u32; 3],
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_payload_size: u32,
    pub max_task_shared_memory_size: u32,
    pub max_task_payload_and_shared_memory_size: u32,
    pub max_mesh_work_group_total_count: u32,
    pub max_mesh_work_group_count: [u32; 3],
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_shared_memory_size: u32,
    pub max_mesh_payload_and_shared_memory_size: u32,
    pub max_mesh_output_memory_size: u32,
    pub max_mesh_payload_and_output_memory_size: u32,
    pub max_mesh_output_components: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_output_layers: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub max_preferred_task_work_group_invocations: u32,
    pub max_preferred_mesh_work_group_invocations: u32,
    pub prefers_local_invocation_vertex_output: Bool32,
    pub prefers_local_invocation_primitive_output: Bool32,
    pub prefers_compact_vertex_output: Bool32,
    pub prefers_compact_primitive_output: Bool32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Limits"]
#[doc = "<br>"]
#[doc = "**Description**: Structure describing descriptor buffer properties supported by an implementation"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkPhysicalDeviceDescriptorBufferPropertiesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let physical_device_descriptor_buffer_properties_ext = vk::PhysicalDeviceDescriptorBufferPropertiesEXT {
    s_type: vk::StructureType::PhysicalDeviceDescriptorBufferPropertiesEXT,
    p_next: null_mut(),
    combined_image_sampler_descriptor_single_array: todo!("vk::Bool32"),
    bufferless_push_descriptors: todo!("vk::Bool32"),
    allow_sampler_image_view_post_submit_creation: todo!("vk::Bool32"),
    descriptor_buffer_offset_alignment: todo!("vk::DeviceSize"),
    max_descriptor_buffer_bindings: todo!("u32"),
    max_resource_descriptor_buffer_bindings: todo!("u32"),
    max_sampler_descriptor_buffer_bindings: todo!("u32"),
    max_embedded_immutable_sampler_bindings: todo!("u32"),
    max_embedded_immutable_samplers: todo!("u32"),
    buffer_capture_replay_descriptor_data_size: todo!("usize"),
    image_capture_replay_descriptor_data_size: todo!("usize"),
    image_view_capture_replay_descriptor_data_size: todo!("usize"),
    sampler_capture_replay_descriptor_data_size: todo!("usize"),
    acceleration_structure_capture_replay_descriptor_data_size: todo!("usize"),
    sampler_descriptor_size: todo!("usize"),
    combined_image_sampler_descriptor_size: todo!("usize"),
    sampled_image_descriptor_size: todo!("usize"),
    storage_image_descriptor_size: todo!("usize"),
    uniform_texel_buffer_descriptor_size: todo!("usize"),
    robust_uniform_texel_buffer_descriptor_size: todo!("usize"),
    storage_texel_buffer_descriptor_size: todo!("usize"),
    robust_storage_texel_buffer_descriptor_size: todo!("usize"),
    uniform_buffer_descriptor_size: todo!("usize"),
    robust_uniform_buffer_descriptor_size: todo!("usize"),
    storage_buffer_descriptor_size: todo!("usize"),
    robust_storage_buffer_descriptor_size: todo!("usize"),
    input_attachment_descriptor_size: todo!("usize"),
    acceleration_structure_descriptor_size: todo!("usize"),
    max_sampler_descriptor_buffer_range: todo!("vk::DeviceSize"),
    max_resource_descriptor_buffer_range: todo!("vk::DeviceSize"),
    sampler_descriptor_buffer_address_space_size: todo!("vk::DeviceSize"),
    resource_descriptor_buffer_address_space_size: todo!("vk::DeviceSize"),
    descriptor_buffer_address_space_size: todo!("vk::DeviceSize"),
};
```"#]
pub struct PhysicalDeviceDescriptorBufferPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_single_array: Bool32,
    pub bufferless_push_descriptors: Bool32,
    pub allow_sampler_image_view_post_submit_creation: Bool32,
    pub descriptor_buffer_offset_alignment: DeviceSize,
    pub max_descriptor_buffer_bindings: u32,
    pub max_resource_descriptor_buffer_bindings: u32,
    pub max_sampler_descriptor_buffer_bindings: u32,
    pub max_embedded_immutable_sampler_bindings: u32,
    pub max_embedded_immutable_samplers: u32,
    pub buffer_capture_replay_descriptor_data_size: usize,
    pub image_capture_replay_descriptor_data_size: usize,
    pub image_view_capture_replay_descriptor_data_size: usize,
    pub sampler_capture_replay_descriptor_data_size: usize,
    pub acceleration_structure_capture_replay_descriptor_data_size: usize,
    pub sampler_descriptor_size: usize,
    pub combined_image_sampler_descriptor_size: usize,
    pub sampled_image_descriptor_size: usize,
    pub storage_image_descriptor_size: usize,
    pub uniform_texel_buffer_descriptor_size: usize,
    pub robust_uniform_texel_buffer_descriptor_size: usize,
    pub storage_texel_buffer_descriptor_size: usize,
    pub robust_storage_texel_buffer_descriptor_size: usize,
    pub uniform_buffer_descriptor_size: usize,
    pub robust_uniform_buffer_descriptor_size: usize,
    pub storage_buffer_descriptor_size: usize,
    pub robust_storage_buffer_descriptor_size: usize,
    pub input_attachment_descriptor_size: usize,
    pub acceleration_structure_descriptor_size: usize,
    pub max_sampler_descriptor_buffer_range: DeviceSize,
    pub max_resource_descriptor_buffer_range: DeviceSize,
    pub sampler_descriptor_buffer_address_space_size: DeviceSize,
    pub resource_descriptor_buffer_address_space_size: DeviceSize,
    pub descriptor_buffer_address_space_size: DeviceSize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Specify parameters of a name to give to an object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsObjectNameInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let debug_utils_object_name_info_ext = vk::DebugUtilsObjectNameInfoEXT {
    s_type: vk::StructureType::DebugUtilsObjectNameInfoEXT,
    p_next: null(),
    object_type: todo!("vk::ObjectType"),
    object_handle: todo!("u64"),
    p_object_name: todo!("*const c_char"),
};
```"#]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Specify parameters of a label region"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsLabelEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let debug_utils_label_ext = vk::DebugUtilsLabelEXT {
    s_type: vk::StructureType::DebugUtilsLabelEXT,
    p_next: null(),
    p_label_name: todo!("*const c_char"),
    color: todo!("[f32; 4]"),
};
```"#]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters of a newly created debug messenger"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsMessengerCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let debug_utils_messenger_create_info_ext = vk::DebugUtilsMessengerCreateInfoEXT {
    s_type: vk::StructureType::DebugUtilsMessengerCreateInfoEXT,
    p_next: null(),
    flags: todo!("vk::DebugUtilsMessengerCreateFlagsEXT"),
    message_severity: todo!("vk::DebugUtilsMessageSeverityFlagsEXT"),
    message_type: todo!("vk::DebugUtilsMessageTypeFlagsEXT"),
    pfn_user_callback: todo!("vk::PfnDebugUtilsMessengerCallbackEXT"),
    p_user_data: todo!("*mut c_void"),
};
```"#]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: PfnDebugUtilsMessengerCallbackEXT,
    pub p_user_data: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying parameters returned to the callback"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDebugUtilsMessengerCallbackDataEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html)"]
#[doc = "<br>"]
#[doc = "**Initialization template**:"]
#[doc = r#"```
let debug_utils_messenger_callback_data_ext = vk::DebugUtilsMessengerCallbackDataEXT {
    s_type: vk::StructureType::DebugUtilsMessengerCallbackDataEXT,
    p_next: null(),
    flags: todo!("vk::DebugUtilsMessengerCallbackDataFlagsEXT"),
    p_message_id_name: todo!("*const c_char"),
    message_id_number: todo!("i32"),
    p_message: todo!("*const c_char"),
    queue_label_count: todo!("u32"),
    p_queue_labels: todo!("*const vk::DebugUtilsLabelEXT"),
    cmd_buf_label_count: todo!("u32"),
    p_cmd_buf_labels: todo!("*const vk::DebugUtilsLabelEXT"),
    object_count: todo!("u32"),
    p_objects: todo!("*const vk::DebugUtilsObjectNameInfoEXT"),
};
```"#]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}

//
// Unions
//

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying descriptor data"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkDescriptorDataEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorDataEXT.html)"]
pub union DescriptorDataEXT {
    pub p_sampler: *const Sampler,
    pub p_combined_image_sampler: *const DescriptorImageInfo,
    pub p_input_attachment_image: *const DescriptorImageInfo,
    pub p_sampled_image: *const DescriptorImageInfo,
    pub p_storage_image: *const DescriptorImageInfo,
    pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_uniform_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_buffer: *const DescriptorAddressInfoEXT,
    pub acceleration_structure: DeviceAddress,
}

impl std::fmt::Debug for DescriptorDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DescriptorDataEXT").finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Clear Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a clear color value"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkClearColorValue`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html)"]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl std::fmt::Debug for ClearColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClearColorValue").finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "**Chapter**: Clear Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Structure specifying a clear value"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`VkClearValue`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearValue.html)"]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

impl std::fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClearValue").finish()
    }
}

//
// Command types
//

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Return a function pointer for a command"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)"]
pub type GetInstanceProcAddr = unsafe extern "C" fn(
    instance: Instance,    //
    p_name: *const c_char, //
) -> PfnVoidFunction;

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Return a function pointer for a command"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)"]
pub type GetDeviceProcAddr = unsafe extern "C" fn(
    device: Device,        //
    p_name: *const c_char, //
) -> PfnVoidFunction;

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new Vulkan instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)"]
pub type CreateInstance = unsafe extern "C" fn(
    p_create_info: *const InstanceCreateInfo, //
    p_allocator: *const AllocationCallbacks,  //
    p_instance: *mut Instance,                //
) -> Result;

#[doc = "**Chapter**: Initialization"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy an instance of Vulkan"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)"]
pub type DestroyInstance = unsafe extern "C" fn(
    instance: Instance,                      //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Enumerates the physical devices accessible to a Vulkan instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)"]
pub type EnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: Instance,                      //
    p_physical_device_count: *mut u32,       //
    p_physical_devices: *mut PhysicalDevice, //
) -> Result;

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Returns properties of a physical device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetPhysicalDeviceProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)"]
pub type GetPhysicalDeviceProperties2 = unsafe extern "C" fn(
    physical_device: PhysicalDevice,              //
    p_properties: *mut PhysicalDeviceProperties2, //
);

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Reports properties of the queues of the specified physical device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)"]
pub type GetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "C" fn(
    physical_device: PhysicalDevice,                        //
    p_queue_family_property_count: *mut u32,                //
    p_queue_family_properties: *mut QueueFamilyProperties2, //
);

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new device instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)"]
pub type CreateDevice = unsafe extern "C" fn(
    physical_device: PhysicalDevice,         //
    p_create_info: *const DeviceCreateInfo,  //
    p_allocator: *const AllocationCallbacks, //
    p_device: *mut Device,                   //
) -> Result;

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a logical device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)"]
pub type DestroyDevice = unsafe extern "C" fn(
    device: Device,                          //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Devices and Queues"]
#[doc = "<br>"]
#[doc = "**Description**: Get a queue handle from a device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDeviceQueue2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)"]
pub type GetDeviceQueue2 = unsafe extern "C" fn(
    device: Device,                        //
    p_queue_info: *const DeviceQueueInfo2, //
    p_queue: *mut Queue,                   //
);

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new command pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)"]
pub type CreateCommandPool = unsafe extern "C" fn(
    device: Device,                              //
    p_create_info: *const CommandPoolCreateInfo, //
    p_allocator: *const AllocationCallbacks,     //
    p_command_pool: *mut CommandPool,            //
) -> Result;

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Reset a command pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)"]
pub type ResetCommandPool = unsafe extern "C" fn(
    device: Device,               //
    command_pool: CommandPool,    //
    flags: CommandPoolResetFlags, //
) -> Result;

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a command pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)"]
pub type DestroyCommandPool = unsafe extern "C" fn(
    device: Device,                          //
    command_pool: CommandPool,               //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Allocate command buffers from an existing command pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)"]
pub type AllocateCommandBuffers = unsafe extern "C" fn(
    device: Device,                                    //
    p_allocate_info: *const CommandBufferAllocateInfo, //
    p_command_buffers: *mut CommandBuffer,             //
) -> Result;

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Free command buffers"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)"]
pub type FreeCommandBuffers = unsafe extern "C" fn(
    device: Device,                          //
    command_pool: CommandPool,               //
    command_buffer_count: u32,               //
    p_command_buffers: *const CommandBuffer, //
);

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Start recording a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)"]
pub type BeginCommandBuffer = unsafe extern "C" fn(
    command_buffer: CommandBuffer,               //
    p_begin_info: *const CommandBufferBeginInfo, //
) -> Result;

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Finish recording a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)"]
pub type EndCommandBuffer = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
) -> Result;

#[doc = "**Chapter**: Command Buffers"]
#[doc = "<br>"]
#[doc = "**Description**: Submits command buffers to a queue"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkQueueSubmit2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)"]
pub type QueueSubmit2 = unsafe extern "C" fn(
    queue: Queue,                  //
    submit_count: u32,             //
    p_submits: *const SubmitInfo2, //
    fence: Fence,                  //
) -> Result;

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new queue semaphore object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)"]
pub type CreateSemaphore = unsafe extern "C" fn(
    device: Device,                            //
    p_create_info: *const SemaphoreCreateInfo, //
    p_allocator: *const AllocationCallbacks,   //
    p_semaphore: *mut Semaphore,               //
) -> Result;

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a semaphore object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)"]
pub type DestroySemaphore = unsafe extern "C" fn(
    device: Device,                          //
    semaphore: Semaphore,                    //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Wait for timeline semaphores on the host"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkWaitSemaphores`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)"]
pub type WaitSemaphores = unsafe extern "C" fn(
    device: Device,                        //
    p_wait_info: *const SemaphoreWaitInfo, //
    timeout: u64,                          //
) -> Result;

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Insert a memory dependency"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdPipelineBarrier2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)"]
pub type CmdPipelineBarrier2 = unsafe extern "C" fn(
    command_buffer: CommandBuffer,            //
    p_dependency_info: *const DependencyInfo, //
);

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Wait for a queue to become idle"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)"]
pub type QueueWaitIdle = unsafe extern "C" fn(
    queue: Queue, //
) -> Result;

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Wait for a device to become idle"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)"]
pub type DeviceWaitIdle = unsafe extern "C" fn(
    device: Device, //
) -> Result;

#[doc = "**Chapter**: Synchronization and Cache Control"]
#[doc = "<br>"]
#[doc = "**Description**: Query calibrated timestamps"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetCalibratedTimestampsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)"]
pub type GetCalibratedTimestampsEXT = unsafe extern "C" fn(
    device: Device,                                       //
    timestamp_count: u32,                                 //
    p_timestamp_infos: *const CalibratedTimestampInfoEXT, //
    p_timestamps: *mut u64,                               //
    p_max_deviation: *mut u64,                            //
) -> Result;

#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: Begin a dynamic render pass instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdBeginRendering`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)"]
pub type CmdBeginRendering = unsafe extern "C" fn(
    command_buffer: CommandBuffer,          //
    p_rendering_info: *const RenderingInfo, //
);

#[doc = "**Chapter**: Render Pass"]
#[doc = "<br>"]
#[doc = "**Description**: End a dynamic render pass instance"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdEndRendering`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)"]
pub type CmdEndRendering = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
);

#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Create one or more new shaders"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html)"]
pub type CreateShadersEXT = unsafe extern "C" fn(
    device: Device,                             //
    create_info_count: u32,                     //
    p_create_infos: *const ShaderCreateInfoEXT, //
    p_allocator: *const AllocationCallbacks,    //
    p_shaders: *mut ShaderEXT,                  //
) -> Result;

#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Bind shader objects to a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdBindShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html)"]
pub type CmdBindShadersEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,        //
    stage_count: u32,                     //
    p_stages: *const ShaderStageFlagBits, //
    p_shaders: *const ShaderEXT,          //
);

#[doc = "**Chapter**: Shaders"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a shader object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyShaderEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html)"]
pub type DestroyShaderEXT = unsafe extern "C" fn(
    device: Device,                          //
    shader: ShaderEXT,                       //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Reports memory information for the specified physical device"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetPhysicalDeviceMemoryProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)"]
pub type GetPhysicalDeviceMemoryProperties2 = unsafe extern "C" fn(
    physical_device: PhysicalDevice,                           //
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2, //
);

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Allocate device memory"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)"]
pub type AllocateMemory = unsafe extern "C" fn(
    device: Device,                             //
    p_allocate_info: *const MemoryAllocateInfo, //
    p_allocator: *const AllocationCallbacks,    //
    p_memory: *mut DeviceMemory,                //
) -> Result;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Free device memory"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)"]
pub type FreeMemory = unsafe extern "C" fn(
    device: Device,                          //
    memory: DeviceMemory,                    //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Map a memory object into application address space"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkMapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory2KHR.html)"]
pub type MapMemory2KHR = unsafe extern "C" fn(
    device: Device,                             //
    p_memory_map_info: *const MemoryMapInfoKHR, //
    pp_data: *mut *mut c_void,                  //
) -> Result;

#[doc = "**Chapter**: Memory Allocation"]
#[doc = "<br>"]
#[doc = "**Description**: Unmap a previously mapped memory object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkUnmapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory2KHR.html)"]
pub type UnmapMemory2KHR = unsafe extern "C" fn(
    device: Device,                                 //
    p_memory_unmap_info: *const MemoryUnmapInfoKHR, //
) -> Result;

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)"]
pub type CreateBuffer = unsafe extern "C" fn(
    device: Device,                          //
    p_create_info: *const BufferCreateInfo,  //
    p_allocator: *const AllocationCallbacks, //
    p_buffer: *mut Buffer,                   //
) -> Result;

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a buffer object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)"]
pub type DestroyBuffer = unsafe extern "C" fn(
    device: Device,                          //
    buffer: Buffer,                          //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new image object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)"]
pub type CreateImage = unsafe extern "C" fn(
    device: Device,                          //
    p_create_info: *const ImageCreateInfo,   //
    p_allocator: *const AllocationCallbacks, //
    p_image: *mut Image,                     //
) -> Result;

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy an image object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)"]
pub type DestroyImage = unsafe extern "C" fn(
    device: Device,                          //
    image: Image,                            //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Create an image view from an existing image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)"]
pub type CreateImageView = unsafe extern "C" fn(
    device: Device,                            //
    p_create_info: *const ImageViewCreateInfo, //
    p_allocator: *const AllocationCallbacks,   //
    p_view: *mut ImageView,                    //
) -> Result;

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy an image view object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)"]
pub type DestroyImageView = unsafe extern "C" fn(
    device: Device,                          //
    image_view: ImageView,                   //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Returns the memory requirements for specified Vulkan object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDeviceBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)"]
pub type GetDeviceBufferMemoryRequirements = unsafe extern "C" fn(
    device: Device,                                  //
    p_info: *const DeviceBufferMemoryRequirements,   //
    p_memory_requirements: *mut MemoryRequirements2, //
);

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Returns the memory requirements for specified Vulkan object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDeviceImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)"]
pub type GetDeviceImageMemoryRequirements = unsafe extern "C" fn(
    device: Device,                                  //
    p_info: *const DeviceImageMemoryRequirements,    //
    p_memory_requirements: *mut MemoryRequirements2, //
);

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bind device memory to buffer objects"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkBindBufferMemory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)"]
pub type BindBufferMemory2 = unsafe extern "C" fn(
    device: Device,                            //
    bind_info_count: u32,                      //
    p_bind_infos: *const BindBufferMemoryInfo, //
) -> Result;

#[doc = "**Chapter**: Resource Creation"]
#[doc = "<br>"]
#[doc = "**Description**: Bind device memory to image objects"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkBindImageMemory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)"]
pub type BindImageMemory2 = unsafe extern "C" fn(
    device: Device,                           //
    bind_info_count: u32,                     //
    p_bind_infos: *const BindImageMemoryInfo, //
) -> Result;

#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new sampler object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)"]
pub type CreateSampler = unsafe extern "C" fn(
    device: Device,                          //
    p_create_info: *const SamplerCreateInfo, //
    p_allocator: *const AllocationCallbacks, //
    p_sampler: *mut Sampler,                 //
) -> Result;

#[doc = "**Chapter**: Samplers"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a sampler object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)"]
pub type DestroySampler = unsafe extern "C" fn(
    device: Device,                          //
    sampler: Sampler,                        //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new descriptor set layout"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)"]
pub type CreateDescriptorSetLayout = unsafe extern "C" fn(
    device: Device,                                      //
    p_create_info: *const DescriptorSetLayoutCreateInfo, //
    p_allocator: *const AllocationCallbacks,             //
    p_set_layout: *mut DescriptorSetLayout,              //
) -> Result;

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a descriptor set layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)"]
pub type DestroyDescriptorSetLayout = unsafe extern "C" fn(
    device: Device,                             //
    descriptor_set_layout: DescriptorSetLayout, //
    p_allocator: *const AllocationCallbacks,    //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Creates a new pipeline layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)"]
pub type CreatePipelineLayout = unsafe extern "C" fn(
    device: Device,                                 //
    p_create_info: *const PipelineLayoutCreateInfo, //
    p_allocator: *const AllocationCallbacks,        //
    p_pipeline_layout: *mut PipelineLayout,         //
) -> Result;

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a pipeline layout object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)"]
pub type DestroyPipelineLayout = unsafe extern "C" fn(
    device: Device,                          //
    pipeline_layout: PipelineLayout,         //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Update the values of push constants"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)"]
pub type CmdPushConstants = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    layout: PipelineLayout,        //
    stage_flags: ShaderStageFlags, //
    offset: u32,                   //
    size: u32,                     //
    p_values: *const c_void,       //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Query an address of a buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetBufferDeviceAddress`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)"]
pub type GetBufferDeviceAddress = unsafe extern "C" fn(
    device: Device,                         //
    p_info: *const BufferDeviceAddressInfo, //
) -> DeviceAddress;

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Get the size of a descriptor set layout in memory"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDescriptorSetLayoutSizeEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html)"]
pub type GetDescriptorSetLayoutSizeEXT = unsafe extern "C" fn(
    device: Device,                          //
    layout: DescriptorSetLayout,             //
    p_layout_size_in_bytes: *mut DeviceSize, //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Get the offset of a binding within a descriptor set layout"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html)"]
pub type GetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "C" fn(
    device: Device,              //
    layout: DescriptorSetLayout, //
    binding: u32,                //
    p_offset: *mut DeviceSize,   //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: To get a descriptor to place in a buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetDescriptorEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html)"]
pub type GetDescriptorEXT = unsafe extern "C" fn(
    device: Device,                                 //
    p_descriptor_info: *const DescriptorGetInfoEXT, //
    data_size: usize,                               //
    p_descriptor: *mut c_void,                      //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Binding descriptor buffers to a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdBindDescriptorBuffersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html)"]
pub type CmdBindDescriptorBuffersEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,                          //
    buffer_count: u32,                                      //
    p_binding_infos: *const DescriptorBufferBindingInfoEXT, //
);

#[doc = "**Chapter**: Resource Descriptors"]
#[doc = "<br>"]
#[doc = "**Description**: Setting descriptor buffer offsets in a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetDescriptorBufferOffsetsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html)"]
pub type CmdSetDescriptorBufferOffsetsEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,          //
    pipeline_bind_point: PipelineBindPoint, //
    layout: PipelineLayout,                 //
    first_set: u32,                         //
    set_count: u32,                         //
    p_buffer_indices: *const u32,           //
    p_offsets: *const DeviceSize,           //
);

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Create a new query pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)"]
pub type CreateQueryPool = unsafe extern "C" fn(
    device: Device,                            //
    p_create_info: *const QueryPoolCreateInfo, //
    p_allocator: *const AllocationCallbacks,   //
    p_query_pool: *mut QueryPool,              //
) -> Result;

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a query pool object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)"]
pub type DestroyQueryPool = unsafe extern "C" fn(
    device: Device,                          //
    query_pool: QueryPool,                   //
    p_allocator: *const AllocationCallbacks, //
);

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Reset queries in a query pool"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)"]
pub type ResetQueryPool = unsafe extern "C" fn(
    device: Device,        //
    query_pool: QueryPool, //
    first_query: u32,      //
    query_count: u32,      //
);

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Begin a query"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)"]
pub type CmdBeginQuery = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    query_pool: QueryPool,         //
    query: u32,                    //
    flags: QueryControlFlags,      //
);

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Ends a query"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)"]
pub type CmdEndQuery = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    query_pool: QueryPool,         //
    query: u32,                    //
);

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Copy results of queries in a query pool to a host memory region"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)"]
pub type GetQueryPoolResults = unsafe extern "C" fn(
    device: Device,          //
    query_pool: QueryPool,   //
    first_query: u32,        //
    query_count: u32,        //
    data_size: usize,        //
    p_data: *mut c_void,     //
    stride: DeviceSize,      //
    flags: QueryResultFlags, //
) -> Result;

#[doc = "**Chapter**: Queries"]
#[doc = "<br>"]
#[doc = "**Description**: Write a device timestamp into a query object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdWriteTimestamp2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)"]
pub type CmdWriteTimestamp2 = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    stage: PipelineStageFlags2,    //
    query_pool: QueryPool,         //
    query: u32,                    //
);

#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Copy data from a buffer into an image"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdCopyBufferToImage2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)"]
pub type CmdCopyBufferToImage2 = unsafe extern "C" fn(
    command_buffer: CommandBuffer,                              //
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2, //
);

#[doc = "**Chapter**: Copy Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Copy image data into a buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdCopyImageToBuffer2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)"]
pub type CmdCopyImageToBuffer2 = unsafe extern "C" fn(
    command_buffer: CommandBuffer,                              //
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2, //
);

#[doc = "**Chapter**: Drawing Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Draw mesh task work items"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdDrawMeshTasksEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)"]
pub type CmdDrawMeshTasksEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    group_count_x: u32,            //
    group_count_y: u32,            //
    group_count_z: u32,            //
);

#[doc = "**Chapter**: Fixed-Function Vertex Post-Processing"]
#[doc = "<br>"]
#[doc = "**Description**: Set the viewport count and viewports dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetViewportWithCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)"]
pub type CmdSetViewportWithCount = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    viewport_count: u32,           //
    p_viewports: *const Viewport,  //
);

#[doc = "**Chapter**: Fixed-Function Vertex Post-Processing"]
#[doc = "<br>"]
#[doc = "**Description**: Set the scissor count and scissor rectangular bounds dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetScissorWithCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)"]
pub type CmdSetScissorWithCount = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    scissor_count: u32,            //
    p_scissors: *const Rect2D,     //
);

#[doc = "**Chapter**: Rasterization"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the rasterization samples dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetRasterizationSamplesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)"]
pub type CmdSetRasterizationSamplesEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,              //
    rasterization_samples: SampleCountFlagBits, //
);

#[doc = "**Chapter**: Rasterization"]
#[doc = "<br>"]
#[doc = "**Description**: Set front face orientation dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)"]
pub type CmdSetFrontFace = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    front_face: FrontFace,         //
);

#[doc = "**Chapter**: Rasterization"]
#[doc = "<br>"]
#[doc = "**Description**: Set cull mode dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetCullMode`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)"]
pub type CmdSetCullMode = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    cull_mode: CullModeFlags,      //
);

#[doc = "**Chapter**: Fragment Operations"]
#[doc = "<br>"]
#[doc = "**Description**: Set depth test enable dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetDepthTestEnable`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)"]
pub type CmdSetDepthTestEnable = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    depth_test_enable: Bool32,     //
);

#[doc = "**Chapter**: Fragment Operations"]
#[doc = "<br>"]
#[doc = "**Description**: Set depth comparison operator dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetDepthCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)"]
pub type CmdSetDepthCompareOp = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    depth_compare_op: CompareOp,   //
);

#[doc = "**Chapter**: Fragment Operations"]
#[doc = "<br>"]
#[doc = "**Description**: Set depth write enable dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetDepthWriteEnable`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)"]
pub type CmdSetDepthWriteEnable = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    depth_write_enable: Bool32,    //
);

#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the pname:blendEnable for each attachment dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetColorBlendEnableEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)"]
pub type CmdSetColorBlendEnableEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,        //
    first_attachment: u32,                //
    attachment_count: u32,                //
    p_color_blend_enables: *const Bool32, //
);

#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the blend factors and operations dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetColorBlendEquationEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)"]
pub type CmdSetColorBlendEquationEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,                         //
    first_attachment: u32,                                 //
    attachment_count: u32,                                 //
    p_color_blend_equations: *const ColorBlendEquationEXT, //
);

#[doc = "**Chapter**: The Framebuffer"]
#[doc = "<br>"]
#[doc = "**Description**: Specify the color write masks for each attachment dynamically for a command buffer"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdSetColorWriteMaskEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)"]
pub type CmdSetColorWriteMaskEXT = unsafe extern "C" fn(
    command_buffer: CommandBuffer,                   //
    first_attachment: u32,                           //
    attachment_count: u32,                           //
    p_color_write_masks: *const ColorComponentFlags, //
);

#[doc = "**Chapter**: Dispatching Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Dispatch compute work items"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)"]
pub type CmdDispatch = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    group_count_x: u32,            //
    group_count_y: u32,            //
    group_count_z: u32,            //
);

#[doc = "**Chapter**: Dispatching Commands"]
#[doc = "<br>"]
#[doc = "**Description**: Dispatch compute work items with indirect parameters"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)"]
pub type CmdDispatchIndirect = unsafe extern "C" fn(
    command_buffer: CommandBuffer, //
    buffer: Buffer,                //
    offset: DeviceSize,            //
);

#[doc = "**Chapter**: Additional Capabilities"]
#[doc = "<br>"]
#[doc = "**Description**: Query calibrateable time domains"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)"]
pub type GetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "C" fn(
    physical_device: PhysicalDevice,    //
    p_time_domain_count: *mut u32,      //
    p_time_domains: *mut TimeDomainEXT, //
) -> Result;

#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Create a debug messenger object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkCreateDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)"]
pub type CreateDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: Instance,                                     //
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT, //
    p_allocator: *const AllocationCallbacks,                //
    p_messenger: *mut DebugUtilsMessengerEXT,               //
) -> Result;

#[doc = "**Chapter**: Debugging"]
#[doc = "<br>"]
#[doc = "**Description**: Destroy a debug messenger object"]
#[doc = "<br>"]
#[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
#[doc = "<br>"]
#[doc = "**Reference**: [`vkDestroyDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)"]
pub type DestroyDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: Instance,                      //
    messenger: DebugUtilsMessengerEXT,       //
    p_allocator: *const AllocationCallbacks, //
);
