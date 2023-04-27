use super::*;

pub struct InstanceCreateInfo<'a> {
    pub application_name: &'a str,
    pub engine_name: &'a str,
    pub validation_layers: bool,
}

impl Default for InstanceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            application_name: "vulk",
            engine_name: "vulk",
            validation_layers: true,
        }
    }
}

pub struct Instance {
    _init: vulk::Init,
    instance: vulk::Instance,
    debug_utils: DebugUtils,
}

impl Instance {
    pub unsafe fn create(create_info: &InstanceCreateInfo<'_>) -> Result<Self> {
        // Vulk.
        let init = vulk::Init::load().context("Initializing Vulk")?;

        // Instance-specific debug messenger.
        let debug_utils_messenger_create_info_ext =
            debug_utils::debug_utils_messenger_create_info_ext();

        // Validation features.
        let enabled_validation_features = [
            vk::ValidationFeatureEnableEXT::BestPracticesEXT,
            vk::ValidationFeatureEnableEXT::SynchronizationValidationEXT,
        ];
        let validation_features_ext = vk::ValidationFeaturesEXT {
            s_type: vk::StructureType::ValidationFeaturesEXT,
            p_next: addr_of!(debug_utils_messenger_create_info_ext).cast(),
            enabled_validation_feature_count: enabled_validation_features.len() as _,
            p_enabled_validation_features: enabled_validation_features.as_ptr(),
            disabled_validation_feature_count: 0,
            p_disabled_validation_features: null(),
        };

        // Layers.
        let mut enabled_layer_names = vec![];
        if create_info.validation_layers {
            enabled_layer_names.push(b"VK_LAYER_KHRONOS_validation\0".as_ptr().cast());
        }

        // Instance.
        let application_name = std::ffi::CString::new(create_info.application_name)?;
        let engine_name = std::ffi::CString::new(create_info.engine_name)?;
        let instance = init.create_instance(&vk::InstanceCreateInfo {
            s_type: vk::StructureType::InstanceCreateInfo,
            p_next: addr_of!(validation_features_ext).cast(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &vk::ApplicationInfo {
                s_type: vk::StructureType::ApplicationInfo,
                p_next: null(),
                p_application_name: application_name.as_ptr(),
                application_version: 1,
                p_engine_name: engine_name.as_ptr(),
                engine_version: 1,
                api_version: vulk::REQUIRED_VULKAN_VERSION,
            },
            enabled_layer_count: enabled_layer_names.len() as _,
            pp_enabled_layer_names: enabled_layer_names.as_ptr(),
            enabled_extension_count: vulk::REQUIRED_INSTANCE_EXTENSIONS.len() as _,
            pp_enabled_extension_names: vulk::REQUIRED_INSTANCE_EXTENSIONS.as_ptr(),
        })?;
        let instance = vulk::Instance::load(&init, instance)?;

        // Debug utils.
        let debug_utils = debug_utils::DebugUtils::create(&instance)?;

        Ok(Self {
            _init: init,
            instance,
            debug_utils,
        })
    }

    pub unsafe fn destroy(self) {
        self.debug_utils.destroy(&self.instance);
        self.instance.destroy_instance();
    }
}

impl std::ops::Deref for Instance {
    type Target = vulk::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
