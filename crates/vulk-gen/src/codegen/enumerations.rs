use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
pub enum {{rs_ident}} {
    {{rs_members}}
}

impl std::fmt::Display for {{rs_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
"#;

const TEMPLATE_MEMBER: &str = r#"{{vk_member_attr}}{{rs_member_ident}} = {{rs_member_value}},"#;

const TEMPLATE_FORMAT_ASPECT_MASK: &str = r#"
impl Format {
    #[must_use]
    pub fn aspect_mask(self) -> ImageAspectFlags {
        #[allow(clippy::match_same_arms)]
        match self {
            {{aspect_mask_matches}}
        }
    }

    #[must_use]
    pub const fn block_size(self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            {{block_size_matches}}
        }
    }
}
"#;

const TEMPLATE_FORMAT_MATCH: &str = r#"Format::{{match_lhs}} => {{match_rhs}},"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    let format_map = {
        let mut map = HashMap::new();
        for format in &ctx.registry.formats {
            map.insert(format.name.as_str(), format);
        }
        map
    };

    for registry_enum in &ctx.registry.enums {
        let registry::EnumType::Enum = registry_enum.ty else {
            continue;
        };

        let vk_ident = &registry_enum.name;
        let vk_attr = attributes::Builder::new()
            .repr("i32")
            .derive("Clone, Copy, PartialEq, Eq, Debug")
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_simple_type(vk_ident)?;

        let vk_member_idents = registry_enum
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>();
        let rs_member_idents = translation::vk_enum(vk_ident, &vk_member_idents)?;
        let mut rs_members = String::new();
        for (member, rs_member_ident) in registry_enum.members.iter().zip(&rs_member_idents) {
            if member.alias.is_some() {
                continue;
            }
            let vk_member_ident = &member.name;
            let vk_member_attr = attributes::Builder::new()
                .doc_translated(vk_member_ident)
                .build();
            let vk_member_value = member.value.as_ref().with_context(|| {
                format!("Missing value, enum={vk_ident}, member={vk_member_ident}")
            })?;
            let rs_member_value = vk_member_value;
            writeln!(
                rs_members,
                "{}",
                TEMPLATE_MEMBER
                    .replace("{{vk_member_attr}}", &vk_member_attr)
                    .replace("{{rs_member_ident}}", rs_member_ident)
                    .replace("{{rs_member_value}}", rs_member_value)
            )?;
        }

        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_attr}}", &vk_attr)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;

        // Special: VkFormat implementations.
        if vk_ident == "VkFormat" {
            let mut aspect_mask_matches = String::new();
            let mut block_size_matches = String::new();
            for (vk_member_ident, rs_member_ident) in vk_member_idents.iter().zip(&rs_member_idents)
            {
                let aspect_flags = if let Some(&format) = format_map.get(vk_member_ident) {
                    let color_aspect = format.components.iter().any(|c| {
                        matches!(
                            c,
                            registry::Component::R
                                | registry::Component::G
                                | registry::Component::B
                                | registry::Component::A
                        )
                    });
                    let depth_aspect = format
                        .components
                        .iter()
                        .any(|c| matches!(c, registry::Component::D));
                    let stencil_aspect = format
                        .components
                        .iter()
                        .any(|c| matches!(c, registry::Component::S));
                    let plane_0_aspect = format
                        .planes
                        .iter()
                        .any(|c| matches!(c, registry::Plane::P0));
                    let plane_1_aspect = format
                        .planes
                        .iter()
                        .any(|c| matches!(c, registry::Plane::P1));
                    let plane_2_aspect = format
                        .planes
                        .iter()
                        .any(|c| matches!(c, registry::Plane::P2));

                    let mut flags = String::new();
                    macro_rules! push {
                        ($flags:expr, $cond:expr, $aspect:literal) => {
                            if $cond {
                                if !$flags.is_empty() {
                                    flags.push('|');
                                }
                                flags.push_str("ImageAspectFlagBits::");
                                flags.push_str($aspect);
                            }
                        };
                    }
                    push!(flags, color_aspect, "Color");
                    push!(flags, depth_aspect, "Depth");
                    push!(flags, stencil_aspect, "Stencil");
                    push!(flags, plane_0_aspect, "Plane0");
                    push!(flags, plane_1_aspect, "Plane1");
                    push!(flags, plane_2_aspect, "Plane2");

                    // Special: make sure the return types match.
                    if !flags.contains('|') {
                        flags.push_str(".into()");
                    }

                    format!("{flags}")
                } else {
                    "ImageAspectFlags::empty()".to_string()
                };

                let block_size = if let Some(&format) = format_map.get(vk_member_ident) {
                    format.block_size.to_string()
                } else {
                    "0".to_string()
                };

                writeln!(
                    aspect_mask_matches,
                    "{}",
                    TEMPLATE_FORMAT_MATCH
                        .replace("{{match_lhs}}", rs_member_ident)
                        .replace("{{match_rhs}}", &aspect_flags)
                )?;
                writeln!(
                    block_size_matches,
                    "{}",
                    TEMPLATE_FORMAT_MATCH
                        .replace("{{match_lhs}}", rs_member_ident)
                        .replace("{{match_rhs}}", &block_size)
                )?;
            }

            writeln!(
                str,
                "{}",
                TEMPLATE_FORMAT_ASPECT_MASK
                    .replace("{{aspect_mask_matches}}", &aspect_mask_matches)
                    .replace("{{block_size_matches}}", &block_size_matches)
            )?;
        }
    }

    Ok(str)
}
