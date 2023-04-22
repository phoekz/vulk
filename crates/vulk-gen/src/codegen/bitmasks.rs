use super::*;

const TEMPLATE: &str = r#"
{{vk_flags_attr}}
pub struct {{rs_flags_ident}}({{rs_type}});

impl {{rs_flags_ident}} {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub fn contains(self, rhs: Self) -> bool {
        self.0 & rhs.0 == rhs.0
    }
}

impl std::ops::BitAnd for {{rs_flags_ident}} {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl From<{{rs_flag_bits_ident}}> for {{rs_flags_ident}} {
    fn from(flag_bits: {{rs_flag_bits_ident}}) -> Self {
        Self(flag_bits as {{rs_type}})
    }
}

impl std::ops::BitOr<{{rs_flag_bits_ident}}> for {{rs_flags_ident}} {
    type Output = {{rs_flags_ident}};
    fn bitor(self, rhs: {{rs_flag_bits_ident}}) -> Self::Output {
        Self(self.0 | rhs as {{rs_type}})
    }
}

impl std::fmt::Display for {{rs_flags_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(deprecated)]
        display_flag_bits_{{rs_type}}(f, self.0, &[
            {{rs_flag_bits_idents}}
        ])
    }
}

impl std::fmt::Debug for {{rs_flags_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("{{rs_flags_ident}}")
            .field(&format!("{self}"))
            .finish()
    }
}

{{vk_flag_bits_attr}}
pub enum {{rs_flag_bits_ident}} {
    {{rs_flag_bits_members}}
}

impl From<{{rs_flag_bits_ident}}> for {{rs_type}} {
    fn from(flag_bits: {{rs_flag_bits_ident}}) -> Self {
        flag_bits as {{rs_type}}
    }
}

impl std::ops::BitOr for {{rs_flag_bits_ident}} {
    type Output = {{rs_flags_ident}};
    fn bitor(self, rhs: Self) -> Self::Output {
        {{rs_flags_ident}}(self as {{rs_type}} | rhs as {{rs_type}})
    }
}

impl std::ops::BitOr<{{rs_flags_ident}}> for {{rs_flag_bits_ident}} {
    type Output = {{rs_flags_ident}};
    fn bitor(self, rhs: {{rs_flags_ident}}) -> Self::Output {
        {{rs_flags_ident}}(self as {{rs_type}} | rhs.0)
    }
}

impl std::fmt::Display for {{rs_flag_bits_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
"#;

const TEMPLATE_NO_FLAG_BITS: &str = r#"
{{vk_flags_attr}}
pub struct {{rs_flags_ident}}({{rs_type}});

impl {{rs_flags_ident}} {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }
}
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    let bitvalues_map = {
        let mut map = HashMap::new();
        for registry_enum in &ctx.registry.enums {
            let registry::EnumType::Bitmask = registry_enum.ty else {
                continue;
            };
            map.insert(registry_enum.name.as_str(), registry_enum);
        }
        map
    };

    let deprecations = {
        let mut map = HashMap::new();
        map.insert(
            "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT",
            "Replace with: `vk::PipelineStageFlagBits2::None`",
        );
        map.insert(
            "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT",
            "Replace with: `vk::PipelineStageFlagBits2::AllCommands`",
        );
        map
    };

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Bitmask {
            ty,
            requires,
            bitvalues,
        } = &registry_type.category else {
            continue;
        };

        let vk_flags_ident = &registry_type.name;
        let rs_flags_ident = translation::vk_simple_type(vk_flags_ident)?;
        let rs_type = match ty.as_str() {
            "VkFlags" => "u32",
            "VkFlags64" => "u64",
            ty => bail!("Unknown bitmask type: {ty}"),
        };

        let bitvalues = {
            if let Some(bitvalues) = bitvalues {
                Some(bitvalues.as_str())
            } else {
                requires.as_ref().map(String::as_str)
            }
        };

        if let Some(bitvalues) = bitvalues {
            let vk_flag_bits_ident = bitvalues;
            let vk_flag_bits_attr = attributes::Builder::new()
                .repr(rs_type)
                .derive("Clone, Copy, PartialEq, Eq, Debug")
                .doc_chapter(ctx.vkspec.type_chapter(vk_flag_bits_ident))
                .doc_br()
                .doc_desc(ctx.vkspec.type_desc(vk_flag_bits_ident))
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_flag_bits_ident))
                .doc_br()
                .doc_ref(vk_flag_bits_ident)
                .build();
            let rs_flag_bits_ident = translation::vk_simple_type(vk_flag_bits_ident)?;
            let vk_flags_attr = attributes::Builder::new()
                .repr("C")
                .derive("Clone, Copy, PartialEq, Eq")
                .doc_chapter(ctx.vkspec.type_chapter(vk_flags_ident))
                .doc_br()
                .doc_desc(ctx.vkspec.type_desc(vk_flags_ident))
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_flags_ident))
                .doc_br()
                .doc_ref(vk_flags_ident)
                .doc_br()
                .doc_ref(vk_flag_bits_ident)
                .build();

            let bitvalues = bitvalues_map.get(bitvalues).context("Missing bitvalues")?;
            let (member_idents, members): (Vec<_>, Vec<_>) = bitvalues
                .members
                .iter()
                .filter_map(|member| {
                    if member.alias.is_some() {
                        return None;
                    }
                    Some((member.name.as_str(), member))
                })
                .unzip();
            let member_idents = translation::vk_bitmask(vk_flag_bits_ident, &member_idents)?;

            let mut rs_flag_bits_members = String::new();
            let mut rs_flag_bits_idents = String::new();
            for (member, rs_member_ident) in members.iter().zip(&member_idents) {
                let vk_member_ident = &member.name;

                let rs_member_value = if let Some(bitpos) = &member.bitpos {
                    let bitpos: u64 = bitpos.parse()?;
                    format!("0b{:0b}", 1_u64 << bitpos)
                } else if let Some(value) = &member.value {
                    if let Some(value) = value.strip_prefix("0x") {
                        let value = u64::from_str_radix(value, 16)?;
                        format!("0x{value:0x}")
                    } else {
                        let value: u64 = value.parse()?;
                        format!("{value}")
                    }
                } else {
                    bail!("Bitmask member has neither bitpos or value.");
                };

                writeln!(
                    rs_flag_bits_members,
                    r#"#[doc = "Translated from: `{vk_member_ident}`"]"#
                )?;
                if let Some(deprecation) = deprecations.get(vk_member_ident.as_str()) {
                    writeln!(
                        rs_flag_bits_members,
                        r#"#[deprecated(note = "{deprecation}")]"#
                    )?;
                }
                writeln!(
                    rs_flag_bits_members,
                    "{rs_member_ident} = {rs_member_value},"
                )?;
                writeln!(
                    rs_flag_bits_idents,
                    "{rs_flag_bits_ident}::{rs_member_ident},",
                )?;
            }
            let mut rs_flag_bits_members = rs_flag_bits_members.trim_end().to_string();
            let mut rs_flag_bits_idents = rs_flag_bits_idents.trim_end().to_string();

            // Special: Rust enums must have at least one variant.
            if member_idents.is_empty() {
                rs_flag_bits_members.push_str("Placeholder = 0b0,");
                rs_flag_bits_idents.push_str(&format!("{rs_flag_bits_ident}::Placeholder,"));
            }

            writeln!(
                str,
                "{}",
                TEMPLATE
                    .replace("{{vk_flag_bits_attr}}", &vk_flag_bits_attr)
                    .replace("{{vk_flags_attr}}", &vk_flags_attr)
                    .replace("{{rs_type}}", rs_type)
                    .replace("{{rs_flag_bits_ident}}", &rs_flag_bits_ident)
                    .replace("{{rs_flags_ident}}", &rs_flags_ident)
                    .replace("{{rs_flag_bits_members}}", &rs_flag_bits_members)
                    .replace("{{rs_flag_bits_idents}}", &rs_flag_bits_idents)
            )?;
        } else {
            let vk_flags_attr = attributes::Builder::new()
                .repr("C")
                .derive("Clone, Copy, PartialEq, Eq, Debug")
                .doc_chapter(ctx.vkspec.type_chapter(vk_flags_ident))
                .doc_br()
                .doc_desc(ctx.vkspec.type_desc(vk_flags_ident))
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_flags_ident))
                .doc_br()
                .doc_ref(vk_flags_ident)
                .build();

            writeln!(
                str,
                "{}",
                TEMPLATE_NO_FLAG_BITS
                    .replace("{{vk_flags_attr}}", &vk_flags_attr)
                    .replace("{{rs_flags_ident}}", &rs_flags_ident)
                    .replace("{{rs_type}}", rs_type)
            )?;
        }
    }

    Ok(str)
}
