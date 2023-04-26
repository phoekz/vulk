use super::*;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct Surface {
    surface: vk::SurfaceKHR,
    pub create_info: vk::Win32SurfaceCreateInfoKHR,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub surface_formats: Vec<vk::SurfaceFormatKHR>,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_modes: Vec<vk::PresentModeKHR>,
    pub present_mode: vk::PresentModeKHR,
}

impl Surface {
    #[cfg(target_family = "windows")]
    pub unsafe fn create<Window>(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        window: &Window,
    ) -> Result<Self>
    where
        Window: HasRawDisplayHandle + HasRawWindowHandle,
    {
        use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

        // Surface.
        let display_handle = window.raw_display_handle();
        let window_handle = window.raw_window_handle();
        let create_info = match (display_handle, window_handle) {
            (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
                vk::Win32SurfaceCreateInfoKHR {
                    s_type: vk::StructureType::Win32SurfaceCreateInfoKHR,
                    p_next: null(),
                    flags: vk::Win32SurfaceCreateFlagsKHR::empty(),
                    hinstance: window.hinstance,
                    hwnd: window.hwnd,
                }
            }
            _ => {
                bail!("Unsupported platform: display_handle={display_handle:?}, window_handle={window_handle:?}");
            }
        };
        let surface = instance.create_win32_surface_khr(&create_info)?;

        // Surface capabilities.
        let surface_capabilities = instance
            .get_physical_device_surface_capabilities_khr(physical_device.handle(), surface)?;

        // Surface formats.
        let surface_formats = vulk::read_to_vec(
            |count, ptr| {
                instance.get_physical_device_surface_formats_khr(
                    physical_device.handle(),
                    surface,
                    count,
                    ptr,
                )
            },
            None,
        )?;
        let surface_format = *surface_formats
            .iter()
            .find(|f| f.format == vk::Format::B8g8r8a8Unorm)
            .context("Finding surface format")?;

        // Present modes.
        let present_modes = vulk::read_to_vec(
            |count, ptr| {
                instance.get_physical_device_surface_present_modes_khr(
                    physical_device.handle(),
                    surface,
                    count,
                    ptr,
                )
            },
            None,
        )?;
        let present_mode = *present_modes
            .iter()
            .find(|&&p| p == vk::PresentModeKHR::FifoKHR)
            .context("Finding present mode")?;

        Ok(Self {
            surface,
            create_info,
            surface_capabilities,
            surface_formats,
            surface_format,
            present_modes,
            present_mode,
        })
    }

    #[cfg(not(target_family = "windows"))]
    pub unsafe fn create<Window>(
        _instance: &Instance,
        _physical_device: &PhysicalDevice,
        _window: &Window,
    ) -> Result<Self>
    where
        Window: HasRawDisplayHandle + HasRawWindowHandle,
    {
        todo!("Unsupported platform");
    }

    pub unsafe fn destroy(self, instance: &Instance) {
        instance.destroy_surface_khr(self.surface);
    }

    #[must_use]
    pub fn handle(&self) -> vk::SurfaceKHR {
        self.surface
    }
}
