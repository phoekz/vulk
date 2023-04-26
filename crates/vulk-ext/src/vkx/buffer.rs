use super::*;

/// **Example**:
///
/// ```no_run
/// # use vulk::vk as vk;
/// # use vulk_ext::vkx as vkx;
/// # unsafe {
/// # let device = todo!();
/// # let size = todo!();
/// # let usage = todo!();
/// let (buffer, buffer_create_info) =
///     vkx::BufferCreator::new(size, usage)
///         .create(device)
///         .unwrap();
/// # }
/// ```
pub struct BufferCreator(vk::BufferCreateInfo);

impl BufferCreator {
    #[must_use]
    pub fn new(size: vk::DeviceSize, usage: vk::BufferUsageFlags) -> Self {
        Self(vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage: usage | vk::BufferUsageFlagBits::ShaderDeviceAddress,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
        })
    }

    pub unsafe fn create(self, device: &Device) -> Result<(vk::Buffer, vk::BufferCreateInfo)> {
        let buffer_create_info = self.0;
        let buffer = device.create_buffer(&buffer_create_info)?;
        Ok((buffer, buffer_create_info))
    }
}
