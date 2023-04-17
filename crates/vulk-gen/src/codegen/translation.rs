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

pub fn vk_enum(vk_name: &str, vk_members: &[String]) -> Result<Vec<String>> {
    // Prepare name.
    let name = format!("{}", heck::AsShoutySnakeCase(vk_name));
    let name_parts = name.split('_').map(ToString::to_string).collect::<Vec<_>>();
    ensure!(
        name_parts.iter().all(|part| !part
            .chars()
            .next()
            .expect("Part must be non-empty")
            .is_ascii_digit()),
        "Enum name parts can not start with a digit. {vk_name:?} {vk_members:?}"
    );

    // Prepare members.
    let members_parts = vk_members
        .iter()
        .map(|vk_member| vk_member.split('_').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Count the minimum number of shared parts in members.
    let shared_parts = members_parts
        .iter()
        .map(|member_parts| {
            name_parts
                .iter()
                .zip(member_parts)
                .take_while(|(a, b)| a == b)
                .count()
        })
        .min()
        .unwrap();

    // Strip shared parts from members.
    let mut members_parts = members_parts
        .into_iter()
        .map(|member_parts| {
            let (_, rhs_parts) = member_parts.split_at(shared_parts);
            rhs_parts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Special: if resulting enum member starts with a digit, append the last part from the enum name.
    let any_member_starts_with_digit = members_parts.iter().any(|member_parts| {
        member_parts
            .first()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_ascii_digit()
    });
    if any_member_starts_with_digit {
        for member_parts in &mut members_parts {
            member_parts.insert(0, name_parts.last().unwrap().clone());
        }
    }

    // Finalize.
    let members = members_parts
        .into_iter()
        .map(|member_parts| {
            member_parts
                .into_iter()
                .map(|part| match part.as_str() {
                    "KHR" | "EXT" => part, // Special: retain capitalization on certain suffixes.
                    part => format!("{}", heck::AsPascalCase(&part)),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    Ok(members)
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
    fn test_vk_enum() {
        let name = "VkImageViewType";
        let members = [
            ("VK_IMAGE_VIEW_TYPE_1D", "Type1d"),
            ("VK_IMAGE_VIEW_TYPE_2D", "Type2d"),
            ("VK_IMAGE_VIEW_TYPE_3D", "Type3d"),
            ("VK_IMAGE_VIEW_TYPE_CUBE", "TypeCube"),
            ("VK_IMAGE_VIEW_TYPE_1D_ARRAY", "Type1dArray"),
            ("VK_IMAGE_VIEW_TYPE_2D_ARRAY", "Type2dArray"),
            ("VK_IMAGE_VIEW_TYPE_CUBE_ARRAY", "TypeCubeArray"),
        ];
        let (member_idents, member_expect): (Vec<String>, Vec<String>) = members
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .unzip();
        let output_idents = vk_enum(name, &member_idents).unwrap();
        for (input, output) in member_expect.into_iter().zip(output_idents) {
            assert_eq!(input, output);
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
