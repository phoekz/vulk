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
// Init
//

pub struct InitFunctions {
    {{init::struct_members}}
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
        let library = libloading::Library::new(VULKAN_LIB_PATH).map_err(|_| Error::LibraryLoad(std::borrow::Cow::Borrowed(VULKAN_LIB_PATH))).map(std::sync::Arc::new)?;

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
                {{init::loaders}}
            },
            _library: library,
        })
    }

    #[must_use]
    pub fn fns(&self) -> &InitFunctions {
        &self.fns
    }

    {{init::wrappers}}
}

//
// Instance
//

pub struct InstanceFunctions {
    {{instance::struct_members}}
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
                {{instance::loaders}}
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

    {{instance::wrappers}}
}

//
// Device
//

pub struct DeviceFunctions {
    {{device::struct_members}}
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
                {{device::loaders}}
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

    {{device::wrappers}}
}
"#;
