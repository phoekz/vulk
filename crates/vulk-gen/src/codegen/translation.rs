use super::*;

pub fn c_define(c: &str) -> Result<String> {
    const PREFIX: &str = "VK_";
    ensure!(
        c.starts_with(PREFIX),
        "C defines must begin with {PREFIX}, got {c}"
    );
    let rs = c.trim_start_matches(PREFIX);
    Ok(rs.to_string())
}

pub fn c_type(c_type_map: &CtypeMap, c: &str) -> Result<&'static str> {
    c_type_map
        .get(c)
        .copied()
        .with_context(|| format!("Unknown c_type={c}"))
}

pub fn c_value(c: &str) -> Result<String> {
    let rs = c.trim_start_matches('(');
    let rs = rs.trim_end_matches(')');
    let rs = rs.trim_end_matches('F');
    let rs = rs.trim_end_matches("ULL");
    let rs = rs.trim_end_matches('U');
    let rs = rs.replace('~', "!");
    Ok(rs)
}

pub fn c_specifier(c: &str) -> Result<String> {
    let rs = if c == "*" {
        "*mut".to_string()
    } else if c == "const*" {
        "*const".to_string()
    } else if c == "**" {
        "*mut *mut".to_string()
    } else if c == "const*const*" {
        "*const *const".to_string()
    } else {
        bail!("Unknown c_specifier={c}");
    };
    Ok(rs)
}

pub fn vk_function_pointer(vk: &str) -> Result<String> {
    const PREFIX: &str = "PFN_vk";
    ensure!(
        vk.starts_with(PREFIX),
        "Vk function pointers must begin with {PREFIX}, got {vk}"
    );
    let rs = vk.trim_start_matches(PREFIX);
    let rs = format!("Pfn{rs}");
    Ok(rs)
}

pub fn vk_simple_type(vk: &str) -> Result<String> {
    const PREFIX: &str = "Vk";
    ensure!(
        vk.starts_with(PREFIX),
        "Vk types must begin with {PREFIX}, got {vk}"
    );
    let rs = vk.trim_start_matches(PREFIX);
    let rs = rs.to_string();
    Ok(rs)
}

pub fn vk_complex_type(
    c_type_map: &CtypeMap,
    vk_type: &str,
    vk_text: &Option<String>,
    vk_enum: &Option<String>,
    vk_namespace: bool,
) -> Result<String> {
    // Base name.
    let (mut rs_type, built_in_type) = if let Ok(c_type) = c_type(c_type_map, vk_type) {
        ((*c_type).to_string(), true)
    } else if vk_type.starts_with("PFN_") {
        (
            format!("Pfn{}", vk_type.trim_start_matches("PFN_vk")),
            false,
        )
    } else {
        (vk_simple_type(vk_type)?, false)
    };

    // Append namespace.
    if !built_in_type && vk_namespace {
        rs_type = format!("vk::{rs_type}");
    }

    // Specifiers.
    let rs_type = if let Some(vk_text) = vk_text {
        if vk_text.starts_with('[') && vk_text.ends_with(']') {
            let rs_text = vk_text.trim_start_matches('[').trim_end_matches(']');
            if let Some(vk_enum) = vk_enum.as_ref() {
                assert!(rs_text.is_empty());
                assert!(vk_enum.starts_with("VK_"));
                let constant_name = vk_enum.trim_start_matches("VK_");
                format!("[{rs_type}; {constant_name} as _]")
            } else {
                let element_count: u32 = rs_text.parse()?;
                format!("[{rs_type}; {element_count}]")
            }
        } else {
            let rs_specifier = translation::c_specifier(vk_text)?;
            format!("{rs_specifier} {rs_type}")
        }
    } else {
        rs_type
    };

    Ok(rs_type)
}

pub fn vk_simple_ident(vk: &str) -> Result<String> {
    let mut rs = format!("{}", heck::AsSnakeCase(vk));
    if rs == "type" {
        rs = "ty".to_string();
    }
    let rs = rs
        .replace("1_d", "_1d")
        .replace("2_d", "_2d")
        .replace("3_d", "_3d");
    Ok(rs)
}

pub fn vk_enum_member(vk_name: &str, vk_member: &str) -> Result<String> {
    const ENUM_PREFIX: &str = "Vk";
    const ENUM_MEMBER_PREFIX: &str = "VK_";
    ensure!(
        vk_name.starts_with(ENUM_PREFIX),
        "Vk enums must begin with {ENUM_PREFIX}, got {vk_name}"
    );
    ensure!(
        vk_member.starts_with(ENUM_MEMBER_PREFIX),
        "Vk enum members must begin with {ENUM_MEMBER_PREFIX}, got {vk_member}"
    );

    let rs_name = format!("{}", heck::AsShoutySnakeCase(vk_name));
    let rs_name = rs_name.trim_end_matches("KHR").trim_end_matches("EXT");
    let rs_name = rs_name.trim_start_matches('_').trim_end_matches('_');
    let rs_member = vk_member
        .trim_start_matches(rs_name)
        .trim_start_matches("VK");
    let rs_member = rs_member.trim_start_matches('_').trim_end_matches('_');
    let mut rs_member = format!("{}", heck::AsPascalCase(rs_member));

    if rs_member.ends_with("Khr") {
        rs_member = rs_member.replace("Khr", "");
        rs_member.push_str("KHR");
    }
    if rs_member.ends_with("Ext") {
        rs_member = rs_member.replace("Ext", "");
        rs_member.push_str("EXT");
    }

    Ok(rs_member)
}

pub fn vk_bitmask_member(vk_name: &str, vk_member: &str) -> Result<String> {
    const BITMASK_PREFIX: &str = "Vk";
    const BITMASK_SUFFIX: &str = "Flags";
    const BITMASK_SUFFIX_2: &str = "Flags2";
    const BITMASK_SUFFIX_KHR: &str = "FlagsKHR";
    const BITMASK_SUFFIX_EXT: &str = "FlagsEXT";
    const BITMASK_MEMBER_PREFIX: &str = "VK_";
    ensure!(
        vk_name.starts_with(BITMASK_PREFIX),
        "Vk bitmasks must begin with {BITMASK_PREFIX}, got {vk_name}"
    );
    ensure!(
        vk_name.ends_with(BITMASK_SUFFIX)
            || vk_name.ends_with(BITMASK_SUFFIX_2)
            || vk_name.ends_with(BITMASK_SUFFIX_KHR)
            || vk_name.ends_with(BITMASK_SUFFIX_EXT),
        "Vk bitmasks must end with Flags|Flags2|FlagsKHR|FlagsEXT, got {vk_name}"
    );
    ensure!(
        vk_member.starts_with(BITMASK_MEMBER_PREFIX),
        "Vk bitmask members must begin with {BITMASK_MEMBER_PREFIX}, got {vk_member}"
    );

    let rs_name = format!("{}", heck::AsShoutySnakeCase(vk_name));
    let rs_name = rs_name
        .replace("FLAGS", "")
        .replace("KHR", "")
        .replace("EXT", "");
    let rs_name = rs_name.trim_start_matches('_').trim_end_matches('_');
    let rs_member = vk_member;
    assert!(rs_member.starts_with(rs_name));
    let rs_member = rs_member.replace("_BIT", "");
    let rs_member = rs_member.trim_start_matches(rs_name);
    let rs_member = rs_member.trim_start_matches('_').trim_end_matches('_');

    let rs_member = if rs_member.chars().next().unwrap().is_ascii_digit() {
        format!("NUM_{rs_member}")
    } else {
        rs_member.to_string()
    };

    Ok(rs_member)
}

pub fn vk_simple_function(vk: &str) -> Result<String> {
    const PREFIX: &str = "vk";
    ensure!(
        vk.starts_with(PREFIX),
        "Vk functions must begin with {PREFIX}, got {vk}"
    );
    let rs = vk.trim_start_matches(PREFIX);
    let rs = rs.to_string();
    Ok(rs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vk_simple_ident() {
        #[rustfmt::skip]
        let cases = [
            ("driverVersion", "driver_version"),
            ("sType", "s_type"),
            ("type", "ty"),
            ("Extent3D", "extent_3d"),
        ];
        for (input, expect) in cases {
            assert_eq!(vk_simple_ident(input).unwrap(), expect);
        }
    }

    #[test]
    fn test_vk_enum_member() {
        #[rustfmt::skip]
        let cases = [
            ("VkImageLayout", "VK_IMAGE_LAYOUT_UNDEFINED", "Undefined"),
            ("VkResult", "VK_SUCCESS", "Success"),
            ("VkStructureType", "VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR", "MemoryMapInfoKHR"),
            ("VkQueueGlobalPriorityKHR", "VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR", "LowKHR"),
            ("VkValidationFeatureEnableEXT", "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT", "GpuAssistedEXT"),
        ];
        for (input_name, input_member, expect) in cases {
            assert_eq!(vk_enum_member(input_name, input_member).unwrap(), expect);
        }
    }

    #[test]
    fn test_vk_bitmask_member() {
        #[rustfmt::skip]
        let cases = [
            ("VkBufferUsageFlags", "VK_BUFFER_USAGE_TRANSFER_DST_BIT", "TRANSFER_DST"),
            ("VkVideoDecodeCapabilityFlagsKHR", "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR", "DPB_AND_OUTPUT_COINCIDE_KHR"),
            ("VkDebugUtilsMessageSeverityFlagsEXT", "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT", "VERBOSE_EXT"),
            ("VkPipelineStageFlags2", "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT", "TOP_OF_PIPE"),
            ("VkPipelineStageFlags2", "VK_PIPELINE_STAGE_2_NONE", "NONE"),
            ("VkShaderStageFlags", "VK_SHADER_STAGE_ALL_GRAPHICS", "ALL_GRAPHICS"),
            ("VkSampleCountFlags", "VK_SAMPLE_COUNT_32_BIT", "NUM_32"),
        ];
        for (input_name, input_member, expect) in cases {
            assert_eq!(vk_bitmask_member(input_name, input_member).unwrap(), expect);
        }
    }
}
