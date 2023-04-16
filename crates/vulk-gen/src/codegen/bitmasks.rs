use super::*;

const TEMPLATE: &str = r#"
bitflags! {
{{vk_flags_attr}}
    pub struct {{rs_flags_ident}}: {{rs_flags_type}} {
{{rs_bits_members}}
    }
}

{{vk_bits_attr}}
pub type {{rs_bits_ident}} = {{rs_flags_ident}};
"#;

const TEMPLATE_NO_BITS: &str = r#"
bitflags! {
{{vk_flags_attr}}
    pub struct {{rs_flags_ident}}: {{rs_flags_type}} {

    }
}
"#;

const TEMPLATE_MEMBER: &str = r#"        #[doc = "Translated from: `{{vk_member_ident}}`"]
        const {{rs_member_ident}} = {{rs_member_value}};"#;

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

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Bitmask {
            ty,
            requires,
            bitvalues,
        } = &registry_type.category else
        {
            continue;
        };

        let vk_flags_ident = &registry_type.name;
        let vk_flags_chapter = ctx.vkspec.type_chapter(vk_flags_ident);
        let vk_flags_desc = ctx.vkspec.type_desc(vk_flags_ident);
        let rs_flags_ident = translation::vk_simple_type(vk_flags_ident)?;
        let rs_flags_type = match ty.as_str() {
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
            let vk_bits_ident = bitvalues;
            let vk_bits_attr = attributes::Builder::new()
                .doc_chapter(ctx.vkspec.type_chapter(vk_bits_ident))
                .doc_br()
                .doc_desc(ctx.vkspec.type_desc(vk_bits_ident))
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_bits_ident))
                .doc_br()
                .doc_ref(vk_bits_ident)
                .build();
            let rs_bits_ident = translation::vk_simple_type(vk_bits_ident)?;

            let bitvalues = bitvalues_map.get(bitvalues).context("Missing bitvalues")?;
            let mut rs_bits_members = String::new();
            for member in &bitvalues.members {
                if member.alias.is_some() {
                    continue;
                }

                let vk_member_ident = &member.name;
                let rs_member_ident =
                    translation::vk_bitmask_member(vk_flags_ident, vk_member_ident)?;

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
                    rs_bits_members,
                    "{}",
                    TEMPLATE_MEMBER
                        .replace("{{vk_member_ident}}", vk_member_ident)
                        .replace("{{rs_member_ident}}", &rs_member_ident)
                        .replace("{{rs_member_value}}", &rs_member_value)
                )?;
            }
            let rs_bits_members = rs_bits_members.trim_end();

            let vk_flags_attr = attributes::Builder::new()
                .repr("C")
                .derive("Clone, Copy, PartialEq, Eq, Debug")
                .doc_chapter(vk_flags_chapter)
                .doc_br()
                .doc_desc(vk_flags_desc)
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_flags_ident))
                .doc_br()
                .doc_ref(vk_flags_ident)
                .doc_br()
                .doc_ref(vk_bits_ident)
                .indent()
                .build();

            writeln!(
                str,
                "{}",
                TEMPLATE
                    .replace("{{vk_flags_attr}}", &vk_flags_attr)
                    .replace("{{vk_flags_ident}}", vk_flags_ident)
                    .replace("{{rs_flags_ident}}", &rs_flags_ident)
                    .replace("{{rs_flags_type}}", rs_flags_type)
                    .replace("{{vk_bits_attr}}", &vk_bits_attr)
                    .replace("{{vk_bits_ident}}", vk_bits_ident)
                    .replace("{{rs_bits_ident}}", &rs_bits_ident)
                    .replace("{{rs_bits_members}}", rs_bits_members)
            )?;
        } else {
            let vk_flags_attr = attributes::Builder::new()
                .repr("C")
                .derive("Clone, Copy, PartialEq, Eq, Debug")
                .doc_chapter(vk_flags_chapter)
                .doc_br()
                .doc_desc(vk_flags_desc)
                .doc_br()
                .doc_provided(ctx.provided_by_map.get(vk_flags_ident))
                .doc_br()
                .doc_ref(vk_flags_ident)
                .indent()
                .build();

            writeln!(
                str,
                "{}",
                TEMPLATE_NO_BITS
                    .replace("{{vk_flags_attr}}", &vk_flags_attr)
                    .replace("{{vk_flags_ident}}", vk_flags_ident)
                    .replace("{{rs_flags_ident}}", &rs_flags_ident)
                    .replace("{{rs_flags_type}}", rs_flags_type)
            )?;
        }
    }

    Ok(str)
}
