pub const TEMPLATE: &str = r#"
//
// Imports
//

use std::ffi::{c_char, c_void};
use thiserror::Error;
use super::vk;

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
    {{loader::struct_members}}
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
        let loader = libloading::Library::new(VULKAN_LIB_PATH).map_err(|_| Error::LibraryLoad(std::borrow::Cow::Borrowed(VULKAN_LIB_PATH))).map(std::sync::Arc::new)?;

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
            {{loader::loaders}}
            _loader: loader,
        })
    }

    {{loader::wrappers}}
}

//
// Instance
//

pub struct InstanceFunctions {
    {{instance::struct_members}}
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
            {{instance::loaders}}
        })
    }

    {{instance::wrappers}}
}

//
// Device
//

pub struct DeviceFunctions {
    {{device::struct_members}}
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
            {{device::loaders}}
        })
    }

    {{device::wrappers}}
}
"#;
