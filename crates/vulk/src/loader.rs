//
// Imports
//

use super::{vk, Error};
use std::ffi::{c_char, c_void, CStr};

//
// Init
//

pub struct InitFunctions {
    pub get_instance_proc_addr: Option<vk::GetInstanceProcAddr>,
    pub create_instance: Option<vk::CreateInstance>,
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
        let load = |name: &'static CStr| {
            let pfn = if let Ok(symbol) = library.get::<*const c_void>(name.to_bytes()) {
                *symbol
            } else {
                return None;
            };
            if pfn as usize == 0 {
                return None;
            }
            Some(pfn)
        };

        Ok(Self {
            fns: InitFunctions {
                get_instance_proc_addr: load(c"vkGetInstanceProcAddr").map(|f| std::mem::transmute(f)),
                create_instance: load(c"vkCreateInstance").map(|f| std::mem::transmute(f)),
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
        (self.fns.get_instance_proc_addr.unwrap_unchecked())(instance, p_name)
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
        match (self.fns.create_instance.unwrap_unchecked())(p_create_info, std::ptr::null(), p_instance.as_mut_ptr()) {
            vk::Result::Success => Ok(p_instance.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }
}

//
// Instance
//

pub struct InstanceFunctions {
    pub get_device_proc_addr: Option<vk::GetDeviceProcAddr>,
    pub destroy_instance: Option<vk::DestroyInstance>,
    pub enumerate_physical_devices: Option<vk::EnumeratePhysicalDevices>,
    pub get_physical_device_properties2: Option<vk::GetPhysicalDeviceProperties2>,
    pub get_physical_device_queue_family_properties2: Option<vk::GetPhysicalDeviceQueueFamilyProperties2>,
    pub create_device: Option<vk::CreateDevice>,
    pub get_physical_device_memory_properties2: Option<vk::GetPhysicalDeviceMemoryProperties2>,
    pub create_win32_surface_khr: Option<vk::CreateWin32SurfaceKHR>,
    pub destroy_surface_khr: Option<vk::DestroySurfaceKHR>,
    pub get_physical_device_surface_support_khr: Option<vk::GetPhysicalDeviceSurfaceSupportKHR>,
    pub get_physical_device_surface_capabilities_khr: Option<vk::GetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub get_physical_device_surface_formats_khr: Option<vk::GetPhysicalDeviceSurfaceFormatsKHR>,
    pub get_physical_device_surface_present_modes_khr: Option<vk::GetPhysicalDeviceSurfacePresentModesKHR>,
    pub get_physical_device_calibrateable_time_domains_ext: Option<vk::GetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    pub create_debug_utils_messenger_ext: Option<vk::CreateDebugUtilsMessengerEXT>,
    pub destroy_debug_utils_messenger_ext: Option<vk::DestroyDebugUtilsMessengerEXT>,
}

pub struct Instance {
    fns: InstanceFunctions,
    handle: vk::Instance,
}

impl Instance {
    pub unsafe fn load(init: &Init, instance: vk::Instance) -> Result<Self, Error> {
        let load = |name: &'static CStr| {
            let pfn = init.get_instance_proc_addr(instance, name.as_ptr().cast());
            if pfn as usize == 0 {
                return None;
            }
            Some(pfn)
        };

        Ok(Self {
            fns: InstanceFunctions {
                get_device_proc_addr: load(c"vkGetDeviceProcAddr").map(|f| std::mem::transmute(f)),
                destroy_instance: load(c"vkDestroyInstance").map(|f| std::mem::transmute(f)),
                enumerate_physical_devices: load(c"vkEnumeratePhysicalDevices").map(|f| std::mem::transmute(f)),
                get_physical_device_properties2: load(c"vkGetPhysicalDeviceProperties2").map(|f| std::mem::transmute(f)),
                get_physical_device_queue_family_properties2: load(c"vkGetPhysicalDeviceQueueFamilyProperties2").map(|f| std::mem::transmute(f)),
                create_device: load(c"vkCreateDevice").map(|f| std::mem::transmute(f)),
                get_physical_device_memory_properties2: load(c"vkGetPhysicalDeviceMemoryProperties2").map(|f| std::mem::transmute(f)),
                create_win32_surface_khr: load(c"vkCreateWin32SurfaceKHR").map(|f| std::mem::transmute(f)),
                destroy_surface_khr: load(c"vkDestroySurfaceKHR").map(|f| std::mem::transmute(f)),
                get_physical_device_surface_support_khr: load(c"vkGetPhysicalDeviceSurfaceSupportKHR").map(|f| std::mem::transmute(f)),
                get_physical_device_surface_capabilities_khr: load(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR").map(|f| std::mem::transmute(f)),
                get_physical_device_surface_formats_khr: load(c"vkGetPhysicalDeviceSurfaceFormatsKHR").map(|f| std::mem::transmute(f)),
                get_physical_device_surface_present_modes_khr: load(c"vkGetPhysicalDeviceSurfacePresentModesKHR").map(|f| std::mem::transmute(f)),
                get_physical_device_calibrateable_time_domains_ext: load(c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT").map(|f| std::mem::transmute(f)),
                create_debug_utils_messenger_ext: load(c"vkCreateDebugUtilsMessengerEXT").map(|f| std::mem::transmute(f)),
                destroy_debug_utils_messenger_ext: load(c"vkDestroyDebugUtilsMessengerEXT").map(|f| std::mem::transmute(f)),
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
        (self.fns.get_device_proc_addr.unwrap_unchecked())(device, p_name)
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
        (self.fns.destroy_instance.unwrap_unchecked())(self.handle, std::ptr::null());
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
        match (self.fns.enumerate_physical_devices.unwrap_unchecked())(self.handle, p_physical_device_count, p_physical_devices) {
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
        (self.fns.get_physical_device_properties2.unwrap_unchecked())(physical_device, p_properties);
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
        (self.fns.get_physical_device_queue_family_properties2.unwrap_unchecked())(physical_device, p_queue_family_property_count, p_queue_family_properties);
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
        match (self.fns.create_device.unwrap_unchecked())(physical_device, p_create_info, std::ptr::null(), p_device.as_mut_ptr()) {
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
        (self.fns.get_physical_device_memory_properties2.unwrap_unchecked())(physical_device, p_memory_properties);
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a VkSurfaceKHR object for an Win32 native window"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)"]
    pub unsafe fn create_win32_surface_khr(&self, p_create_info: *const vk::Win32SurfaceCreateInfoKHR) -> Result<vk::SurfaceKHR, Error> {
        let mut p_surface = std::mem::MaybeUninit::uninit();
        match (self.fns.create_win32_surface_khr.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_surface.as_mut_ptr()) {
            vk::Result::Success => Ok(p_surface.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a VkSurfaceKHR object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroySurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)"]
    pub unsafe fn destroy_surface_khr(&self, surface: vk::SurfaceKHR) {
        (self.fns.destroy_surface_khr.unwrap_unchecked())(self.handle, surface, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query if presentation is supported"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)"]
    pub unsafe fn get_physical_device_surface_support_khr(&self, physical_device: vk::PhysicalDevice, queue_family_index: u32, surface: vk::SurfaceKHR) -> Result<vk::Bool32, Error> {
        let mut p_supported = std::mem::MaybeUninit::uninit();
        match (self.fns.get_physical_device_surface_support_khr.unwrap_unchecked())(physical_device, queue_family_index, surface, p_supported.as_mut_ptr()) {
            vk::Result::Success => Ok(p_supported.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query surface capabilities"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)"]
    pub unsafe fn get_physical_device_surface_capabilities_khr(&self, physical_device: vk::PhysicalDevice, surface: vk::SurfaceKHR) -> Result<vk::SurfaceCapabilitiesKHR, Error> {
        let mut p_surface_capabilities = std::mem::MaybeUninit::uninit();
        match (self.fns.get_physical_device_surface_capabilities_khr.unwrap_unchecked())(physical_device, surface, p_surface_capabilities.as_mut_ptr()) {
            vk::Result::Success => Ok(p_surface_capabilities.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query color formats supported by surface"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)"]
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut vk::SurfaceFormatKHR,
    ) -> Result<(), Error> {
        match (self.fns.get_physical_device_surface_formats_khr.unwrap_unchecked())(physical_device, surface, p_surface_format_count, p_surface_formats) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query supported presentation modes"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)"]
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut vk::PresentModeKHR,
    ) -> Result<(), Error> {
        match (self.fns.get_physical_device_surface_present_modes_khr.unwrap_unchecked())(physical_device, surface, p_present_mode_count, p_present_modes) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
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
        match (self.fns.get_physical_device_calibrateable_time_domains_ext.unwrap_unchecked())(physical_device, p_time_domain_count, p_time_domains) {
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
        match (self.fns.create_debug_utils_messenger_ext.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_messenger.as_mut_ptr()) {
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
        (self.fns.destroy_debug_utils_messenger_ext.unwrap_unchecked())(self.handle, messenger, std::ptr::null());
    }
}

//
// Device
//

pub struct DeviceFunctions {
    pub destroy_device: Option<vk::DestroyDevice>,
    pub get_device_queue2: Option<vk::GetDeviceQueue2>,
    pub create_command_pool: Option<vk::CreateCommandPool>,
    pub reset_command_pool: Option<vk::ResetCommandPool>,
    pub destroy_command_pool: Option<vk::DestroyCommandPool>,
    pub allocate_command_buffers: Option<vk::AllocateCommandBuffers>,
    pub reset_command_buffer: Option<vk::ResetCommandBuffer>,
    pub free_command_buffers: Option<vk::FreeCommandBuffers>,
    pub begin_command_buffer: Option<vk::BeginCommandBuffer>,
    pub end_command_buffer: Option<vk::EndCommandBuffer>,
    pub queue_submit2: Option<vk::QueueSubmit2>,
    pub create_semaphore: Option<vk::CreateSemaphore>,
    pub destroy_semaphore: Option<vk::DestroySemaphore>,
    pub get_semaphore_counter_value: Option<vk::GetSemaphoreCounterValue>,
    pub wait_semaphores: Option<vk::WaitSemaphores>,
    pub signal_semaphore: Option<vk::SignalSemaphore>,
    pub cmd_pipeline_barrier2: Option<vk::CmdPipelineBarrier2>,
    pub queue_wait_idle: Option<vk::QueueWaitIdle>,
    pub device_wait_idle: Option<vk::DeviceWaitIdle>,
    pub get_calibrated_timestamps_ext: Option<vk::GetCalibratedTimestampsEXT>,
    pub cmd_begin_rendering: Option<vk::CmdBeginRendering>,
    pub cmd_end_rendering: Option<vk::CmdEndRendering>,
    pub create_shaders_ext: Option<vk::CreateShadersEXT>,
    pub cmd_bind_shaders_ext: Option<vk::CmdBindShadersEXT>,
    pub destroy_shader_ext: Option<vk::DestroyShaderEXT>,
    pub create_shader_module: Option<vk::CreateShaderModule>,
    pub destroy_shader_module: Option<vk::DestroyShaderModule>,
    pub create_ray_tracing_pipelines_khr: Option<vk::CreateRayTracingPipelinesKHR>,
    pub get_ray_tracing_shader_group_handles_khr: Option<vk::GetRayTracingShaderGroupHandlesKHR>,
    pub destroy_pipeline: Option<vk::DestroyPipeline>,
    pub cmd_bind_pipeline: Option<vk::CmdBindPipeline>,
    pub allocate_memory: Option<vk::AllocateMemory>,
    pub free_memory: Option<vk::FreeMemory>,
    pub map_memory2_khr: Option<vk::MapMemory2KHR>,
    pub unmap_memory2_khr: Option<vk::UnmapMemory2KHR>,
    pub create_buffer: Option<vk::CreateBuffer>,
    pub destroy_buffer: Option<vk::DestroyBuffer>,
    pub create_image: Option<vk::CreateImage>,
    pub destroy_image: Option<vk::DestroyImage>,
    pub create_image_view: Option<vk::CreateImageView>,
    pub destroy_image_view: Option<vk::DestroyImageView>,
    pub create_acceleration_structure_khr: Option<vk::CreateAccelerationStructureKHR>,
    pub get_acceleration_structure_build_sizes_khr: Option<vk::GetAccelerationStructureBuildSizesKHR>,
    pub destroy_acceleration_structure_khr: Option<vk::DestroyAccelerationStructureKHR>,
    pub get_acceleration_structure_device_address_khr: Option<vk::GetAccelerationStructureDeviceAddressKHR>,
    pub get_device_buffer_memory_requirements: Option<vk::GetDeviceBufferMemoryRequirements>,
    pub get_device_image_memory_requirements: Option<vk::GetDeviceImageMemoryRequirements>,
    pub bind_buffer_memory2: Option<vk::BindBufferMemory2>,
    pub bind_image_memory2: Option<vk::BindImageMemory2>,
    pub create_sampler: Option<vk::CreateSampler>,
    pub destroy_sampler: Option<vk::DestroySampler>,
    pub create_descriptor_set_layout: Option<vk::CreateDescriptorSetLayout>,
    pub destroy_descriptor_set_layout: Option<vk::DestroyDescriptorSetLayout>,
    pub create_pipeline_layout: Option<vk::CreatePipelineLayout>,
    pub destroy_pipeline_layout: Option<vk::DestroyPipelineLayout>,
    pub cmd_push_constants: Option<vk::CmdPushConstants>,
    pub get_buffer_device_address: Option<vk::GetBufferDeviceAddress>,
    pub get_descriptor_set_layout_size_ext: Option<vk::GetDescriptorSetLayoutSizeEXT>,
    pub get_descriptor_set_layout_binding_offset_ext: Option<vk::GetDescriptorSetLayoutBindingOffsetEXT>,
    pub get_descriptor_ext: Option<vk::GetDescriptorEXT>,
    pub cmd_bind_descriptor_buffers_ext: Option<vk::CmdBindDescriptorBuffersEXT>,
    pub cmd_set_descriptor_buffer_offsets_ext: Option<vk::CmdSetDescriptorBufferOffsetsEXT>,
    pub create_query_pool: Option<vk::CreateQueryPool>,
    pub destroy_query_pool: Option<vk::DestroyQueryPool>,
    pub reset_query_pool: Option<vk::ResetQueryPool>,
    pub cmd_begin_query: Option<vk::CmdBeginQuery>,
    pub cmd_end_query: Option<vk::CmdEndQuery>,
    pub get_query_pool_results: Option<vk::GetQueryPoolResults>,
    pub cmd_write_timestamp2: Option<vk::CmdWriteTimestamp2>,
    pub cmd_copy_buffer2: Option<vk::CmdCopyBuffer2>,
    pub cmd_copy_image2: Option<vk::CmdCopyImage2>,
    pub cmd_copy_buffer_to_image2: Option<vk::CmdCopyBufferToImage2>,
    pub cmd_copy_image_to_buffer2: Option<vk::CmdCopyImageToBuffer2>,
    pub cmd_draw_mesh_tasks_ext: Option<vk::CmdDrawMeshTasksEXT>,
    pub cmd_draw_mesh_tasks_indirect_ext: Option<vk::CmdDrawMeshTasksIndirectEXT>,
    pub cmd_draw_mesh_tasks_indirect_count_ext: Option<vk::CmdDrawMeshTasksIndirectCountEXT>,
    pub cmd_set_viewport_with_count: Option<vk::CmdSetViewportWithCount>,
    pub cmd_set_scissor_with_count: Option<vk::CmdSetScissorWithCount>,
    pub cmd_set_rasterization_samples_ext: Option<vk::CmdSetRasterizationSamplesEXT>,
    pub cmd_set_front_face: Option<vk::CmdSetFrontFace>,
    pub cmd_set_cull_mode: Option<vk::CmdSetCullMode>,
    pub cmd_set_depth_test_enable: Option<vk::CmdSetDepthTestEnable>,
    pub cmd_set_depth_compare_op: Option<vk::CmdSetDepthCompareOp>,
    pub cmd_set_depth_write_enable: Option<vk::CmdSetDepthWriteEnable>,
    pub cmd_set_color_blend_enable_ext: Option<vk::CmdSetColorBlendEnableEXT>,
    pub cmd_set_color_blend_equation_ext: Option<vk::CmdSetColorBlendEquationEXT>,
    pub cmd_set_color_write_mask_ext: Option<vk::CmdSetColorWriteMaskEXT>,
    pub cmd_dispatch: Option<vk::CmdDispatch>,
    pub cmd_dispatch_indirect: Option<vk::CmdDispatchIndirect>,
    pub create_swapchain_khr: Option<vk::CreateSwapchainKHR>,
    pub destroy_swapchain_khr: Option<vk::DestroySwapchainKHR>,
    pub get_swapchain_images_khr: Option<vk::GetSwapchainImagesKHR>,
    pub acquire_next_image2_khr: Option<vk::AcquireNextImage2KHR>,
    pub queue_present_khr: Option<vk::QueuePresentKHR>,
    pub cmd_build_acceleration_structures_khr: Option<vk::CmdBuildAccelerationStructuresKHR>,
    pub cmd_trace_rays_khr: Option<vk::CmdTraceRaysKHR>,
    pub cmd_trace_rays_indirect2_khr: Option<vk::CmdTraceRaysIndirect2KHR>,
}

pub struct Device {
    fns: DeviceFunctions,
    handle: vk::Device,
}

impl Device {
    pub unsafe fn load(instance: &Instance, device: vk::Device) -> Result<Self, Error> {
        let load = |name: &'static CStr| {
            let pfn = instance.get_device_proc_addr(device, name.as_ptr().cast());
            if pfn as usize == 0 {
                return None;
            }
            Some(pfn)
        };

        Ok(Self {
            fns: DeviceFunctions {
                destroy_device: load(c"vkDestroyDevice").map(|f| std::mem::transmute(f)),
                get_device_queue2: load(c"vkGetDeviceQueue2").map(|f| std::mem::transmute(f)),
                create_command_pool: load(c"vkCreateCommandPool").map(|f| std::mem::transmute(f)),
                reset_command_pool: load(c"vkResetCommandPool").map(|f| std::mem::transmute(f)),
                destroy_command_pool: load(c"vkDestroyCommandPool").map(|f| std::mem::transmute(f)),
                allocate_command_buffers: load(c"vkAllocateCommandBuffers").map(|f| std::mem::transmute(f)),
                reset_command_buffer: load(c"vkResetCommandBuffer").map(|f| std::mem::transmute(f)),
                free_command_buffers: load(c"vkFreeCommandBuffers").map(|f| std::mem::transmute(f)),
                begin_command_buffer: load(c"vkBeginCommandBuffer").map(|f| std::mem::transmute(f)),
                end_command_buffer: load(c"vkEndCommandBuffer").map(|f| std::mem::transmute(f)),
                queue_submit2: load(c"vkQueueSubmit2").map(|f| std::mem::transmute(f)),
                create_semaphore: load(c"vkCreateSemaphore").map(|f| std::mem::transmute(f)),
                destroy_semaphore: load(c"vkDestroySemaphore").map(|f| std::mem::transmute(f)),
                get_semaphore_counter_value: load(c"vkGetSemaphoreCounterValue").map(|f| std::mem::transmute(f)),
                wait_semaphores: load(c"vkWaitSemaphores").map(|f| std::mem::transmute(f)),
                signal_semaphore: load(c"vkSignalSemaphore").map(|f| std::mem::transmute(f)),
                cmd_pipeline_barrier2: load(c"vkCmdPipelineBarrier2").map(|f| std::mem::transmute(f)),
                queue_wait_idle: load(c"vkQueueWaitIdle").map(|f| std::mem::transmute(f)),
                device_wait_idle: load(c"vkDeviceWaitIdle").map(|f| std::mem::transmute(f)),
                get_calibrated_timestamps_ext: load(c"vkGetCalibratedTimestampsEXT").map(|f| std::mem::transmute(f)),
                cmd_begin_rendering: load(c"vkCmdBeginRendering").map(|f| std::mem::transmute(f)),
                cmd_end_rendering: load(c"vkCmdEndRendering").map(|f| std::mem::transmute(f)),
                create_shaders_ext: load(c"vkCreateShadersEXT").map(|f| std::mem::transmute(f)),
                cmd_bind_shaders_ext: load(c"vkCmdBindShadersEXT").map(|f| std::mem::transmute(f)),
                destroy_shader_ext: load(c"vkDestroyShaderEXT").map(|f| std::mem::transmute(f)),
                create_shader_module: load(c"vkCreateShaderModule").map(|f| std::mem::transmute(f)),
                destroy_shader_module: load(c"vkDestroyShaderModule").map(|f| std::mem::transmute(f)),
                create_ray_tracing_pipelines_khr: load(c"vkCreateRayTracingPipelinesKHR").map(|f| std::mem::transmute(f)),
                get_ray_tracing_shader_group_handles_khr: load(c"vkGetRayTracingShaderGroupHandlesKHR").map(|f| std::mem::transmute(f)),
                destroy_pipeline: load(c"vkDestroyPipeline").map(|f| std::mem::transmute(f)),
                cmd_bind_pipeline: load(c"vkCmdBindPipeline").map(|f| std::mem::transmute(f)),
                allocate_memory: load(c"vkAllocateMemory").map(|f| std::mem::transmute(f)),
                free_memory: load(c"vkFreeMemory").map(|f| std::mem::transmute(f)),
                map_memory2_khr: load(c"vkMapMemory2KHR").map(|f| std::mem::transmute(f)),
                unmap_memory2_khr: load(c"vkUnmapMemory2KHR").map(|f| std::mem::transmute(f)),
                create_buffer: load(c"vkCreateBuffer").map(|f| std::mem::transmute(f)),
                destroy_buffer: load(c"vkDestroyBuffer").map(|f| std::mem::transmute(f)),
                create_image: load(c"vkCreateImage").map(|f| std::mem::transmute(f)),
                destroy_image: load(c"vkDestroyImage").map(|f| std::mem::transmute(f)),
                create_image_view: load(c"vkCreateImageView").map(|f| std::mem::transmute(f)),
                destroy_image_view: load(c"vkDestroyImageView").map(|f| std::mem::transmute(f)),
                create_acceleration_structure_khr: load(c"vkCreateAccelerationStructureKHR").map(|f| std::mem::transmute(f)),
                get_acceleration_structure_build_sizes_khr: load(c"vkGetAccelerationStructureBuildSizesKHR").map(|f| std::mem::transmute(f)),
                destroy_acceleration_structure_khr: load(c"vkDestroyAccelerationStructureKHR").map(|f| std::mem::transmute(f)),
                get_acceleration_structure_device_address_khr: load(c"vkGetAccelerationStructureDeviceAddressKHR").map(|f| std::mem::transmute(f)),
                get_device_buffer_memory_requirements: load(c"vkGetDeviceBufferMemoryRequirements").map(|f| std::mem::transmute(f)),
                get_device_image_memory_requirements: load(c"vkGetDeviceImageMemoryRequirements").map(|f| std::mem::transmute(f)),
                bind_buffer_memory2: load(c"vkBindBufferMemory2").map(|f| std::mem::transmute(f)),
                bind_image_memory2: load(c"vkBindImageMemory2").map(|f| std::mem::transmute(f)),
                create_sampler: load(c"vkCreateSampler").map(|f| std::mem::transmute(f)),
                destroy_sampler: load(c"vkDestroySampler").map(|f| std::mem::transmute(f)),
                create_descriptor_set_layout: load(c"vkCreateDescriptorSetLayout").map(|f| std::mem::transmute(f)),
                destroy_descriptor_set_layout: load(c"vkDestroyDescriptorSetLayout").map(|f| std::mem::transmute(f)),
                create_pipeline_layout: load(c"vkCreatePipelineLayout").map(|f| std::mem::transmute(f)),
                destroy_pipeline_layout: load(c"vkDestroyPipelineLayout").map(|f| std::mem::transmute(f)),
                cmd_push_constants: load(c"vkCmdPushConstants").map(|f| std::mem::transmute(f)),
                get_buffer_device_address: load(c"vkGetBufferDeviceAddress").map(|f| std::mem::transmute(f)),
                get_descriptor_set_layout_size_ext: load(c"vkGetDescriptorSetLayoutSizeEXT").map(|f| std::mem::transmute(f)),
                get_descriptor_set_layout_binding_offset_ext: load(c"vkGetDescriptorSetLayoutBindingOffsetEXT").map(|f| std::mem::transmute(f)),
                get_descriptor_ext: load(c"vkGetDescriptorEXT").map(|f| std::mem::transmute(f)),
                cmd_bind_descriptor_buffers_ext: load(c"vkCmdBindDescriptorBuffersEXT").map(|f| std::mem::transmute(f)),
                cmd_set_descriptor_buffer_offsets_ext: load(c"vkCmdSetDescriptorBufferOffsetsEXT").map(|f| std::mem::transmute(f)),
                create_query_pool: load(c"vkCreateQueryPool").map(|f| std::mem::transmute(f)),
                destroy_query_pool: load(c"vkDestroyQueryPool").map(|f| std::mem::transmute(f)),
                reset_query_pool: load(c"vkResetQueryPool").map(|f| std::mem::transmute(f)),
                cmd_begin_query: load(c"vkCmdBeginQuery").map(|f| std::mem::transmute(f)),
                cmd_end_query: load(c"vkCmdEndQuery").map(|f| std::mem::transmute(f)),
                get_query_pool_results: load(c"vkGetQueryPoolResults").map(|f| std::mem::transmute(f)),
                cmd_write_timestamp2: load(c"vkCmdWriteTimestamp2").map(|f| std::mem::transmute(f)),
                cmd_copy_buffer2: load(c"vkCmdCopyBuffer2").map(|f| std::mem::transmute(f)),
                cmd_copy_image2: load(c"vkCmdCopyImage2").map(|f| std::mem::transmute(f)),
                cmd_copy_buffer_to_image2: load(c"vkCmdCopyBufferToImage2").map(|f| std::mem::transmute(f)),
                cmd_copy_image_to_buffer2: load(c"vkCmdCopyImageToBuffer2").map(|f| std::mem::transmute(f)),
                cmd_draw_mesh_tasks_ext: load(c"vkCmdDrawMeshTasksEXT").map(|f| std::mem::transmute(f)),
                cmd_draw_mesh_tasks_indirect_ext: load(c"vkCmdDrawMeshTasksIndirectEXT").map(|f| std::mem::transmute(f)),
                cmd_draw_mesh_tasks_indirect_count_ext: load(c"vkCmdDrawMeshTasksIndirectCountEXT").map(|f| std::mem::transmute(f)),
                cmd_set_viewport_with_count: load(c"vkCmdSetViewportWithCount").map(|f| std::mem::transmute(f)),
                cmd_set_scissor_with_count: load(c"vkCmdSetScissorWithCount").map(|f| std::mem::transmute(f)),
                cmd_set_rasterization_samples_ext: load(c"vkCmdSetRasterizationSamplesEXT").map(|f| std::mem::transmute(f)),
                cmd_set_front_face: load(c"vkCmdSetFrontFace").map(|f| std::mem::transmute(f)),
                cmd_set_cull_mode: load(c"vkCmdSetCullMode").map(|f| std::mem::transmute(f)),
                cmd_set_depth_test_enable: load(c"vkCmdSetDepthTestEnable").map(|f| std::mem::transmute(f)),
                cmd_set_depth_compare_op: load(c"vkCmdSetDepthCompareOp").map(|f| std::mem::transmute(f)),
                cmd_set_depth_write_enable: load(c"vkCmdSetDepthWriteEnable").map(|f| std::mem::transmute(f)),
                cmd_set_color_blend_enable_ext: load(c"vkCmdSetColorBlendEnableEXT").map(|f| std::mem::transmute(f)),
                cmd_set_color_blend_equation_ext: load(c"vkCmdSetColorBlendEquationEXT").map(|f| std::mem::transmute(f)),
                cmd_set_color_write_mask_ext: load(c"vkCmdSetColorWriteMaskEXT").map(|f| std::mem::transmute(f)),
                cmd_dispatch: load(c"vkCmdDispatch").map(|f| std::mem::transmute(f)),
                cmd_dispatch_indirect: load(c"vkCmdDispatchIndirect").map(|f| std::mem::transmute(f)),
                create_swapchain_khr: load(c"vkCreateSwapchainKHR").map(|f| std::mem::transmute(f)),
                destroy_swapchain_khr: load(c"vkDestroySwapchainKHR").map(|f| std::mem::transmute(f)),
                get_swapchain_images_khr: load(c"vkGetSwapchainImagesKHR").map(|f| std::mem::transmute(f)),
                acquire_next_image2_khr: load(c"vkAcquireNextImage2KHR").map(|f| std::mem::transmute(f)),
                queue_present_khr: load(c"vkQueuePresentKHR").map(|f| std::mem::transmute(f)),
                cmd_build_acceleration_structures_khr: load(c"vkCmdBuildAccelerationStructuresKHR").map(|f| std::mem::transmute(f)),
                cmd_trace_rays_khr: load(c"vkCmdTraceRaysKHR").map(|f| std::mem::transmute(f)),
                cmd_trace_rays_indirect2_khr: load(c"vkCmdTraceRaysIndirect2KHR").map(|f| std::mem::transmute(f)),
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
        (self.fns.destroy_device.unwrap_unchecked())(self.handle, std::ptr::null());
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
        (self.fns.get_device_queue2.unwrap_unchecked())(self.handle, p_queue_info, p_queue.as_mut_ptr());
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
        match (self.fns.create_command_pool.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_command_pool.as_mut_ptr()) {
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
        match (self.fns.reset_command_pool.unwrap_unchecked())(self.handle, command_pool, flags) {
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
        (self.fns.destroy_command_pool.unwrap_unchecked())(self.handle, command_pool, std::ptr::null());
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
        match (self.fns.allocate_command_buffers.unwrap_unchecked())(self.handle, p_allocate_info, p_command_buffers) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Command Buffers"]
    #[doc = "<br>"]
    #[doc = "**Description**: Reset a command buffer to the initial state"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)"]
    pub unsafe fn reset_command_buffer(&self, command_buffer: vk::CommandBuffer, flags: vk::CommandBufferResetFlags) -> Result<(), Error> {
        match (self.fns.reset_command_buffer.unwrap_unchecked())(command_buffer, flags) {
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
        (self.fns.free_command_buffers.unwrap_unchecked())(self.handle, command_pool, command_buffer_count, p_command_buffers);
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
        match (self.fns.begin_command_buffer.unwrap_unchecked())(command_buffer, p_begin_info) {
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
        match (self.fns.end_command_buffer.unwrap_unchecked())(command_buffer) {
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
        match (self.fns.queue_submit2.unwrap_unchecked())(queue, submit_count, p_submits, fence) {
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
        match (self.fns.create_semaphore.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_semaphore.as_mut_ptr()) {
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
        (self.fns.destroy_semaphore.unwrap_unchecked())(self.handle, semaphore, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query the current state of a timeline semaphore"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetSemaphoreCounterValue`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html)"]
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: vk::Semaphore) -> Result<u64, Error> {
        let mut p_value = std::mem::MaybeUninit::uninit();
        match (self.fns.get_semaphore_counter_value.unwrap_unchecked())(self.handle, semaphore, p_value.as_mut_ptr()) {
            vk::Result::Success => Ok(p_value.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
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
        match (self.fns.wait_semaphores.unwrap_unchecked())(self.handle, p_wait_info, timeout) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Synchronization and Cache Control"]
    #[doc = "<br>"]
    #[doc = "**Description**: Signal a timeline semaphore on the host"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkSignalSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html)"]
    pub unsafe fn signal_semaphore(&self, p_signal_info: *const vk::SemaphoreSignalInfo) -> Result<(), Error> {
        match (self.fns.signal_semaphore.unwrap_unchecked())(self.handle, p_signal_info) {
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
        (self.fns.cmd_pipeline_barrier2.unwrap_unchecked())(command_buffer, p_dependency_info);
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
        match (self.fns.queue_wait_idle.unwrap_unchecked())(queue) {
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
        match (self.fns.device_wait_idle.unwrap_unchecked())(self.handle) {
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
        match (self.fns.get_calibrated_timestamps_ext.unwrap_unchecked())(self.handle, timestamp_count, p_timestamp_infos, p_timestamps, p_max_deviation) {
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
        (self.fns.cmd_begin_rendering.unwrap_unchecked())(command_buffer, p_rendering_info);
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
        (self.fns.cmd_end_rendering.unwrap_unchecked())(command_buffer);
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
        match (self.fns.create_shaders_ext.unwrap_unchecked())(self.handle, create_info_count, p_create_infos, std::ptr::null(), p_shaders) {
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
        (self.fns.cmd_bind_shaders_ext.unwrap_unchecked())(command_buffer, stage_count, p_stages, p_shaders);
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
        (self.fns.destroy_shader_ext.unwrap_unchecked())(self.handle, shader, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Creates a new shader module object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)"]
    pub unsafe fn create_shader_module(&self, p_create_info: *const vk::ShaderModuleCreateInfo) -> Result<vk::ShaderModule, Error> {
        let mut p_shader_module = std::mem::MaybeUninit::uninit();
        match (self.fns.create_shader_module.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_shader_module.as_mut_ptr()) {
            vk::Result::Success => Ok(p_shader_module.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Shaders"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a shader module"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)"]
    pub unsafe fn destroy_shader_module(&self, shader_module: vk::ShaderModule) {
        (self.fns.destroy_shader_module.unwrap_unchecked())(self.handle, shader_module, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Pipelines"]
    #[doc = "<br>"]
    #[doc = "**Description**: Creates a new ray tracing pipeline object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_ray_tracing_pipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateRayTracingPipelinesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)"]
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        pipeline_cache: vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const vk::RayTracingPipelineCreateInfoKHR,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<(), Error> {
        match (self.fns.create_ray_tracing_pipelines_khr.unwrap_unchecked())(self.handle, deferred_operation, pipeline_cache, create_info_count, p_create_infos, std::ptr::null(), p_pipelines) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Pipelines"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query ray tracing pipeline shader group handles"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_ray_tracing_pipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetRayTracingShaderGroupHandlesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)"]
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(&self, pipeline: vk::Pipeline, first_group: u32, group_count: u32, data_size: usize, p_data: *mut c_void) -> Result<(), Error> {
        match (self.fns.get_ray_tracing_shader_group_handles_khr.unwrap_unchecked())(self.handle, pipeline, first_group, group_count, data_size, p_data) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Pipelines"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a pipeline object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)"]
    pub unsafe fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        (self.fns.destroy_pipeline.unwrap_unchecked())(self.handle, pipeline, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Pipelines"]
    #[doc = "<br>"]
    #[doc = "**Description**: Bind a pipeline object to a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_0`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)"]
    pub unsafe fn cmd_bind_pipeline(&self, command_buffer: vk::CommandBuffer, pipeline_bind_point: vk::PipelineBindPoint, pipeline: vk::Pipeline) {
        (self.fns.cmd_bind_pipeline.unwrap_unchecked())(command_buffer, pipeline_bind_point, pipeline);
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
        match (self.fns.allocate_memory.unwrap_unchecked())(self.handle, p_allocate_info, std::ptr::null(), p_memory.as_mut_ptr()) {
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
        (self.fns.free_memory.unwrap_unchecked())(self.handle, memory, std::ptr::null());
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
        match (self.fns.map_memory2_khr.unwrap_unchecked())(self.handle, p_memory_map_info, pp_data.as_mut_ptr()) {
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
        match (self.fns.unmap_memory2_khr.unwrap_unchecked())(self.handle, p_memory_unmap_info) {
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
        match (self.fns.create_buffer.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_buffer.as_mut_ptr()) {
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
        (self.fns.destroy_buffer.unwrap_unchecked())(self.handle, buffer, std::ptr::null());
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
        match (self.fns.create_image.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_image.as_mut_ptr()) {
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
        (self.fns.destroy_image.unwrap_unchecked())(self.handle, image, std::ptr::null());
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
        match (self.fns.create_image_view.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_view.as_mut_ptr()) {
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
        (self.fns.destroy_image_view.unwrap_unchecked())(self.handle, image_view, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a new acceleration structure object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_acceleration_structure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateAccelerationStructureKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)"]
    pub unsafe fn create_acceleration_structure_khr(&self, p_create_info: *const vk::AccelerationStructureCreateInfoKHR) -> Result<vk::AccelerationStructureKHR, Error> {
        let mut p_acceleration_structure = std::mem::MaybeUninit::uninit();
        match (self.fns.create_acceleration_structure_khr.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_acceleration_structure.as_mut_ptr()) {
            vk::Result::Success => Ok(p_acceleration_structure.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Retrieve the required size for an acceleration structure"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_acceleration_structure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetAccelerationStructureBuildSizesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)"]
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        build_type: vk::AccelerationStructureBuildTypeKHR,
        p_build_info: *const vk::AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut vk::AccelerationStructureBuildSizesInfoKHR,
    ) {
        (self.fns.get_acceleration_structure_build_sizes_khr.unwrap_unchecked())(self.handle, build_type, p_build_info, p_max_primitive_counts, p_size_info);
    }

    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy an acceleration structure object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_acceleration_structure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroyAccelerationStructureKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)"]
    pub unsafe fn destroy_acceleration_structure_khr(&self, acceleration_structure: vk::AccelerationStructureKHR) {
        (self.fns.destroy_acceleration_structure_khr.unwrap_unchecked())(self.handle, acceleration_structure, std::ptr::null());
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Resource Creation"]
    #[doc = "<br>"]
    #[doc = "**Description**: Query an address of a acceleration structure"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_acceleration_structure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetAccelerationStructureDeviceAddressKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)"]
    pub unsafe fn get_acceleration_structure_device_address_khr(&self, p_info: *const vk::AccelerationStructureDeviceAddressInfoKHR) -> vk::DeviceAddress {
        (self.fns.get_acceleration_structure_device_address_khr.unwrap_unchecked())(self.handle, p_info)
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
        (self.fns.get_device_buffer_memory_requirements.unwrap_unchecked())(self.handle, p_info, p_memory_requirements);
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
        (self.fns.get_device_image_memory_requirements.unwrap_unchecked())(self.handle, p_info, p_memory_requirements);
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
        match (self.fns.bind_buffer_memory2.unwrap_unchecked())(self.handle, bind_info_count, p_bind_infos) {
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
        match (self.fns.bind_image_memory2.unwrap_unchecked())(self.handle, bind_info_count, p_bind_infos) {
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
        match (self.fns.create_sampler.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_sampler.as_mut_ptr()) {
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
        (self.fns.destroy_sampler.unwrap_unchecked())(self.handle, sampler, std::ptr::null());
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
        match (self.fns.create_descriptor_set_layout.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_set_layout.as_mut_ptr()) {
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
        (self.fns.destroy_descriptor_set_layout.unwrap_unchecked())(self.handle, descriptor_set_layout, std::ptr::null());
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
        match (self.fns.create_pipeline_layout.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_pipeline_layout.as_mut_ptr()) {
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
        (self.fns.destroy_pipeline_layout.unwrap_unchecked())(self.handle, pipeline_layout, std::ptr::null());
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
        (self.fns.cmd_push_constants.unwrap_unchecked())(command_buffer, layout, stage_flags, offset, size, p_values);
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
        (self.fns.get_buffer_device_address.unwrap_unchecked())(self.handle, p_info)
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
        (self.fns.get_descriptor_set_layout_size_ext.unwrap_unchecked())(self.handle, layout, p_layout_size_in_bytes.as_mut_ptr());
        p_layout_size_in_bytes.assume_init()
    }

    #[must_use]
    #[inline]
    #[doc = "**Chapter**: Resource Descriptors"]
    #[doc = "<br>"]
    #[doc = "**Description**: Get the offset of a binding within a descriptor set layout"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_descriptor_buffer`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html)"]
    pub unsafe fn get_descriptor_set_layout_binding_offset_ext(&self, layout: vk::DescriptorSetLayout, binding: u32) -> vk::DeviceSize {
        let mut p_offset = std::mem::MaybeUninit::uninit();
        (self.fns.get_descriptor_set_layout_binding_offset_ext.unwrap_unchecked())(self.handle, layout, binding, p_offset.as_mut_ptr());
        p_offset.assume_init()
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
        (self.fns.get_descriptor_ext.unwrap_unchecked())(self.handle, p_descriptor_info, data_size, p_descriptor);
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
        (self.fns.cmd_bind_descriptor_buffers_ext.unwrap_unchecked())(command_buffer, buffer_count, p_binding_infos);
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
        (self.fns.cmd_set_descriptor_buffer_offsets_ext.unwrap_unchecked())(command_buffer, pipeline_bind_point, layout, first_set, set_count, p_buffer_indices, p_offsets);
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
        match (self.fns.create_query_pool.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_query_pool.as_mut_ptr()) {
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
        (self.fns.destroy_query_pool.unwrap_unchecked())(self.handle, query_pool, std::ptr::null());
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
        (self.fns.reset_query_pool.unwrap_unchecked())(self.handle, query_pool, first_query, query_count);
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
        (self.fns.cmd_begin_query.unwrap_unchecked())(command_buffer, query_pool, query, flags);
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
        (self.fns.cmd_end_query.unwrap_unchecked())(command_buffer, query_pool, query);
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
        match (self.fns.get_query_pool_results.unwrap_unchecked())(self.handle, query_pool, first_query, query_count, data_size, p_data, stride, flags) {
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
        (self.fns.cmd_write_timestamp2.unwrap_unchecked())(command_buffer, stage, query_pool, query);
    }

    #[inline]
    #[doc = "**Chapter**: Copy Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Copy data between buffer regions"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdCopyBuffer2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html)"]
    pub unsafe fn cmd_copy_buffer2(&self, command_buffer: vk::CommandBuffer, p_copy_buffer_info: *const vk::CopyBufferInfo2) {
        (self.fns.cmd_copy_buffer2.unwrap_unchecked())(command_buffer, p_copy_buffer_info);
    }

    #[inline]
    #[doc = "**Chapter**: Copy Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Copy data between images"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_VERSION_1_3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdCopyImage2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html)"]
    pub unsafe fn cmd_copy_image2(&self, command_buffer: vk::CommandBuffer, p_copy_image_info: *const vk::CopyImageInfo2) {
        (self.fns.cmd_copy_image2.unwrap_unchecked())(command_buffer, p_copy_image_info);
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
        (self.fns.cmd_copy_buffer_to_image2.unwrap_unchecked())(command_buffer, p_copy_buffer_to_image_info);
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
        (self.fns.cmd_copy_image_to_buffer2.unwrap_unchecked())(command_buffer, p_copy_image_to_buffer_info);
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
        (self.fns.cmd_draw_mesh_tasks_ext.unwrap_unchecked())(command_buffer, group_count_x, group_count_y, group_count_z);
    }

    #[inline]
    #[doc = "**Chapter**: Drawing Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Issue an indirect mesh tasks draw into a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdDrawMeshTasksIndirectEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html)"]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(&self, command_buffer: vk::CommandBuffer, buffer: vk::Buffer, offset: vk::DeviceSize, draw_count: u32, stride: u32) {
        (self.fns.cmd_draw_mesh_tasks_indirect_ext.unwrap_unchecked())(command_buffer, buffer, offset, draw_count, stride);
    }

    #[inline]
    #[doc = "**Chapter**: Drawing Commands"]
    #[doc = "<br>"]
    #[doc = "**Description**: Perform an indirect mesh tasks draw with the draw count sourced from a buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_mesh_shader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdDrawMeshTasksIndirectCountEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html)"]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fns.cmd_draw_mesh_tasks_indirect_count_ext.unwrap_unchecked())(command_buffer, buffer, offset, count_buffer, count_buffer_offset, max_draw_count, stride);
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
        (self.fns.cmd_set_viewport_with_count.unwrap_unchecked())(command_buffer, viewport_count, p_viewports);
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
        (self.fns.cmd_set_scissor_with_count.unwrap_unchecked())(command_buffer, scissor_count, p_scissors);
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
        (self.fns.cmd_set_rasterization_samples_ext.unwrap_unchecked())(command_buffer, rasterization_samples);
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
        (self.fns.cmd_set_front_face.unwrap_unchecked())(command_buffer, front_face);
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
        (self.fns.cmd_set_cull_mode.unwrap_unchecked())(command_buffer, cull_mode);
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
        (self.fns.cmd_set_depth_test_enable.unwrap_unchecked())(command_buffer, depth_test_enable);
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
        (self.fns.cmd_set_depth_compare_op.unwrap_unchecked())(command_buffer, depth_compare_op);
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
        (self.fns.cmd_set_depth_write_enable.unwrap_unchecked())(command_buffer, depth_write_enable);
    }

    #[inline]
    #[doc = "**Chapter**: The Framebuffer"]
    #[doc = "<br>"]
    #[doc = "**Description**: Specify the pname:blendEnable for each attachment dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetColorBlendEnableEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)"]
    pub unsafe fn cmd_set_color_blend_enable_ext(&self, command_buffer: vk::CommandBuffer, first_attachment: u32, attachment_count: u32, p_color_blend_enables: *const vk::Bool32) {
        (self.fns.cmd_set_color_blend_enable_ext.unwrap_unchecked())(command_buffer, first_attachment, attachment_count, p_color_blend_enables);
    }

    #[inline]
    #[doc = "**Chapter**: The Framebuffer"]
    #[doc = "<br>"]
    #[doc = "**Description**: Specify the blend factors and operations dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetColorBlendEquationEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)"]
    pub unsafe fn cmd_set_color_blend_equation_ext(&self, command_buffer: vk::CommandBuffer, first_attachment: u32, attachment_count: u32, p_color_blend_equations: *const vk::ColorBlendEquationEXT) {
        (self.fns.cmd_set_color_blend_equation_ext.unwrap_unchecked())(command_buffer, first_attachment, attachment_count, p_color_blend_equations);
    }

    #[inline]
    #[doc = "**Chapter**: The Framebuffer"]
    #[doc = "<br>"]
    #[doc = "**Description**: Specify the color write masks for each attachment dynamically for a command buffer"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_EXT_extended_dynamic_state3`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdSetColorWriteMaskEXT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)"]
    pub unsafe fn cmd_set_color_write_mask_ext(&self, command_buffer: vk::CommandBuffer, first_attachment: u32, attachment_count: u32, p_color_write_masks: *const vk::ColorComponentFlags) {
        (self.fns.cmd_set_color_write_mask_ext.unwrap_unchecked())(command_buffer, first_attachment, attachment_count, p_color_write_masks);
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
        (self.fns.cmd_dispatch.unwrap_unchecked())(command_buffer, group_count_x, group_count_y, group_count_z);
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
        (self.fns.cmd_dispatch_indirect.unwrap_unchecked())(command_buffer, buffer, offset);
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Create a swapchain"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)"]
    pub unsafe fn create_swapchain_khr(&self, p_create_info: *const vk::SwapchainCreateInfoKHR) -> Result<vk::SwapchainKHR, Error> {
        let mut p_swapchain = std::mem::MaybeUninit::uninit();
        match (self.fns.create_swapchain_khr.unwrap_unchecked())(self.handle, p_create_info, std::ptr::null(), p_swapchain.as_mut_ptr()) {
            vk::Result::Success => Ok(p_swapchain.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Destroy a swapchain object"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkDestroySwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)"]
    pub unsafe fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        (self.fns.destroy_swapchain_khr.unwrap_unchecked())(self.handle, swapchain, std::ptr::null());
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Obtain the array of presentable images associated with a swapchain"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)"]
    pub unsafe fn get_swapchain_images_khr(&self, swapchain: vk::SwapchainKHR, p_swapchain_image_count: *mut u32, p_swapchain_images: *mut vk::Image) -> Result<(), Error> {
        match (self.fns.get_swapchain_images_khr.unwrap_unchecked())(self.handle, swapchain, p_swapchain_image_count, p_swapchain_images) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Retrieve the index of the next available presentable image"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkAcquireNextImage2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)"]
    pub unsafe fn acquire_next_image2_khr(&self, p_acquire_info: *const vk::AcquireNextImageInfoKHR) -> Result<u32, Error> {
        let mut p_image_index = std::mem::MaybeUninit::uninit();
        match (self.fns.acquire_next_image2_khr.unwrap_unchecked())(self.handle, p_acquire_info, p_image_index.as_mut_ptr()) {
            vk::Result::Success => Ok(p_image_index.assume_init()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Window System Integration (WSI)"]
    #[doc = "<br>"]
    #[doc = "**Description**: Queue an image for presentation"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)"]
    pub unsafe fn queue_present_khr(&self, queue: vk::Queue, p_present_info: *const vk::PresentInfoKHR) -> Result<(), Error> {
        match (self.fns.queue_present_khr.unwrap_unchecked())(queue, p_present_info) {
            vk::Result::Success => Ok(()),
            result => Err(Error::Vulkan(result)),
        }
    }

    #[inline]
    #[doc = "**Chapter**: Acceleration Structures"]
    #[doc = "<br>"]
    #[doc = "**Description**: Build an acceleration structure"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_acceleration_structure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdBuildAccelerationStructuresKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)"]
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        info_count: u32,
        p_infos: *const vk::AccelerationStructureBuildGeometryInfoKHR,
        pp_build_range_infos: *const *const vk::AccelerationStructureBuildRangeInfoKHR,
    ) {
        (self.fns.cmd_build_acceleration_structures_khr.unwrap_unchecked())(command_buffer, info_count, p_infos, pp_build_range_infos);
    }

    #[inline]
    #[doc = "**Chapter**: Ray Tracing"]
    #[doc = "<br>"]
    #[doc = "**Description**: Initialize a ray tracing dispatch"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_ray_tracing_pipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdTraceRaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)"]
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_raygen_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self.fns.cmd_trace_rays_khr.unwrap_unchecked())(
            command_buffer,
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            width,
            height,
            depth,
        );
    }

    #[inline]
    #[doc = "**Chapter**: Ray Tracing"]
    #[doc = "<br>"]
    #[doc = "**Description**: Initialize an indirect ray tracing dispatch with indirect shader binding tables"]
    #[doc = "<br>"]
    #[doc = "**Provided by**: [`VK_KHR_ray_tracing_maintenance1`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_maintenance1.html)"]
    #[doc = "<br>"]
    #[doc = "**Reference**: [`vkCmdTraceRaysIndirect2KHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html)"]
    pub unsafe fn cmd_trace_rays_indirect2_khr(&self, command_buffer: vk::CommandBuffer, indirect_device_address: vk::DeviceAddress) {
        (self.fns.cmd_trace_rays_indirect2_khr.unwrap_unchecked())(command_buffer, indirect_device_address);
    }
}
