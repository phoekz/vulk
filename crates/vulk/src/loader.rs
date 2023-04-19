//
// Imports
//

use super::{vk, Error};
use std::ffi::{c_char, c_void};

//
// Init
//

pub struct InitFunctions {
    pub get_instance_proc_addr: vk::GetInstanceProcAddr,
    pub create_instance: vk::CreateInstance,
}

pub struct Init {
    fns: InitFunctions,
    _library: std::sync::Arc<libloading::Library>,
}

impl Init {
    pub unsafe fn load() -> Result<Self, Error> {
        // Only Windows and Linux are supported.
        #[cfg(windows)]
        const VULKAN_LIB_PATH: &str = "vulkan-1.dll";
        #[cfg(unix)]
        const VULKAN_LIB_PATH: &str = "libvulkan.so.1";

        // Load library.
        let library = libloading::Library::new(VULKAN_LIB_PATH)
            .map_err(|_| Error::LibraryLoad(std::borrow::Cow::Borrowed(VULKAN_LIB_PATH)))
            .map(std::sync::Arc::new)?;

        // Load functions.
        let load = |name: &'static [u8]| {
            let pfn = if let Ok(symbol) = library.get::<*const c_void>(name) {
                *symbol
            } else {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            };
            if pfn as usize == 0 {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            }
            Ok(pfn)
        };

        Ok(Self {
            fns: InitFunctions {
                get_instance_proc_addr: std::mem::transmute(load(b"vkGetInstanceProcAddr\0")?),
                create_instance: std::mem::transmute(load(b"vkCreateInstance\0")?),
            },
            _library: library,
        })
    }

    #[must_use]
    pub fn fns(&self) -> &InitFunctions {
        &self.fns
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Initialization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Return a function pointer for a command"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)"]
    pub unsafe fn get_instance_proc_addr(&self, instance: vk::Instance, p_name: *const c_char) -> vk::PfnVoidFunction {
        (self.fns.get_instance_proc_addr)(instance, p_name)
    }

    #[inline]
    #[doc = "**Chapter**: Initialization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new Vulkan instance"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)"]
    pub unsafe fn create_instance(&self, p_create_info: *const vk::InstanceCreateInfo) -> Result<vk::Instance, Error> {
        let mut p_instance = std::mem::MaybeUninit::uninit();
        match (self.fns.create_instance)(p_create_info, std::ptr::null(), p_instance.as_mut_ptr()) {
            vk::Result::Success => Ok(p_instance.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }
}

//
// Instance
//

pub struct InstanceFunctions {
    pub get_device_proc_addr: vk::GetDeviceProcAddr,
    pub destroy_instance: vk::DestroyInstance,
    pub enumerate_physical_devices: vk::EnumeratePhysicalDevices,
    pub get_physical_device_properties2: vk::GetPhysicalDeviceProperties2,
    pub get_physical_device_queue_family_properties2: vk::GetPhysicalDeviceQueueFamilyProperties2,
    pub create_device: vk::CreateDevice,
    pub get_physical_device_memory_properties2: vk::GetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_calibrateable_time_domains_ext: vk::GetPhysicalDeviceCalibrateableTimeDomainsEXT,
    pub create_debug_utils_messenger_ext: vk::CreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger_ext: vk::DestroyDebugUtilsMessengerEXT,
}

pub struct Instance {
    fns: InstanceFunctions,
    handle: vk::Instance,
}

impl Instance {
    pub unsafe fn load(init: &Init, instance: vk::Instance) -> Result<Self, Error> {
        let load = |name: &'static [u8]| {
            let pfn = init.get_instance_proc_addr(instance, name.as_ptr().cast());
            if pfn as usize == 0 {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            }
            Ok(pfn)
        };

        Ok(Self {
            fns: InstanceFunctions {
                get_device_proc_addr: std::mem::transmute(load(b"vkGetDeviceProcAddr\0")?),
                destroy_instance: std::mem::transmute(load(b"vkDestroyInstance\0")?),
                enumerate_physical_devices: std::mem::transmute(load(b"vkEnumeratePhysicalDevices\0")?),
                get_physical_device_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceProperties2\0")?),
                get_physical_device_queue_family_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceQueueFamilyProperties2\0")?),
                create_device: std::mem::transmute(load(b"vkCreateDevice\0")?),
                get_physical_device_memory_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceMemoryProperties2\0")?),
                get_physical_device_calibrateable_time_domains_ext: std::mem::transmute(load(b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0")?),
                create_debug_utils_messenger_ext: std::mem::transmute(load(b"vkCreateDebugUtilsMessengerEXT\0")?),
                destroy_debug_utils_messenger_ext: std::mem::transmute(load(b"vkDestroyDebugUtilsMessengerEXT\0")?),
            },
            handle: instance,
        })
    }

    #[must_use]
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    #[must_use]
    pub fn fns(&self) -> &InstanceFunctions {
        &self.fns
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Initialization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Return a function pointer for a command"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)"]
    pub unsafe fn get_device_proc_addr(&self, device: vk::Device, p_name: *const c_char) -> vk::PfnVoidFunction {
        (self.fns.get_device_proc_addr)(device, p_name)
    }

    #[inline]
    #[doc = "**Chapter**: Initialization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy an instance of Vulkan"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)"]
    pub unsafe fn destroy_instance(&self) {
        (self.fns.destroy_instance)(self.handle, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Enumerates the physical devices accessible to a Vulkan instance"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)"]
    pub unsafe fn enumerate_physical_devices(&self, p_physical_device_count: *mut u32, p_physical_devices: *mut vk::PhysicalDevice) -> Result<(), Error> {
        match (self.fns.enumerate_physical_devices)(self.handle, p_physical_device_count, p_physical_devices) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Returns properties of a physical device"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)"]
    pub unsafe fn get_physical_device_properties2(&self, physical_device: vk::PhysicalDevice, p_properties: *mut vk::PhysicalDeviceProperties2) {
        (self.fns.get_physical_device_properties2)(physical_device, p_properties);
    }

    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reports properties of the queues of the specified physical device"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)"]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut vk::QueueFamilyProperties2,
    ) {
        (self.fns.get_physical_device_queue_family_properties2)(physical_device, p_queue_family_property_count, p_queue_family_properties);
    }

    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new device instance"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)"]
    pub unsafe fn create_device(&self, physical_device: vk::PhysicalDevice, p_create_info: *const vk::DeviceCreateInfo) -> Result<vk::Device, Error> {
        let mut p_device = std::mem::MaybeUninit::uninit();
        match (self.fns.create_device)(physical_device, p_create_info, std::ptr::null(), p_device.as_mut_ptr()) {
            vk::Result::Success => Ok(p_device.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reports memory information for the specified physical device"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceMemoryProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)"]
    pub unsafe fn get_physical_device_memory_properties2(&self, physical_device: vk::PhysicalDevice, p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2) {
        (self.fns.get_physical_device_memory_properties2)(physical_device, p_memory_properties);
    }

    #[inline]
    #[doc = "**Chapter**: Additional Capabilities"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query calibrateable time domains"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)"]
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut vk::TimeDomainEXT,
    ) -> Result<(), Error> {
        match (self.fns.get_physical_device_calibrateable_time_domains_ext)(physical_device, p_time_domain_count, p_time_domains) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a debug messenger object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)"]
    pub unsafe fn create_debug_utils_messenger_ext(&self, p_create_info: *const vk::DebugUtilsMessengerCreateInfoEXT) -> Result<vk::DebugUtilsMessengerEXT, Error> {
        let mut p_messenger = std::mem::MaybeUninit::uninit();
        match (self.fns.create_debug_utils_messenger_ext)(self.handle, p_create_info, std::ptr::null(), p_messenger.as_mut_ptr()) {
            vk::Result::Success => Ok(p_messenger.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Debugging"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a debug messenger object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)"]
    pub unsafe fn destroy_debug_utils_messenger_ext(&self, messenger: vk::DebugUtilsMessengerEXT) {
        (self.fns.destroy_debug_utils_messenger_ext)(self.handle, messenger, std::ptr::null());
    }
}

//
// Device
//

pub struct DeviceFunctions {
    pub destroy_device: vk::DestroyDevice,
    pub get_device_queue2: vk::GetDeviceQueue2,
    pub create_command_pool: vk::CreateCommandPool,
    pub reset_command_pool: vk::ResetCommandPool,
    pub destroy_command_pool: vk::DestroyCommandPool,
    pub allocate_command_buffers: vk::AllocateCommandBuffers,
    pub free_command_buffers: vk::FreeCommandBuffers,
    pub begin_command_buffer: vk::BeginCommandBuffer,
    pub end_command_buffer: vk::EndCommandBuffer,
    pub queue_submit2: vk::QueueSubmit2,
    pub create_semaphore: vk::CreateSemaphore,
    pub destroy_semaphore: vk::DestroySemaphore,
    pub wait_semaphores: vk::WaitSemaphores,
    pub cmd_pipeline_barrier2: vk::CmdPipelineBarrier2,
    pub queue_wait_idle: vk::QueueWaitIdle,
    pub device_wait_idle: vk::DeviceWaitIdle,
    pub get_calibrated_timestamps_ext: vk::GetCalibratedTimestampsEXT,
    pub cmd_begin_rendering: vk::CmdBeginRendering,
    pub cmd_end_rendering: vk::CmdEndRendering,
    pub create_shaders_ext: vk::CreateShadersEXT,
    pub cmd_bind_shaders_ext: vk::CmdBindShadersEXT,
    pub destroy_shader_ext: vk::DestroyShaderEXT,
    pub allocate_memory: vk::AllocateMemory,
    pub free_memory: vk::FreeMemory,
    pub map_memory2_khr: vk::MapMemory2KHR,
    pub unmap_memory2_khr: vk::UnmapMemory2KHR,
    pub create_buffer: vk::CreateBuffer,
    pub destroy_buffer: vk::DestroyBuffer,
    pub create_image: vk::CreateImage,
    pub destroy_image: vk::DestroyImage,
    pub create_image_view: vk::CreateImageView,
    pub destroy_image_view: vk::DestroyImageView,
    pub get_device_buffer_memory_requirements: vk::GetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements: vk::GetDeviceImageMemoryRequirements,
    pub bind_buffer_memory2: vk::BindBufferMemory2,
    pub bind_image_memory2: vk::BindImageMemory2,
    pub create_sampler: vk::CreateSampler,
    pub destroy_sampler: vk::DestroySampler,
    pub create_descriptor_set_layout: vk::CreateDescriptorSetLayout,
    pub destroy_descriptor_set_layout: vk::DestroyDescriptorSetLayout,
    pub create_pipeline_layout: vk::CreatePipelineLayout,
    pub destroy_pipeline_layout: vk::DestroyPipelineLayout,
    pub cmd_push_constants: vk::CmdPushConstants,
    pub get_buffer_device_address: vk::GetBufferDeviceAddress,
    pub get_descriptor_set_layout_size_ext: vk::GetDescriptorSetLayoutSizeEXT,
    pub get_descriptor_ext: vk::GetDescriptorEXT,
    pub cmd_bind_descriptor_buffers_ext: vk::CmdBindDescriptorBuffersEXT,
    pub cmd_set_descriptor_buffer_offsets_ext: vk::CmdSetDescriptorBufferOffsetsEXT,
    pub create_query_pool: vk::CreateQueryPool,
    pub destroy_query_pool: vk::DestroyQueryPool,
    pub reset_query_pool: vk::ResetQueryPool,
    pub cmd_begin_query: vk::CmdBeginQuery,
    pub cmd_end_query: vk::CmdEndQuery,
    pub get_query_pool_results: vk::GetQueryPoolResults,
    pub cmd_write_timestamp2: vk::CmdWriteTimestamp2,
    pub cmd_copy_buffer_to_image2: vk::CmdCopyBufferToImage2,
    pub cmd_copy_image_to_buffer2: vk::CmdCopyImageToBuffer2,
    pub cmd_draw_mesh_tasks_ext: vk::CmdDrawMeshTasksEXT,
    pub cmd_set_viewport_with_count: vk::CmdSetViewportWithCount,
    pub cmd_set_scissor_with_count: vk::CmdSetScissorWithCount,
    pub cmd_set_rasterization_samples_ext: vk::CmdSetRasterizationSamplesEXT,
    pub cmd_set_front_face: vk::CmdSetFrontFace,
    pub cmd_set_cull_mode: vk::CmdSetCullMode,
    pub cmd_set_depth_test_enable: vk::CmdSetDepthTestEnable,
    pub cmd_set_depth_compare_op: vk::CmdSetDepthCompareOp,
    pub cmd_set_depth_write_enable: vk::CmdSetDepthWriteEnable,
    pub cmd_dispatch: vk::CmdDispatch,
    pub cmd_dispatch_indirect: vk::CmdDispatchIndirect,
}

pub struct Device {
    fns: DeviceFunctions,
    handle: vk::Device,
}

impl Device {
    pub unsafe fn load(instance: &Instance, device: vk::Device) -> Result<Self, Error> {
        let load = |name: &'static [u8]| {
            let pfn = instance.get_device_proc_addr(device, name.as_ptr().cast());
            if pfn as usize == 0 {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            }
            Ok(pfn)
        };

        Ok(Self {
            fns: DeviceFunctions {
                destroy_device: std::mem::transmute(load(b"vkDestroyDevice\0")?),
                get_device_queue2: std::mem::transmute(load(b"vkGetDeviceQueue2\0")?),
                create_command_pool: std::mem::transmute(load(b"vkCreateCommandPool\0")?),
                reset_command_pool: std::mem::transmute(load(b"vkResetCommandPool\0")?),
                destroy_command_pool: std::mem::transmute(load(b"vkDestroyCommandPool\0")?),
                allocate_command_buffers: std::mem::transmute(load(b"vkAllocateCommandBuffers\0")?),
                free_command_buffers: std::mem::transmute(load(b"vkFreeCommandBuffers\0")?),
                begin_command_buffer: std::mem::transmute(load(b"vkBeginCommandBuffer\0")?),
                end_command_buffer: std::mem::transmute(load(b"vkEndCommandBuffer\0")?),
                queue_submit2: std::mem::transmute(load(b"vkQueueSubmit2\0")?),
                create_semaphore: std::mem::transmute(load(b"vkCreateSemaphore\0")?),
                destroy_semaphore: std::mem::transmute(load(b"vkDestroySemaphore\0")?),
                wait_semaphores: std::mem::transmute(load(b"vkWaitSemaphores\0")?),
                cmd_pipeline_barrier2: std::mem::transmute(load(b"vkCmdPipelineBarrier2\0")?),
                queue_wait_idle: std::mem::transmute(load(b"vkQueueWaitIdle\0")?),
                device_wait_idle: std::mem::transmute(load(b"vkDeviceWaitIdle\0")?),
                get_calibrated_timestamps_ext: std::mem::transmute(load(b"vkGetCalibratedTimestampsEXT\0")?),
                cmd_begin_rendering: std::mem::transmute(load(b"vkCmdBeginRendering\0")?),
                cmd_end_rendering: std::mem::transmute(load(b"vkCmdEndRendering\0")?),
                create_shaders_ext: std::mem::transmute(load(b"vkCreateShadersEXT\0")?),
                cmd_bind_shaders_ext: std::mem::transmute(load(b"vkCmdBindShadersEXT\0")?),
                destroy_shader_ext: std::mem::transmute(load(b"vkDestroyShaderEXT\0")?),
                allocate_memory: std::mem::transmute(load(b"vkAllocateMemory\0")?),
                free_memory: std::mem::transmute(load(b"vkFreeMemory\0")?),
                map_memory2_khr: std::mem::transmute(load(b"vkMapMemory2KHR\0")?),
                unmap_memory2_khr: std::mem::transmute(load(b"vkUnmapMemory2KHR\0")?),
                create_buffer: std::mem::transmute(load(b"vkCreateBuffer\0")?),
                destroy_buffer: std::mem::transmute(load(b"vkDestroyBuffer\0")?),
                create_image: std::mem::transmute(load(b"vkCreateImage\0")?),
                destroy_image: std::mem::transmute(load(b"vkDestroyImage\0")?),
                create_image_view: std::mem::transmute(load(b"vkCreateImageView\0")?),
                destroy_image_view: std::mem::transmute(load(b"vkDestroyImageView\0")?),
                get_device_buffer_memory_requirements: std::mem::transmute(load(b"vkGetDeviceBufferMemoryRequirements\0")?),
                get_device_image_memory_requirements: std::mem::transmute(load(b"vkGetDeviceImageMemoryRequirements\0")?),
                bind_buffer_memory2: std::mem::transmute(load(b"vkBindBufferMemory2\0")?),
                bind_image_memory2: std::mem::transmute(load(b"vkBindImageMemory2\0")?),
                create_sampler: std::mem::transmute(load(b"vkCreateSampler\0")?),
                destroy_sampler: std::mem::transmute(load(b"vkDestroySampler\0")?),
                create_descriptor_set_layout: std::mem::transmute(load(b"vkCreateDescriptorSetLayout\0")?),
                destroy_descriptor_set_layout: std::mem::transmute(load(b"vkDestroyDescriptorSetLayout\0")?),
                create_pipeline_layout: std::mem::transmute(load(b"vkCreatePipelineLayout\0")?),
                destroy_pipeline_layout: std::mem::transmute(load(b"vkDestroyPipelineLayout\0")?),
                cmd_push_constants: std::mem::transmute(load(b"vkCmdPushConstants\0")?),
                get_buffer_device_address: std::mem::transmute(load(b"vkGetBufferDeviceAddress\0")?),
                get_descriptor_set_layout_size_ext: std::mem::transmute(load(b"vkGetDescriptorSetLayoutSizeEXT\0")?),
                get_descriptor_ext: std::mem::transmute(load(b"vkGetDescriptorEXT\0")?),
                cmd_bind_descriptor_buffers_ext: std::mem::transmute(load(b"vkCmdBindDescriptorBuffersEXT\0")?),
                cmd_set_descriptor_buffer_offsets_ext: std::mem::transmute(load(b"vkCmdSetDescriptorBufferOffsetsEXT\0")?),
                create_query_pool: std::mem::transmute(load(b"vkCreateQueryPool\0")?),
                destroy_query_pool: std::mem::transmute(load(b"vkDestroyQueryPool\0")?),
                reset_query_pool: std::mem::transmute(load(b"vkResetQueryPool\0")?),
                cmd_begin_query: std::mem::transmute(load(b"vkCmdBeginQuery\0")?),
                cmd_end_query: std::mem::transmute(load(b"vkCmdEndQuery\0")?),
                get_query_pool_results: std::mem::transmute(load(b"vkGetQueryPoolResults\0")?),
                cmd_write_timestamp2: std::mem::transmute(load(b"vkCmdWriteTimestamp2\0")?),
                cmd_copy_buffer_to_image2: std::mem::transmute(load(b"vkCmdCopyBufferToImage2\0")?),
                cmd_copy_image_to_buffer2: std::mem::transmute(load(b"vkCmdCopyImageToBuffer2\0")?),
                cmd_draw_mesh_tasks_ext: std::mem::transmute(load(b"vkCmdDrawMeshTasksEXT\0")?),
                cmd_set_viewport_with_count: std::mem::transmute(load(b"vkCmdSetViewportWithCount\0")?),
                cmd_set_scissor_with_count: std::mem::transmute(load(b"vkCmdSetScissorWithCount\0")?),
                cmd_set_rasterization_samples_ext: std::mem::transmute(load(b"vkCmdSetRasterizationSamplesEXT\0")?),
                cmd_set_front_face: std::mem::transmute(load(b"vkCmdSetFrontFace\0")?),
                cmd_set_cull_mode: std::mem::transmute(load(b"vkCmdSetCullMode\0")?),
                cmd_set_depth_test_enable: std::mem::transmute(load(b"vkCmdSetDepthTestEnable\0")?),
                cmd_set_depth_compare_op: std::mem::transmute(load(b"vkCmdSetDepthCompareOp\0")?),
                cmd_set_depth_write_enable: std::mem::transmute(load(b"vkCmdSetDepthWriteEnable\0")?),
                cmd_dispatch: std::mem::transmute(load(b"vkCmdDispatch\0")?),
                cmd_dispatch_indirect: std::mem::transmute(load(b"vkCmdDispatchIndirect\0")?),
            },
            handle: device,
        })
    }

    #[must_use]
    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    #[must_use]
    pub fn fns(&self) -> &DeviceFunctions {
        &self.fns
    }

    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a logical device"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)"]
    pub unsafe fn destroy_device(&self) {
        (self.fns.destroy_device)(self.handle, std::ptr::null());
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Devices and Queues"]
    #[doc = "<br>"]
    #[doc = "**Description**: Get a queue handle from a device"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDeviceQueue2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)"]
    pub unsafe fn get_device_queue2(&self, p_queue_info: *const vk::DeviceQueueInfo2) -> vk::Queue {
        let mut p_queue = std::mem::MaybeUninit::uninit();
        (self.fns.get_device_queue2)(self.handle, p_queue_info, p_queue.as_mut_ptr());
        p_queue.assume_init()
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new command pool object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)"]
    pub unsafe fn create_command_pool(&self, p_create_info: *const vk::CommandPoolCreateInfo) -> Result<vk::CommandPool, Error> {
        let mut p_command_pool = std::mem::MaybeUninit::uninit();
        match (self.fns.create_command_pool)(self.handle, p_create_info, std::ptr::null(), p_command_pool.as_mut_ptr()) {
            vk::Result::Success => Ok(p_command_pool.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reset a command pool"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)"]
    pub unsafe fn reset_command_pool(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolResetFlags) -> Result<(), Error> {
        match (self.fns.reset_command_pool)(self.handle, command_pool, flags) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a command pool object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)"]
    pub unsafe fn destroy_command_pool(&self, command_pool: vk::CommandPool) {
        (self.fns.destroy_command_pool)(self.handle, command_pool, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Allocate command buffers from an existing command pool"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)"]
    pub unsafe fn allocate_command_buffers(&self, p_allocate_info: *const vk::CommandBufferAllocateInfo, p_command_buffers: *mut vk::CommandBuffer) -> Result<(), Error> {
        match (self.fns.allocate_command_buffers)(self.handle, p_allocate_info, p_command_buffers) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Free command buffers"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)"]
    pub unsafe fn free_command_buffers(&self, command_pool: vk::CommandPool, command_buffer_count: u32, p_command_buffers: *const vk::CommandBuffer) {
        (self.fns.free_command_buffers)(self.handle, command_pool, command_buffer_count, p_command_buffers);
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Start recording a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)"]
    pub unsafe fn begin_command_buffer(&self, command_buffer: vk::CommandBuffer, p_begin_info: *const vk::CommandBufferBeginInfo) -> Result<(), Error> {
        match (self.fns.begin_command_buffer)(command_buffer, p_begin_info) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Finish recording a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)"]
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<(), Error> {
        match (self.fns.end_command_buffer)(command_buffer) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Submits command buffers to a queue"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkQueueSubmit2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)"]
    pub unsafe fn queue_submit2(&self, queue: vk::Queue, submit_count: u32, p_submits: *const vk::SubmitInfo2, fence: vk::Fence) -> Result<(), Error> {
        match (self.fns.queue_submit2)(queue, submit_count, p_submits, fence) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new queue semaphore object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)"]
    pub unsafe fn create_semaphore(&self, p_create_info: *const vk::SemaphoreCreateInfo) -> Result<vk::Semaphore, Error> {
        let mut p_semaphore = std::mem::MaybeUninit::uninit();
        match (self.fns.create_semaphore)(self.handle, p_create_info, std::ptr::null(), p_semaphore.as_mut_ptr()) {
            vk::Result::Success => Ok(p_semaphore.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a semaphore object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)"]
    pub unsafe fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        (self.fns.destroy_semaphore)(self.handle, semaphore, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Wait for timeline semaphores on the host"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkWaitSemaphores`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)"]
    pub unsafe fn wait_semaphores(&self, p_wait_info: *const vk::SemaphoreWaitInfo, timeout: u64) -> Result<(), Error> {
        match (self.fns.wait_semaphores)(self.handle, p_wait_info, timeout) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Insert a memory dependency"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdPipelineBarrier2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)"]
    pub unsafe fn cmd_pipeline_barrier2(&self, command_buffer: vk::CommandBuffer, p_dependency_info: *const vk::DependencyInfo) {
        (self.fns.cmd_pipeline_barrier2)(command_buffer, p_dependency_info);
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Wait for a queue to become idle"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)"]
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> Result<(), Error> {
        match (self.fns.queue_wait_idle)(queue) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Wait for a device to become idle"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)"]
    pub unsafe fn device_wait_idle(&self) -> Result<(), Error> {
        match (self.fns.device_wait_idle)(self.handle) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query calibrated timestamps"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_calibrated_timestamps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetCalibratedTimestampsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)"]
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        timestamp_count: u32,
        p_timestamp_infos: *const vk::CalibratedTimestampInfoEXT,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> Result<(), Error> {
        match (self.fns.get_calibrated_timestamps_ext)(self.handle, timestamp_count, p_timestamp_infos, p_timestamps, p_max_deviation) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Render Pass"]
    #[doc = "<br>"]
    #[doc = "**Description**: Begin a dynamic render pass instance"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBeginRendering`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)"]
    pub unsafe fn cmd_begin_rendering(&self, command_buffer: vk::CommandBuffer, p_rendering_info: *const vk::RenderingInfo) {
        (self.fns.cmd_begin_rendering)(command_buffer, p_rendering_info);
    }

    #[inline]
    #[doc = "**Chapter**: Render Pass"]
    #[doc = "<br>"]
    #[doc = "**Description**: End a dynamic render pass instance"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdEndRendering`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)"]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.fns.cmd_end_rendering)(command_buffer);
    }

    #[inline]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create one or more new shaders"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html)"]
    pub unsafe fn create_shaders_ext(&self, create_info_count: u32, p_create_infos: *const vk::ShaderCreateInfoEXT, p_shaders: *mut vk::ShaderEXT) -> Result<(), Error> {
        match (self.fns.create_shaders_ext)(self.handle, create_info_count, p_create_infos, std::ptr::null(), p_shaders) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bind shader objects to a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBindShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html)"]
    pub unsafe fn cmd_bind_shaders_ext(&self, command_buffer: vk::CommandBuffer, stage_count: u32, p_stages: *const vk::ShaderStageFlagBits, p_shaders: *const vk::ShaderEXT) {
        (self.fns.cmd_bind_shaders_ext)(command_buffer, stage_count, p_stages, p_shaders);
    }

    #[inline]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a shader object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_shader_object`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyShaderEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html)"]
    pub unsafe fn destroy_shader_ext(&self, shader: vk::ShaderEXT) {
        (self.fns.destroy_shader_ext)(self.handle, shader, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Allocate device memory"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)"]
    pub unsafe fn allocate_memory(&self, p_allocate_info: *const vk::MemoryAllocateInfo) -> Result<vk::DeviceMemory, Error> {
        let mut p_memory = std::mem::MaybeUninit::uninit();
        match (self.fns.allocate_memory)(self.handle, p_allocate_info, std::ptr::null(), p_memory.as_mut_ptr()) {
            vk::Result::Success => Ok(p_memory.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Free device memory"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)"]
    pub unsafe fn free_memory(&self, memory: vk::DeviceMemory) {
        (self.fns.free_memory)(self.handle, memory, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Map a memory object into application address space"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkMapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory2KHR.html)"]
    pub unsafe fn map_memory2_khr(&self, p_memory_map_info: *const vk::MemoryMapInfoKHR) -> Result<*mut c_void, Error> {
        let mut pp_data = std::mem::MaybeUninit::uninit();
        match (self.fns.map_memory2_khr)(self.handle, p_memory_map_info, pp_data.as_mut_ptr()) {
            vk::Result::Success => Ok(pp_data.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Memory Allocation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Unmap a previously mapped memory object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_map_memory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_map_memory2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkUnmapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory2KHR.html)"]
    pub unsafe fn unmap_memory2_khr(&self, p_memory_unmap_info: *const vk::MemoryUnmapInfoKHR) -> Result<(), Error> {
        match (self.fns.unmap_memory2_khr)(self.handle, p_memory_unmap_info) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new buffer object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)"]
    pub unsafe fn create_buffer(&self, p_create_info: *const vk::BufferCreateInfo) -> Result<vk::Buffer, Error> {
        let mut p_buffer = std::mem::MaybeUninit::uninit();
        match (self.fns.create_buffer)(self.handle, p_create_info, std::ptr::null(), p_buffer.as_mut_ptr()) {
            vk::Result::Success => Ok(p_buffer.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a buffer object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)"]
    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer) {
        (self.fns.destroy_buffer)(self.handle, buffer, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new image object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)"]
    pub unsafe fn create_image(&self, p_create_info: *const vk::ImageCreateInfo) -> Result<vk::Image, Error> {
        let mut p_image = std::mem::MaybeUninit::uninit();
        match (self.fns.create_image)(self.handle, p_create_info, std::ptr::null(), p_image.as_mut_ptr()) {
            vk::Result::Success => Ok(p_image.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy an image object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyImage`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)"]
    pub unsafe fn destroy_image(&self, image: vk::Image) {
        (self.fns.destroy_image)(self.handle, image, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create an image view from an existing image"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)"]
    pub unsafe fn create_image_view(&self, p_create_info: *const vk::ImageViewCreateInfo) -> Result<vk::ImageView, Error> {
        let mut p_view = std::mem::MaybeUninit::uninit();
        match (self.fns.create_image_view)(self.handle, p_create_info, std::ptr::null(), p_view.as_mut_ptr()) {
            vk::Result::Success => Ok(p_view.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy an image view object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyImageView`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)"]
    pub unsafe fn destroy_image_view(&self, image_view: vk::ImageView) {
        (self.fns.destroy_image_view)(self.handle, image_view, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Returns the memory requirements for specified Vulkan object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDeviceBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)"]
    pub unsafe fn get_device_buffer_memory_requirements(&self, p_info: *const vk::DeviceBufferMemoryRequirements, p_memory_requirements: *mut vk::MemoryRequirements2) {
        (self.fns.get_device_buffer_memory_requirements)(self.handle, p_info, p_memory_requirements);
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Returns the memory requirements for specified Vulkan object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDeviceImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)"]
    pub unsafe fn get_device_image_memory_requirements(&self, p_info: *const vk::DeviceImageMemoryRequirements, p_memory_requirements: *mut vk::MemoryRequirements2) {
        (self.fns.get_device_image_memory_requirements)(self.handle, p_info, p_memory_requirements);
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bind device memory to buffer objects"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkBindBufferMemory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)"]
    pub unsafe fn bind_buffer_memory2(&self, bind_info_count: u32, p_bind_infos: *const vk::BindBufferMemoryInfo) -> Result<(), Error> {
        match (self.fns.bind_buffer_memory2)(self.handle, bind_info_count, p_bind_infos) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bind device memory to image objects"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkBindImageMemory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)"]
    pub unsafe fn bind_image_memory2(&self, bind_info_count: u32, p_bind_infos: *const vk::BindImageMemoryInfo) -> Result<(), Error> {
        match (self.fns.bind_image_memory2)(self.handle, bind_info_count, p_bind_infos) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Samplers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new sampler object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)"]
    pub unsafe fn create_sampler(&self, p_create_info: *const vk::SamplerCreateInfo) -> Result<vk::Sampler, Error> {
        let mut p_sampler = std::mem::MaybeUninit::uninit();
        match (self.fns.create_sampler)(self.handle, p_create_info, std::ptr::null(), p_sampler.as_mut_ptr()) {
            vk::Result::Success => Ok(p_sampler.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Samplers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a sampler object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroySampler`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)"]
    pub unsafe fn destroy_sampler(&self, sampler: vk::Sampler) {
        (self.fns.destroy_sampler)(self.handle, sampler, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new descriptor set layout"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)"]
    pub unsafe fn create_descriptor_set_layout(&self, p_create_info: *const vk::DescriptorSetLayoutCreateInfo) -> Result<vk::DescriptorSetLayout, Error> {
        let mut p_set_layout = std::mem::MaybeUninit::uninit();
        match (self.fns.create_descriptor_set_layout)(self.handle, p_create_info, std::ptr::null(), p_set_layout.as_mut_ptr()) {
            vk::Result::Success => Ok(p_set_layout.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a descriptor set layout object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)"]
    pub unsafe fn destroy_descriptor_set_layout(&self, descriptor_set_layout: vk::DescriptorSetLayout) {
        (self.fns.destroy_descriptor_set_layout)(self.handle, descriptor_set_layout, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Creates a new pipeline layout object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)"]
    pub unsafe fn create_pipeline_layout(&self, p_create_info: *const vk::PipelineLayoutCreateInfo) -> Result<vk::PipelineLayout, Error> {
        let mut p_pipeline_layout = std::mem::MaybeUninit::uninit();
        match (self.fns.create_pipeline_layout)(self.handle, p_create_info, std::ptr::null(), p_pipeline_layout.as_mut_ptr()) {
            vk::Result::Success => Ok(p_pipeline_layout.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a pipeline layout object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)"]
    pub unsafe fn destroy_pipeline_layout(&self, pipeline_layout: vk::PipelineLayout) {
        (self.fns.destroy_pipeline_layout)(self.handle, pipeline_layout, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Update the values of push constants"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)"]
    pub unsafe fn cmd_push_constants(&self, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, stage_flags: vk::ShaderStageFlags, offset: u32, size: u32, p_values: *const c_void) {
        (self.fns.cmd_push_constants)(command_buffer, layout, stage_flags, offset, size, p_values);
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query an address of a buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetBufferDeviceAddress`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)"]
    pub unsafe fn get_buffer_device_address(&self, p_info: *const vk::BufferDeviceAddressInfo) -> vk::DeviceAddress {
        (self.fns.get_buffer_device_address)(self.handle, p_info)
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Get the size of a descriptor set layout in memory"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDescriptorSetLayoutSizeEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html)"]
    pub unsafe fn get_descriptor_set_layout_size_ext(&self, layout: vk::DescriptorSetLayout) -> vk::DeviceSize {
        let mut p_layout_size_in_bytes = std::mem::MaybeUninit::uninit();
        (self.fns.get_descriptor_set_layout_size_ext)(self.handle, layout, p_layout_size_in_bytes.as_mut_ptr());
        p_layout_size_in_bytes.assume_init()
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: To get a descriptor to place in a buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDescriptorEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html)"]
    pub unsafe fn get_descriptor_ext(&self, p_descriptor_info: *const vk::DescriptorGetInfoEXT, data_size: usize, p_descriptor: *mut c_void) {
        (self.fns.get_descriptor_ext)(self.handle, p_descriptor_info, data_size, p_descriptor);
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Binding descriptor buffers to a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBindDescriptorBuffersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html)"]
    pub unsafe fn cmd_bind_descriptor_buffers_ext(&self, command_buffer: vk::CommandBuffer, buffer_count: u32, p_binding_infos: *const vk::DescriptorBufferBindingInfoEXT) {
        (self.fns.cmd_bind_descriptor_buffers_ext)(command_buffer, buffer_count, p_binding_infos);
    }

    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Setting descriptor buffer offsets in a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetDescriptorBufferOffsetsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html)"]
    pub unsafe fn cmd_set_descriptor_buffer_offsets_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const vk::DeviceSize,
    ) {
        (self.fns.cmd_set_descriptor_buffer_offsets_ext)(command_buffer, pipeline_bind_point, layout, first_set, set_count, p_buffer_indices, p_offsets);
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new query pool object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)"]
    pub unsafe fn create_query_pool(&self, p_create_info: *const vk::QueryPoolCreateInfo) -> Result<vk::QueryPool, Error> {
        let mut p_query_pool = std::mem::MaybeUninit::uninit();
        match (self.fns.create_query_pool)(self.handle, p_create_info, std::ptr::null(), p_query_pool.as_mut_ptr()) {
            vk::Result::Success => Ok(p_query_pool.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a query pool object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)"]
    pub unsafe fn destroy_query_pool(&self, query_pool: vk::QueryPool) {
        (self.fns.destroy_query_pool)(self.handle, query_pool, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reset queries in a query pool"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)"]
    pub unsafe fn reset_query_pool(&self, query_pool: vk::QueryPool, first_query: u32, query_count: u32) {
        (self.fns.reset_query_pool)(self.handle, query_pool, first_query, query_count);
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Begin a query"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)"]
    pub unsafe fn cmd_begin_query(&self, command_buffer: vk::CommandBuffer, query_pool: vk::QueryPool, query: u32, flags: vk::QueryControlFlags) {
        (self.fns.cmd_begin_query)(command_buffer, query_pool, query, flags);
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Ends a query"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)"]
    pub unsafe fn cmd_end_query(&self, command_buffer: vk::CommandBuffer, query_pool: vk::QueryPool, query: u32) {
        (self.fns.cmd_end_query)(command_buffer, query_pool, query);
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Copy results of queries in a query pool to a host memory region"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)"]
    pub unsafe fn get_query_pool_results(
        &self,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) -> Result<(), Error> {
        match (self.fns.get_query_pool_results)(self.handle, query_pool, first_query, query_count, data_size, p_data, stride, flags) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Queries"]
    #[doc = "<br>"]
    #[doc = "**Description**: Write a device timestamp into a query object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdWriteTimestamp2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)"]
    pub unsafe fn cmd_write_timestamp2(&self, command_buffer: vk::CommandBuffer, stage: vk::PipelineStageFlags2, query_pool: vk::QueryPool, query: u32) {
        (self.fns.cmd_write_timestamp2)(command_buffer, stage, query_pool, query);
    }

    #[inline]
    #[doc = "**Chapter**: Copy Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Copy data from a buffer into an image"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdCopyBufferToImage2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)"]
    pub unsafe fn cmd_copy_buffer_to_image2(&self, command_buffer: vk::CommandBuffer, p_copy_buffer_to_image_info: *const vk::CopyBufferToImageInfo2) {
        (self.fns.cmd_copy_buffer_to_image2)(command_buffer, p_copy_buffer_to_image_info);
    }

    #[inline]
    #[doc = "**Chapter**: Copy Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Copy image data into a buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdCopyImageToBuffer2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)"]
    pub unsafe fn cmd_copy_image_to_buffer2(&self, command_buffer: vk::CommandBuffer, p_copy_image_to_buffer_info: *const vk::CopyImageToBufferInfo2) {
        (self.fns.cmd_copy_image_to_buffer2)(command_buffer, p_copy_image_to_buffer_info);
    }

    #[inline]
    #[doc = "**Chapter**: Drawing Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Draw mesh task work items"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdDrawMeshTasksEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)"]
    pub unsafe fn cmd_draw_mesh_tasks_ext(&self, command_buffer: vk::CommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        (self.fns.cmd_draw_mesh_tasks_ext)(command_buffer, group_count_x, group_count_y, group_count_z);
    }

    #[inline]
    #[doc = "**Chapter**: Fixed-Function Vertex Post-Processing"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set the viewport count and viewports dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetViewportWithCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)"]
    pub unsafe fn cmd_set_viewport_with_count(&self, command_buffer: vk::CommandBuffer, viewport_count: u32, p_viewports: *const vk::Viewport) {
        (self.fns.cmd_set_viewport_with_count)(command_buffer, viewport_count, p_viewports);
    }

    #[inline]
    #[doc = "**Chapter**: Fixed-Function Vertex Post-Processing"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set the scissor count and scissor rectangular bounds dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetScissorWithCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)"]
    pub unsafe fn cmd_set_scissor_with_count(&self, command_buffer: vk::CommandBuffer, scissor_count: u32, p_scissors: *const vk::Rect2D) {
        (self.fns.cmd_set_scissor_with_count)(command_buffer, scissor_count, p_scissors);
    }

    #[inline]
    #[doc = "**Chapter**: Rasterization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Specify the rasterization samples dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetRasterizationSamplesEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)"]
    pub unsafe fn cmd_set_rasterization_samples_ext(&self, command_buffer: vk::CommandBuffer, rasterization_samples: vk::SampleCountFlagBits) {
        (self.fns.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples);
    }

    #[inline]
    #[doc = "**Chapter**: Rasterization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set front face orientation dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)"]
    pub unsafe fn cmd_set_front_face(&self, command_buffer: vk::CommandBuffer, front_face: vk::FrontFace) {
        (self.fns.cmd_set_front_face)(command_buffer, front_face);
    }

    #[inline]
    #[doc = "**Chapter**: Rasterization"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set cull mode dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetCullMode`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)"]
    pub unsafe fn cmd_set_cull_mode(&self, command_buffer: vk::CommandBuffer, cull_mode: vk::CullModeFlags) {
        (self.fns.cmd_set_cull_mode)(command_buffer, cull_mode);
    }

    #[inline]
    #[doc = "**Chapter**: Fragment Operations"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set depth test enable dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetDepthTestEnable`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)"]
    pub unsafe fn cmd_set_depth_test_enable(&self, command_buffer: vk::CommandBuffer, depth_test_enable: vk::Bool32) {
        (self.fns.cmd_set_depth_test_enable)(command_buffer, depth_test_enable);
    }

    #[inline]
    #[doc = "**Chapter**: Fragment Operations"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set depth comparison operator dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetDepthCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)"]
    pub unsafe fn cmd_set_depth_compare_op(&self, command_buffer: vk::CommandBuffer, depth_compare_op: vk::CompareOp) {
        (self.fns.cmd_set_depth_compare_op)(command_buffer, depth_compare_op);
    }

    #[inline]
    #[doc = "**Chapter**: Fragment Operations"]
    #[doc = "<br>"]
    #[doc = "**Description**: Set depth write enable dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetDepthWriteEnable`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)"]
    pub unsafe fn cmd_set_depth_write_enable(&self, command_buffer: vk::CommandBuffer, depth_write_enable: vk::Bool32) {
        (self.fns.cmd_set_depth_write_enable)(command_buffer, depth_write_enable);
    }

    #[inline]
    #[doc = "**Chapter**: Dispatching Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Dispatch compute work items"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)"]
    pub unsafe fn cmd_dispatch(&self, command_buffer: vk::CommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        (self.fns.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z);
    }

    #[inline]
    #[doc = "**Chapter**: Dispatching Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Dispatch compute work items with indirect parameters"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)"]
    pub unsafe fn cmd_dispatch_indirect(&self, command_buffer: vk::CommandBuffer, buffer: vk::Buffer, offset: vk::DeviceSize) {
        (self.fns.cmd_dispatch_indirect)(command_buffer, buffer, offset);
    }
}
