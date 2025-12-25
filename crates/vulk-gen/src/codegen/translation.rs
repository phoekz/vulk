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
    vk_text: Option<&String>,
    vk_enum: Option<&String>,
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
            let array_sizes = rs_text.split("][").collect::<Vec<_>>();
            if array_sizes.len() == 1 {
                // 1D array.
                let array_size = *array_sizes.first().unwrap();
                if let Some(vk_enum) = vk_enum.as_ref() {
                    assert!(array_size.is_empty());
                    assert!(vk_enum.starts_with("VK_"));
                    let constant_name = vk_enum.trim_start_matches("VK_");
                    format!("[{rs_type}; {constant_name} as _]")
                } else {
                    let element_count: u32 = array_size
                        .parse()
                        .with_context(|| format!("Parsing element_count from {array_size}"))?;
                    format!("[{rs_type}; {element_count}]")
                }
            } else if array_sizes.len() == 2 {
                // 2D array.
                let array_sizes = array_sizes
                    .into_iter()
                    .map(|array_size| {
                        let array_size: u32 = array_size.parse()?;
                        Ok::<_, anyhow::Error>(array_size)
                    })
                    .collect::<Result<Vec<_>>>()?;
                format!("[[{rs_type}; {}]; {}]", array_sizes[1], array_sizes[0])
            } else {
                bail!("Failed to construct array specifier from {vk_text}");
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

pub fn vk_enum(vk_name: &str, vk_members: &[&str]) -> Result<Vec<String>> {
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

pub fn vk_bitmask(vk_name: &str, vk_members: &[&str]) -> Result<Vec<String>> {
    // Special: no members.
    if vk_members.is_empty() {
        return Ok(vec![]);
    }

    // Split into parts.
    let name = format!("{}", heck::AsShoutySnakeCase(vk_name));
    let name = name.replace("BITS2", "BITS_2");
    let mut name_parts = name.split('_').map(ToString::to_string).collect::<Vec<_>>();

    // Remove suffix.
    if name_parts.last().unwrap() == "KHR" || name_parts.last().unwrap() == "EXT" {
        name_parts.pop();
    }

    // Remove `FLAG` and `BITS`.
    let name_parts = name_parts
        .into_iter()
        .filter(|part| !(part == "FLAG" || part == "BITS"))
        .collect::<Vec<_>>();

    // Do the same for members.
    let mut members_parts = vec![];
    let mut members_suffixes = vec![];
    for vk_member in vk_members {
        // Split into parts.
        let mut parts = vk_member
            .split('_')
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        // Remove suffix.
        let mut suffix = None;
        if parts.last().unwrap() == "KHR" || parts.last().unwrap() == "EXT" {
            suffix = parts.pop();
        }

        // Remove `BIT`.
        let parts = parts
            .into_iter()
            .filter(|part| part != "BIT")
            .collect::<Vec<_>>();

        // println!("  {vk_member}\n  {parts:?}");
        members_parts.push(parts);
        members_suffixes.push(suffix);
    }

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
        .zip(members_suffixes)
        .map(|(parts, suffix)| {
            use std::fmt::Write as _;
            let mut parts = parts.into_iter().fold(String::new(), |mut parts, part| {
                write!(parts, "{}", heck::AsPascalCase(&part)).unwrap();
                parts
            });
            if let Some(suffix) = suffix {
                parts.push_str(&suffix);
            }
            parts
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
    fn test_vk_enum_digits() {
        let name = "VkImageViewType";
        let cases = [
            ("VK_IMAGE_VIEW_TYPE_1D", "Type1d"),
            ("VK_IMAGE_VIEW_TYPE_2D", "Type2d"),
            ("VK_IMAGE_VIEW_TYPE_3D", "Type3d"),
            ("VK_IMAGE_VIEW_TYPE_CUBE", "TypeCube"),
            ("VK_IMAGE_VIEW_TYPE_1D_ARRAY", "Type1dArray"),
            ("VK_IMAGE_VIEW_TYPE_2D_ARRAY", "Type2dArray"),
            ("VK_IMAGE_VIEW_TYPE_CUBE_ARRAY", "TypeCubeArray"),
        ];
        let (member_idents, member_expect): (Vec<&str>, Vec<&str>) =
            cases.iter().map(|&(a, b)| (a, b)).unzip();
        let output_idents = vk_enum(name, &member_idents).unwrap();
        for (input, output) in member_expect.into_iter().zip(output_idents) {
            assert_eq!(input, output);
        }
    }

    fn run_vk_bitmask(name: &str, cases: &[(&str, &str)]) {
        let (member_idents, member_expect): (Vec<&str>, Vec<&str>) =
            cases.iter().map(|&(a, b)| (a, b)).unzip();
        let output_idents = vk_bitmask(name, &member_idents).unwrap();
        for (input, output) in member_expect.into_iter().zip(output_idents) {
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_vk_bitmask() {
        #[rustfmt::skip]
        run_vk_bitmask(
            "VkSampleCountFlagBits",
            &[
                ("VK_SAMPLE_COUNT_1_BIT", "Count1"),
                ("VK_SAMPLE_COUNT_2_BIT", "Count2"),
                ("VK_SAMPLE_COUNT_4_BIT", "Count4"),
                ("VK_SAMPLE_COUNT_8_BIT", "Count8"),
                ("VK_SAMPLE_COUNT_16_BIT", "Count16"),
                ("VK_SAMPLE_COUNT_32_BIT", "Count32"),
                ("VK_SAMPLE_COUNT_64_BIT", "Count64"),
            ],
        );

        #[rustfmt::skip]
        run_vk_bitmask(
            "VkDebugUtilsMessageSeverityFlagBitsEXT",
            &[
                ("VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT", "VerboseEXT"),
                ("VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT", "InfoEXT"),
                ("VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT", "WarningEXT"),
                ("VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT", "ErrorEXT"),
            ],
        );

        #[rustfmt::skip]
        run_vk_bitmask(
            "VkAccessFlagBits2",
            &[
                ("VK_ACCESS_2_SHADER_SAMPLED_READ_BIT", "ShaderSampledRead"),
                ("VK_ACCESS_2_SHADER_STORAGE_READ_BIT", "ShaderStorageRead"),
                ("VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT", "ShaderStorageWrite"),
                ("VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT", "DescriptorBufferReadEXT"),
            ],
        );
    }
}
