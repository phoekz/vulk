//
// Imports
//

use super::vk;
use std::ffi::{c_char, c_void};
use thiserror::Error;

//
// Error
//

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load dynamic library {0}")]
    LibraryLoad(std::borrow::Cow<'static, str>),
    #[error("failed to load function {0}")]
    FunctionLoad(std::borrow::Cow<'static, str>),
    #[error("vulkan error {0:?}")]
    Vulkan(vk::Result),
}

//
// Loader
//

pub struct LoaderFunctions {
    create_instance: vk::CreateInstance,
    get_instance_proc_addr: vk::GetInstanceProcAddr,

    _loader: std::sync::Arc<libloading::Library>,
}

impl LoaderFunctions {
    pub unsafe fn load() -> Result<Self, Error> {
        // Only Windows and Linux are supported.
        #[cfg(windows)]
        const VULKAN_LIB_PATH: &str = "vulkan-1.dll";
        #[cfg(unix)]
        const VULKAN_LIB_PATH: &str = "libvulkan.so.1";

        // Load the loader.
        let loader = libloading::Library::new(VULKAN_LIB_PATH)
            .map_err(|_| Error::LibraryLoad(std::borrow::Cow::Borrowed(VULKAN_LIB_PATH)))
            .map(std::sync::Arc::new)?;

        // Load functions.
        let load = |name: &'static [u8]| {
            let pfn = if let Ok(symbol) = loader.get::<*const c_void>(name) {
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
            create_instance: std::mem::transmute(load(b"vkCreateInstance\0")?),
            get_instance_proc_addr: std::mem::transmute(load(b"vkGetInstanceProcAddr\0")?),

            _loader: loader,
        })
    }

    #[inline]
    #[doc = "Description: Create a new Vulkan instance"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)"]
    pub unsafe fn create_instance(&self, p_create_info: *const vk::InstanceCreateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::Instance, Error> {
        let mut p_instance = std::mem::MaybeUninit::uninit();
        match (self.create_instance)(p_create_info, p_allocator, p_instance.as_mut_ptr()) {
            vk::Result::Success => Ok(p_instance.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[must_use]
    #[inline]
    #[doc = "Description: Return a function pointer for a command"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)"]
    pub unsafe fn get_instance_proc_addr(&self, instance: vk::Instance, p_name: *const c_char) -> vk::PfnVoidFunction {
        (self.get_instance_proc_addr)(instance, p_name)
    }
}

//
// Instance
//

pub struct InstanceFunctions {
    destroy_instance: vk::DestroyInstance,
    enumerate_physical_devices: vk::EnumeratePhysicalDevices,
    get_device_proc_addr: vk::GetDeviceProcAddr,
    create_device: vk::CreateDevice,
    get_physical_device_properties2: vk::GetPhysicalDeviceProperties2,
    get_physical_device_queue_family_properties2: vk::GetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2: vk::GetPhysicalDeviceMemoryProperties2,
    create_debug_utils_messenger_ext: vk::CreateDebugUtilsMessengerEXT,
    destroy_debug_utils_messenger_ext: vk::DestroyDebugUtilsMessengerEXT,
}

impl InstanceFunctions {
    pub unsafe fn load(loader: &LoaderFunctions, instance: vk::Instance) -> Result<Self, Error> {
        let load = |name: &'static [u8]| {
            let pfn = loader.get_instance_proc_addr(instance, name.as_ptr().cast());
            if pfn as usize == 0 {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            }
            Ok(pfn)
        };

        Ok(Self {
            destroy_instance: std::mem::transmute(load(b"vkDestroyInstance\0")?),
            enumerate_physical_devices: std::mem::transmute(load(b"vkEnumeratePhysicalDevices\0")?),
            get_device_proc_addr: std::mem::transmute(load(b"vkGetDeviceProcAddr\0")?),
            create_device: std::mem::transmute(load(b"vkCreateDevice\0")?),
            get_physical_device_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceProperties2\0")?),
            get_physical_device_queue_family_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceQueueFamilyProperties2\0")?),
            get_physical_device_memory_properties2: std::mem::transmute(load(b"vkGetPhysicalDeviceMemoryProperties2\0")?),
            create_debug_utils_messenger_ext: std::mem::transmute(load(b"vkCreateDebugUtilsMessengerEXT\0")?),
            destroy_debug_utils_messenger_ext: std::mem::transmute(load(b"vkDestroyDebugUtilsMessengerEXT\0")?),
        })
    }

    #[inline]
    #[doc = "Description: Destroy an instance of Vulkan"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyInstance`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)"]
    pub unsafe fn destroy_instance(&self, instance: vk::Instance, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_instance)(instance, p_allocator);
    }

    #[inline]
    #[doc = "Description: Enumerates the physical devices accessible to a Vulkan instance"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)"]
    pub unsafe fn enumerate_physical_devices(&self, instance: vk::Instance, p_physical_device_count: *mut u32, p_physical_devices: *mut vk::PhysicalDevice) -> Result<(), Error> {
        match (self.enumerate_physical_devices)(instance, p_physical_device_count, p_physical_devices) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[must_use]
    #[inline]
    #[doc = "Description: Return a function pointer for a command"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetDeviceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)"]
    pub unsafe fn get_device_proc_addr(&self, device: vk::Device, p_name: *const c_char) -> vk::PfnVoidFunction {
        (self.get_device_proc_addr)(device, p_name)
    }

    #[inline]
    #[doc = "Description: Create a new device instance"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)"]
    pub unsafe fn create_device(&self, physical_device: vk::PhysicalDevice, p_create_info: *const vk::DeviceCreateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::Device, Error> {
        let mut p_device = std::mem::MaybeUninit::uninit();
        match (self.create_device)(physical_device, p_create_info, p_allocator, p_device.as_mut_ptr()) {
            vk::Result::Success => Ok(p_device.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Returns properties of a physical device"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetPhysicalDeviceProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)"]
    pub unsafe fn get_physical_device_properties2(&self, physical_device: vk::PhysicalDevice, p_properties: *mut vk::PhysicalDeviceProperties2) {
        (self.get_physical_device_properties2)(physical_device, p_properties);
    }

    #[inline]
    #[doc = "Description: Reports properties of the queues of the specified physical device"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)"]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut vk::QueueFamilyProperties2,
    ) {
        (self.get_physical_device_queue_family_properties2)(physical_device, p_queue_family_property_count, p_queue_family_properties);
    }

    #[inline]
    #[doc = "Description: Reports memory information for the specified physical device"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetPhysicalDeviceMemoryProperties2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)"]
    pub unsafe fn get_physical_device_memory_properties2(&self, physical_device: vk::PhysicalDevice, p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2) {
        (self.get_physical_device_memory_properties2)(physical_device, p_memory_properties);
    }

    #[inline]
    #[doc = "Description: Create a debug messenger object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)"]
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: vk::Instance,
        p_create_info: *const vk::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
    ) -> Result<vk::DebugUtilsMessengerEXT, Error> {
        let mut p_messenger = std::mem::MaybeUninit::uninit();
        match (self.create_debug_utils_messenger_ext)(instance, p_create_info, p_allocator, p_messenger.as_mut_ptr()) {
            vk::Result::Success => Ok(p_messenger.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a debug messenger object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyDebugUtilsMessengerEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)"]
    pub unsafe fn destroy_debug_utils_messenger_ext(&self, instance: vk::Instance, messenger: vk::DebugUtilsMessengerEXT, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_debug_utils_messenger_ext)(instance, messenger, p_allocator);
    }
}

//
// Device
//

pub struct DeviceFunctions {
    destroy_device: vk::DestroyDevice,
    queue_wait_idle: vk::QueueWaitIdle,
    device_wait_idle: vk::DeviceWaitIdle,
    allocate_memory: vk::AllocateMemory,
    free_memory: vk::FreeMemory,
    create_semaphore: vk::CreateSemaphore,
    destroy_semaphore: vk::DestroySemaphore,
    create_buffer: vk::CreateBuffer,
    destroy_buffer: vk::DestroyBuffer,
    create_pipeline_layout: vk::CreatePipelineLayout,
    destroy_pipeline_layout: vk::DestroyPipelineLayout,
    create_descriptor_set_layout: vk::CreateDescriptorSetLayout,
    destroy_descriptor_set_layout: vk::DestroyDescriptorSetLayout,
    create_command_pool: vk::CreateCommandPool,
    destroy_command_pool: vk::DestroyCommandPool,
    reset_command_pool: vk::ResetCommandPool,
    allocate_command_buffers: vk::AllocateCommandBuffers,
    free_command_buffers: vk::FreeCommandBuffers,
    begin_command_buffer: vk::BeginCommandBuffer,
    end_command_buffer: vk::EndCommandBuffer,
    cmd_dispatch: vk::CmdDispatch,
    bind_buffer_memory2: vk::BindBufferMemory2,
    get_device_buffer_memory_requirements: vk::GetDeviceBufferMemoryRequirements,
    get_device_queue2: vk::GetDeviceQueue2,
    wait_semaphores: vk::WaitSemaphores,
    get_buffer_device_address: vk::GetBufferDeviceAddress,
    queue_submit2: vk::QueueSubmit2,
    get_descriptor_set_layout_size_ext: vk::GetDescriptorSetLayoutSizeEXT,
    get_descriptor_ext: vk::GetDescriptorEXT,
    cmd_bind_descriptor_buffers_ext: vk::CmdBindDescriptorBuffersEXT,
    cmd_set_descriptor_buffer_offsets_ext: vk::CmdSetDescriptorBufferOffsetsEXT,
    map_memory2_khr: vk::MapMemory2KHR,
    unmap_memory2_khr: vk::UnmapMemory2KHR,
    create_shaders_ext: vk::CreateShadersEXT,
    destroy_shader_ext: vk::DestroyShaderEXT,
    cmd_bind_shaders_ext: vk::CmdBindShadersEXT,
}

impl DeviceFunctions {
    pub unsafe fn load(instance: &InstanceFunctions, device: vk::Device) -> Result<Self, Error> {
        let load = |name: &'static [u8]| {
            let pfn = instance.get_device_proc_addr(device, name.as_ptr().cast());
            if pfn as usize == 0 {
                return Err(Error::FunctionLoad(String::from_utf8_lossy(name)));
            }
            Ok(pfn)
        };

        Ok(Self {
            destroy_device: std::mem::transmute(load(b"vkDestroyDevice\0")?),
            queue_wait_idle: std::mem::transmute(load(b"vkQueueWaitIdle\0")?),
            device_wait_idle: std::mem::transmute(load(b"vkDeviceWaitIdle\0")?),
            allocate_memory: std::mem::transmute(load(b"vkAllocateMemory\0")?),
            free_memory: std::mem::transmute(load(b"vkFreeMemory\0")?),
            create_semaphore: std::mem::transmute(load(b"vkCreateSemaphore\0")?),
            destroy_semaphore: std::mem::transmute(load(b"vkDestroySemaphore\0")?),
            create_buffer: std::mem::transmute(load(b"vkCreateBuffer\0")?),
            destroy_buffer: std::mem::transmute(load(b"vkDestroyBuffer\0")?),
            create_pipeline_layout: std::mem::transmute(load(b"vkCreatePipelineLayout\0")?),
            destroy_pipeline_layout: std::mem::transmute(load(b"vkDestroyPipelineLayout\0")?),
            create_descriptor_set_layout: std::mem::transmute(load(b"vkCreateDescriptorSetLayout\0")?),
            destroy_descriptor_set_layout: std::mem::transmute(load(b"vkDestroyDescriptorSetLayout\0")?),
            create_command_pool: std::mem::transmute(load(b"vkCreateCommandPool\0")?),
            destroy_command_pool: std::mem::transmute(load(b"vkDestroyCommandPool\0")?),
            reset_command_pool: std::mem::transmute(load(b"vkResetCommandPool\0")?),
            allocate_command_buffers: std::mem::transmute(load(b"vkAllocateCommandBuffers\0")?),
            free_command_buffers: std::mem::transmute(load(b"vkFreeCommandBuffers\0")?),
            begin_command_buffer: std::mem::transmute(load(b"vkBeginCommandBuffer\0")?),
            end_command_buffer: std::mem::transmute(load(b"vkEndCommandBuffer\0")?),
            cmd_dispatch: std::mem::transmute(load(b"vkCmdDispatch\0")?),
            bind_buffer_memory2: std::mem::transmute(load(b"vkBindBufferMemory2\0")?),
            get_device_buffer_memory_requirements: std::mem::transmute(load(b"vkGetDeviceBufferMemoryRequirements\0")?),
            get_device_queue2: std::mem::transmute(load(b"vkGetDeviceQueue2\0")?),
            wait_semaphores: std::mem::transmute(load(b"vkWaitSemaphores\0")?),
            get_buffer_device_address: std::mem::transmute(load(b"vkGetBufferDeviceAddress\0")?),
            queue_submit2: std::mem::transmute(load(b"vkQueueSubmit2\0")?),
            get_descriptor_set_layout_size_ext: std::mem::transmute(load(b"vkGetDescriptorSetLayoutSizeEXT\0")?),
            get_descriptor_ext: std::mem::transmute(load(b"vkGetDescriptorEXT\0")?),
            cmd_bind_descriptor_buffers_ext: std::mem::transmute(load(b"vkCmdBindDescriptorBuffersEXT\0")?),
            cmd_set_descriptor_buffer_offsets_ext: std::mem::transmute(load(b"vkCmdSetDescriptorBufferOffsetsEXT\0")?),
            map_memory2_khr: std::mem::transmute(load(b"vkMapMemory2KHR\0")?),
            unmap_memory2_khr: std::mem::transmute(load(b"vkUnmapMemory2KHR\0")?),
            create_shaders_ext: std::mem::transmute(load(b"vkCreateShadersEXT\0")?),
            destroy_shader_ext: std::mem::transmute(load(b"vkDestroyShaderEXT\0")?),
            cmd_bind_shaders_ext: std::mem::transmute(load(b"vkCmdBindShadersEXT\0")?),
        })
    }

    #[inline]
    #[doc = "Description: Destroy a logical device"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyDevice`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)"]
    pub unsafe fn destroy_device(&self, device: vk::Device, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_device)(device, p_allocator);
    }

    #[inline]
    #[doc = "Description: Wait for a queue to become idle"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)"]
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> Result<(), Error> {
        match (self.queue_wait_idle)(queue) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Wait for a device to become idle"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)"]
    pub unsafe fn device_wait_idle(&self, device: vk::Device) -> Result<(), Error> {
        match (self.device_wait_idle)(device) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Allocate device memory"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)"]
    pub unsafe fn allocate_memory(&self, device: vk::Device, p_allocate_info: *const vk::MemoryAllocateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::DeviceMemory, Error> {
        let mut p_memory = std::mem::MaybeUninit::uninit();
        match (self.allocate_memory)(device, p_allocate_info, p_allocator, p_memory.as_mut_ptr()) {
            vk::Result::Success => Ok(p_memory.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Free device memory"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkFreeMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)"]
    pub unsafe fn free_memory(&self, device: vk::Device, memory: vk::DeviceMemory, p_allocator: *const vk::AllocationCallbacks) {
        (self.free_memory)(device, memory, p_allocator);
    }

    #[inline]
    #[doc = "Description: Create a new queue semaphore object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)"]
    pub unsafe fn create_semaphore(&self, device: vk::Device, p_create_info: *const vk::SemaphoreCreateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::Semaphore, Error> {
        let mut p_semaphore = std::mem::MaybeUninit::uninit();
        match (self.create_semaphore)(device, p_create_info, p_allocator, p_semaphore.as_mut_ptr()) {
            vk::Result::Success => Ok(p_semaphore.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a semaphore object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroySemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)"]
    pub unsafe fn destroy_semaphore(&self, device: vk::Device, semaphore: vk::Semaphore, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_semaphore)(device, semaphore, p_allocator);
    }

    #[inline]
    #[doc = "Description: Create a new buffer object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)"]
    pub unsafe fn create_buffer(&self, device: vk::Device, p_create_info: *const vk::BufferCreateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::Buffer, Error> {
        let mut p_buffer = std::mem::MaybeUninit::uninit();
        match (self.create_buffer)(device, p_create_info, p_allocator, p_buffer.as_mut_ptr()) {
            vk::Result::Success => Ok(p_buffer.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a buffer object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)"]
    pub unsafe fn destroy_buffer(&self, device: vk::Device, buffer: vk::Buffer, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_buffer)(device, buffer, p_allocator);
    }

    #[inline]
    #[doc = "Description: Creates a new pipeline layout object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)"]
    pub unsafe fn create_pipeline_layout(
        &self,
        device: vk::Device,
        p_create_info: *const vk::PipelineLayoutCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
    ) -> Result<vk::PipelineLayout, Error> {
        let mut p_pipeline_layout = std::mem::MaybeUninit::uninit();
        match (self.create_pipeline_layout)(device, p_create_info, p_allocator, p_pipeline_layout.as_mut_ptr()) {
            vk::Result::Success => Ok(p_pipeline_layout.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a pipeline layout object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)"]
    pub unsafe fn destroy_pipeline_layout(&self, device: vk::Device, pipeline_layout: vk::PipelineLayout, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_pipeline_layout)(device, pipeline_layout, p_allocator);
    }

    #[inline]
    #[doc = "Description: Create a new descriptor set layout"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)"]
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: vk::Device,
        p_create_info: *const vk::DescriptorSetLayoutCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
    ) -> Result<vk::DescriptorSetLayout, Error> {
        let mut p_set_layout = std::mem::MaybeUninit::uninit();
        match (self.create_descriptor_set_layout)(device, p_create_info, p_allocator, p_set_layout.as_mut_ptr()) {
            vk::Result::Success => Ok(p_set_layout.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a descriptor set layout object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)"]
    pub unsafe fn destroy_descriptor_set_layout(&self, device: vk::Device, descriptor_set_layout: vk::DescriptorSetLayout, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_descriptor_set_layout)(device, descriptor_set_layout, p_allocator);
    }

    #[inline]
    #[doc = "Description: Create a new command pool object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)"]
    pub unsafe fn create_command_pool(&self, device: vk::Device, p_create_info: *const vk::CommandPoolCreateInfo, p_allocator: *const vk::AllocationCallbacks) -> Result<vk::CommandPool, Error> {
        let mut p_command_pool = std::mem::MaybeUninit::uninit();
        match (self.create_command_pool)(device, p_create_info, p_allocator, p_command_pool.as_mut_ptr()) {
            vk::Result::Success => Ok(p_command_pool.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a command pool object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)"]
    pub unsafe fn destroy_command_pool(&self, device: vk::Device, command_pool: vk::CommandPool, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_command_pool)(device, command_pool, p_allocator);
    }

    #[inline]
    #[doc = "Description: Reset a command pool"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)"]
    pub unsafe fn reset_command_pool(&self, device: vk::Device, command_pool: vk::CommandPool, flags: vk::CommandPoolResetFlags) -> Result<(), Error> {
        match (self.reset_command_pool)(device, command_pool, flags) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Allocate command buffers from an existing command pool"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)"]
    pub unsafe fn allocate_command_buffers(&self, device: vk::Device, p_allocate_info: *const vk::CommandBufferAllocateInfo, p_command_buffers: *mut vk::CommandBuffer) -> Result<(), Error> {
        match (self.allocate_command_buffers)(device, p_allocate_info, p_command_buffers) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Free command buffers"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkFreeCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)"]
    pub unsafe fn free_command_buffers(&self, device: vk::Device, command_pool: vk::CommandPool, command_buffer_count: u32, p_command_buffers: *const vk::CommandBuffer) {
        (self.free_command_buffers)(device, command_pool, command_buffer_count, p_command_buffers);
    }

    #[inline]
    #[doc = "Description: Start recording a command buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)"]
    pub unsafe fn begin_command_buffer(&self, command_buffer: vk::CommandBuffer, p_begin_info: *const vk::CommandBufferBeginInfo) -> Result<(), Error> {
        match (self.begin_command_buffer)(command_buffer, p_begin_info) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Finish recording a command buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)"]
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<(), Error> {
        match (self.end_command_buffer)(command_buffer) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Dispatch compute work items"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)"]
    pub unsafe fn cmd_dispatch(&self, command_buffer: vk::CommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        (self.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z);
    }

    #[inline]
    #[doc = "Description: Bind device memory to buffer objects"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkBindBufferMemory2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)"]
    pub unsafe fn bind_buffer_memory2(&self, device: vk::Device, bind_info_count: u32, p_bind_infos: *const vk::BindBufferMemoryInfo) -> Result<(), Error> {
        match (self.bind_buffer_memory2)(device, bind_info_count, p_bind_infos) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Returns the memory requirements for specified Vulkan object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetDeviceBufferMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)"]
    pub unsafe fn get_device_buffer_memory_requirements(&self, device: vk::Device, p_info: *const vk::DeviceBufferMemoryRequirements, p_memory_requirements: *mut vk::MemoryRequirements2) {
        (self.get_device_buffer_memory_requirements)(device, p_info, p_memory_requirements);
    }

    #[inline]
    #[doc = "Description: Get a queue handle from a device"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetDeviceQueue2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)"]
    pub unsafe fn get_device_queue2(&self, device: vk::Device, p_queue_info: *const vk::DeviceQueueInfo2, p_queue: *mut vk::Queue) {
        (self.get_device_queue2)(device, p_queue_info, p_queue);
    }

    #[inline]
    #[doc = "Description: Wait for timeline semaphores on the host"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkWaitSemaphores`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)"]
    pub unsafe fn wait_semaphores(&self, device: vk::Device, p_wait_info: *const vk::SemaphoreWaitInfo, timeout: u64) -> Result<(), Error> {
        match (self.wait_semaphores)(device, p_wait_info, timeout) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[must_use]
    #[inline]
    #[doc = "Description: Query an address of a buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetBufferDeviceAddress`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)"]
    pub unsafe fn get_buffer_device_address(&self, device: vk::Device, p_info: *const vk::BufferDeviceAddressInfo) -> vk::DeviceAddress {
        (self.get_buffer_device_address)(device, p_info)
    }

    #[inline]
    #[doc = "Description: Submits command buffers to a queue"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkQueueSubmit2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)"]
    pub unsafe fn queue_submit2(&self, queue: vk::Queue, submit_count: u32, p_submits: *const vk::SubmitInfo2, fence: vk::Fence) -> Result<(), Error> {
        match (self.queue_submit2)(queue, submit_count, p_submits, fence) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Get the size of a descriptor set layout in memory"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetDescriptorSetLayoutSizeEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html)"]
    pub unsafe fn get_descriptor_set_layout_size_ext(&self, device: vk::Device, layout: vk::DescriptorSetLayout, p_layout_size_in_bytes: *mut vk::DeviceSize) {
        (self.get_descriptor_set_layout_size_ext)(device, layout, p_layout_size_in_bytes);
    }

    #[inline]
    #[doc = "Description: To get a descriptor to place in a buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkGetDescriptorEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html)"]
    pub unsafe fn get_descriptor_ext(&self, device: vk::Device, p_descriptor_info: *const vk::DescriptorGetInfoEXT, data_size: usize, p_descriptor: *mut c_void) {
        (self.get_descriptor_ext)(device, p_descriptor_info, data_size, p_descriptor);
    }

    #[inline]
    #[doc = "Description: Binding descriptor buffers to a command buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCmdBindDescriptorBuffersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html)"]
    pub unsafe fn cmd_bind_descriptor_buffers_ext(&self, command_buffer: vk::CommandBuffer, buffer_count: u32, p_binding_infos: *const vk::DescriptorBufferBindingInfoEXT) {
        (self.cmd_bind_descriptor_buffers_ext)(command_buffer, buffer_count, p_binding_infos);
    }

    #[inline]
    #[doc = "Description: Setting descriptor buffer offsets in a command buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCmdSetDescriptorBufferOffsetsEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html)"]
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
        (self.cmd_set_descriptor_buffer_offsets_ext)(command_buffer, pipeline_bind_point, layout, first_set, set_count, p_buffer_indices, p_offsets);
    }

    #[inline]
    #[doc = "Description: Map a memory object into application address space"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkMapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory2KHR.html)"]
    pub unsafe fn map_memory2_khr(&self, device: vk::Device, p_memory_map_info: *const vk::MemoryMapInfoKHR, pp_data: *mut *mut c_void) -> Result<(), Error> {
        match (self.map_memory2_khr)(device, p_memory_map_info, pp_data) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Unmap a previously mapped memory object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkUnmapMemory2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory2KHR.html)"]
    pub unsafe fn unmap_memory2_khr(&self, device: vk::Device, p_memory_unmap_info: *const vk::MemoryUnmapInfoKHR) -> Result<(), Error> {
        match (self.unmap_memory2_khr)(device, p_memory_unmap_info) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Create one or more new shaders"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCreateShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html)"]
    pub unsafe fn create_shaders_ext(
        &self,
        device: vk::Device,
        create_info_count: u32,
        p_create_infos: *const vk::ShaderCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_shaders: *mut vk::ShaderEXT,
    ) -> Result<(), Error> {
        match (self.create_shaders_ext)(device, create_info_count, p_create_infos, p_allocator, p_shaders) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "Description: Destroy a shader object"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkDestroyShaderEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html)"]
    pub unsafe fn destroy_shader_ext(&self, device: vk::Device, shader: vk::ShaderEXT, p_allocator: *const vk::AllocationCallbacks) {
        (self.destroy_shader_ext)(device, shader, p_allocator);
    }

    #[inline]
    #[doc = "Description: Bind shader objects to a command buffer"]
    #[doc = "<br>"]
    #[doc = "Reference: [`vkCmdBindShadersEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html)"]
    pub unsafe fn cmd_bind_shaders_ext(&self, command_buffer: vk::CommandBuffer, stage_count: u32, p_stages: *const vk::ShaderStageFlagBits, p_shaders: *const vk::ShaderEXT) {
        (self.cmd_bind_shaders_ext)(command_buffer, stage_count, p_stages, p_shaders);
    }
}
