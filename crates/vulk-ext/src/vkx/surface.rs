use super::*;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub struct Surface {
    surface: vk::SurfaceKHR,
    pub create_info: vk::Win32SurfaceCreateInfoKHR,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub surface_formats: Vec<vk::SurfaceFormatKHR>,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_modes: Vec<vk::PresentModeKHR>,
    pub present_mode: vk::PresentModeKHR,
}

// Todo: check this when picking a queue
// let present_supported = instance.get_physical_device_surface_support_khr(
//     physical_device.handle,
//     queue_family.index,
//     surface,
// )?;
// ensure!(present_supported == vk::TRUE);

impl Surface {
    pub unsafe fn create<Window>(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        window: &Window,
    ) -> Result<Self>
    where
        Window: HasRawDisplayHandle + HasRawWindowHandle,
    {
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
        let surface_capabilities =
            instance.get_physical_device_surface_capabilities_khr(**physical_device, surface)?;

        // Surface formats.
        let surface_formats = vulk::read_to_vec(
            |count, ptr| {
                instance.get_physical_device_surface_formats_khr(
                    **physical_device,
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
                    **physical_device,
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

    // Todo: change to consuming self.
    pub unsafe fn destroy(&self, instance: &Instance) {
        instance.destroy_surface_khr(self.surface);
    }

    #[must_use]
    pub fn swapchain_create_info_khr(&self) -> vk::SwapchainCreateInfoKHR {
        vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKHR,
            p_next: null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: self.surface,
            min_image_count: self.surface_capabilities.min_image_count,
            image_format: self.surface_format.format,
            image_color_space: self.surface_format.color_space,
            image_extent: vk::Extent2D {
                width: self.surface_capabilities.min_image_extent.width,
                height: self.surface_capabilities.min_image_extent.height,
            },
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlagBits::ColorAttachment.into(),
            image_sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
            pre_transform: vk::SurfaceTransformFlagBitsKHR::IdentityKHR,
            composite_alpha: vk::CompositeAlphaFlagBitsKHR::OpaqueKHR,
            present_mode: self.present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
        }
    }

    #[must_use]
    pub fn image_view_create_info(&self, image: vk::Image) -> vk::ImageViewCreateInfo {
        vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type: vk::ImageViewType::Type2d,
            format: self.surface_format.format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::Identity,
                g: vk::ComponentSwizzle::Identity,
                b: vk::ComponentSwizzle::Identity,
                a: vk::ComponentSwizzle::Identity,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: self.surface_format.format.aspect_mask(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        }
    }
}
