use super::*;

#[derive(Debug)]
pub(crate) struct DebugUtils {
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugUtils {
    pub(crate) unsafe fn create(instance: &vulk::Instance) -> Result<Self> {
        let debug_utils_messenger_create_info_ext = debug_utils_messenger_create_info_ext();
        let debug_utils_messenger =
            instance.create_debug_utils_messenger_ext(&debug_utils_messenger_create_info_ext)?;
        Ok(Self {
            debug_utils_messenger,
        })
    }

    pub(crate) unsafe fn destroy(&self, instance: &vulk::Instance) {
        instance.destroy_debug_utils_messenger_ext(self.debug_utils_messenger);
    }
}

pub(crate) fn debug_utils_messenger_create_info_ext() -> vk::DebugUtilsMessengerCreateInfoEXT {
    let message_severity = vk::DebugUtilsMessageSeverityFlagBitsEXT::VerboseEXT
        | vk::DebugUtilsMessageSeverityFlagBitsEXT::InfoEXT
        | vk::DebugUtilsMessageSeverityFlagBitsEXT::WarningEXT
        | vk::DebugUtilsMessageSeverityFlagBitsEXT::ErrorEXT;
    let message_type = vk::DebugUtilsMessageTypeFlagBitsEXT::GeneralEXT
        | vk::DebugUtilsMessageTypeFlagBitsEXT::ValidationEXT
        | vk::DebugUtilsMessageTypeFlagBitsEXT::PerformanceEXT;
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DebugUtilsMessengerCreateInfoEXT,
        p_next: null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity,
        message_type,
        pfn_user_callback: debug_utils_messenger_callback as _,
        p_user_data: null_mut(),
    }
}

unsafe extern "C" fn debug_utils_messenger_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    // Unpack.
    let callback_data = *p_callback_data;
    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };
    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };
    let message_id_number: u32 = i32::cast_unsigned(callback_data.message_id_number);

    // Filter.
    if message_id_name == "Loader Message"
        && !message.starts_with("Loading layer library")
        && (message_severity == vk::DebugUtilsMessageSeverityFlagBitsEXT::InfoEXT
            || message_severity == vk::DebugUtilsMessageSeverityFlagBitsEXT::VerboseEXT)
    {
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkCreateInstance-specialuse-extension-debugging"
    {
        // CreateInstance(): Attempting to enable extension VK_EXT_debug_utils,
        // but this extension is intended to support use by applications when
        // debugging and it is strongly recommended that it be otherwise
        // avoided.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkBindBufferMemory-requirements-not-retrieved" {
        // vkBindBufferMemory2() pBindInfos[0]: Binding memory to VkBuffer
        // 0xf56c9b0000000004[] but vkGetBufferMemoryRequirements() has not been
        // called on that buffer.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkBindImageMemory-requirements-not-retrieved" {
        // vkBindImageMemory2() pBindInfos[0]: Binding memory to VkImage
        // 0xdcc8fd0000000012[] but vkGetImageMemoryRequirements() has not been
        // called on that image.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkAllocateMemory-small-allocation" {
        // vkAllocateMemory(): Allocating a VkDeviceMemory of size 256. This is
        // a very small allocation (current threshold is 262144 bytes). You
        // should make large allocations and sub-allocate from one large
        // VkDeviceMemory.
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-vkBindMemory-small-dedicated-allocation" {
        // vkBindBufferMemory2() pBindInfos[0]: Trying to bind VkBuffer
        // 0xcb3ee80000000007[] to a memory block which is fully consumed by the
        // buffer. The required size of the allocation is 256, but smaller
        // buffers like this should be sub-allocated from larger memory blocks.
        // (Current threshold is 1048576 bytes.)
        return vk::FALSE;
    }
    if message_id_name == "UNASSIGNED-BestPractices-pipeline-stage-flags" &&
        message.contains("You are using VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR when vkCmdWriteTimestamp2 is called") {
        return vk::FALSE;
    }

    // Severity.
    let level = match message_severity {
        vk::DebugUtilsMessageSeverityFlagBitsEXT::VerboseEXT => log::Level::Debug,
        vk::DebugUtilsMessageSeverityFlagBitsEXT::InfoEXT => log::Level::Info,
        vk::DebugUtilsMessageSeverityFlagBitsEXT::WarningEXT => log::Level::Warn,
        vk::DebugUtilsMessageSeverityFlagBitsEXT::ErrorEXT => log::Level::Error,
    };

    // Log.
    log!(
        level,
        "message_type={message_type:?}, message_id_name={message_id_name}, message_id_number=0x{message_id_number:08x}, message={message}"
    );

    vk::FALSE
}
