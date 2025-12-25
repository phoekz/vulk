use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
#[doc = "**Initialization template**:"]
#[doc = {{rs_init_template}}]
pub struct {{rs_ident}} {
    {{rs_members}}
}
"#;

const TEMPLATE_MEMBER: &str = r#"pub {{rs_member_ident}}: {{rs_member_type}},"#;

const TEMPLATE_RAW_STRING: &str = "r#\"```{{}}```\"#";

const TEMPLATE_INIT: &str = r#"no_run
# use vulk::vk as vk;
# use std::ptr::{null, null_mut};
let {{rs_init_ident}} = vk::{{rs_ident}} {
{{rs_init_members}}
};
"#;

const TEMPLATE_INIT_MEMBER: &str = r#"    {{rs_init_member_ident}}: {{rs_init_member_value}},"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let extend_map = extend_map(&ctx.registry.types);

    let mut str = String::new();
    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Struct { members, .. } = &registry_type.category else {
            continue;
        };

        let (members, collapsed_bitfields) = collapse_bitfields(members)?;

        let vk_ident = &registry_type.name;
        let mut vk_attr = attributes::Builder::new()
            .repr("C")
            .derive("Clone, Copy, Debug")
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .doc_br();
        if let Some(structextends) = extend_map.get(vk_ident.as_str()) {
            for structextend in structextends {
                vk_attr = vk_attr.doc_extend(structextend).doc_br();
            }
        }
        if collapsed_bitfields > 0 {
            vk_attr = vk_attr
                .doc_note(format!(
                    "The original type contained **{collapsed_bitfields}** bitfields which were collapsed by the generator."
                ))
                .doc_br();
        }
        let vk_attr = vk_attr.build();

        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let rs_init_template = {
            let rs_init_ident = translation::vk_simple_ident(&rs_ident)?;
            let mut rs_init_members = String::new();
            for member in &members {
                let vk_init_member_ident = &member.name;
                let rs_init_member_ident = translation::vk_simple_ident(vk_init_member_ident)?;
                let rs_init_member_value = match rs_init_member_ident.as_str() {
                    "s_type" => {
                        format!("vk::StructureType::{rs_ident}")
                    }
                    "p_next" => {
                        let vk_text = member.text.as_ref().context("Missing text")?;
                        let rs_specifier = translation::c_specifier(vk_text)?;
                        match rs_specifier.as_str() {
                            "*const" => "null()".to_string(),
                            "*mut" => "null_mut()".to_string(),
                            specifier => bail!("Unknown specifier {specifier}"),
                        }
                    }
                    _ => {
                        let vk_type = &member.ty;
                        let rs_type = translation::vk_complex_type(
                            ctx.c_type_map,
                            vk_type,
                            member.text.as_ref(),
                            member.en.as_ref(),
                            true,
                        )
                        .with_context(|| format!("Translating type={vk_type}"))?;

                        // Special: Empty Vk*Flags can be initialized with empty().
                        if ctx.empty_flag_bits_map.contains(vk_type) && vk_type.contains("Flags") {
                            format!("{rs_type}::empty()")
                        } else {
                            // Special: prefer initializing Flags as FlagBits, since that is how the Flags are built.
                            let rs_type = rs_type.replace("Flags", "FlagBits");
                            format!("todo!(\"{rs_type}\")")
                        }
                    }
                };
                writeln!(
                    rs_init_members,
                    "{}",
                    TEMPLATE_INIT_MEMBER
                        .replace("{{rs_init_member_ident}}", &rs_init_member_ident)
                        .replace("{{rs_init_member_value}}", &rs_init_member_value)
                )?;
            }
            let rs_init_members = rs_init_members.trim_end();
            TEMPLATE_RAW_STRING.replace(
                "{{}}",
                &TEMPLATE_INIT
                    .replace("{{rs_init_ident}}", &rs_init_ident)
                    .replace("{{rs_ident}}", &rs_ident)
                    .replace("{{rs_init_members}}", rs_init_members),
            )
        };

        let mut rs_members = String::new();
        for member in &members {
            let vk_member_ident = &member.name;
            let rs_member_ident = translation::vk_simple_ident(vk_member_ident)?;
            let vk_member_type = &member.ty;
            let rs_member_type = translation::vk_complex_type(
                ctx.c_type_map,
                vk_member_type,
                member.text.as_ref(),
                member.en.as_ref(),
                false,
            )
            .with_context(|| format!("Translating member type={vk_member_type}"))?;
            writeln!(
                rs_members,
                "{}",
                TEMPLATE_MEMBER
                    .replace("{{rs_member_ident}}", &rs_member_ident)
                    .replace("{{rs_member_type}}", &rs_member_type)
            )?;
        }

        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_attr}}", &vk_attr)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{rs_init_template}}", &rs_init_template)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;
    }

    Ok(str)
}

fn extend_map(types: &[registry::Type]) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    for ty in types {
        let registry::TypeCategory::Struct { structextends, .. } = &ty.category else {
            continue;
        };
        for structextend in structextends {
            if let Some(vec) = map.get_mut(structextend.as_str()) {
                vec.push(ty.name.as_str());
            } else {
                map.insert(structextend.as_str(), vec![ty.name.as_str()]);
            }
        }
    }
    map
}

type CollapsedBitfields = u32;

fn collapse_bitfields(
    members: &[registry::TypeMember],
) -> Result<(Vec<registry::TypeMember>, CollapsedBitfields)> {
    let is_bitfield = |member: &registry::TypeMember| -> bool {
        if let Some(text) = &member.text {
            text.starts_with(':')
        } else {
            false
        }
    };

    let type_bitsize = {
        let mut map = HashMap::new();
        map.insert("uint32_t", 32_u32);
        map.insert("uint64_t", 64_u32);
        map
    };

    let bitfield_size = |member: &registry::TypeMember| -> Result<u32> {
        if let Some(text) = &member.text {
            if let Some(text) = text.strip_prefix(':') {
                let bitsize: u32 = text.parse().with_context(|| format!("Parsing {text}"))?;
                Ok(bitsize)
            } else {
                bail!("Member is not a bitfield");
            }
        } else {
            bail!("Member is not a specifier");
        }
    };

    if members.iter().any(is_bitfield) {
        // Collapse bitfields.
        let mut output_members = vec![];
        let mut pending_type: Option<&str> = None;
        let mut pending_names = vec![];
        let mut pending_bits = vec![];
        let mut remaining_bits = 0;
        let mut collapsed_bitfields = 0;
        for member in members {
            if is_bitfield(member) {
                // Start a new field.
                if remaining_bits == 0 {
                    remaining_bits = *type_bitsize
                        .get(member.ty.as_str())
                        .with_context(|| format!("Unsupported bitfield type={}", member.ty))?;
                    pending_type = Some(member.ty.as_str());
                }

                // Bitfield size.
                let bitfield_size = bitfield_size(member)?;

                // Update pending.
                pending_names.push(member.name.as_str());
                pending_bits.push(bitfield_size);

                // Subtract remaining.
                remaining_bits = remaining_bits.checked_sub(bitfield_size).with_context(|| {
                    format!("Bitfield is too long, remaining_bits={remaining_bits}, bitfield_size={bitfield_size}")
                })?;

                // Flush?
                if remaining_bits == 0 {
                    let ty = std::mem::take(&mut pending_type).unwrap();
                    let names = std::mem::take(&mut pending_names);
                    let bits = std::mem::take(&mut pending_bits);

                    // Attach bitfield size as a suffix.
                    let names = names
                        .into_iter()
                        .zip(bits)
                        .map(|(name, bits)| format!("{name}{bits}"))
                        .collect::<Vec<_>>();

                    // Each word after the first one should start with a capital
                    // letter. This makes sure when the words are joined,
                    // `heck::AsSnakeCase` can split them.
                    let names = names
                        .into_iter()
                        .enumerate()
                        .map(|(index, name)| {
                            if index == 0 {
                                name
                            } else {
                                format!("{}", heck::AsPascalCase(name))
                            }
                        })
                        .collect::<Vec<_>>();

                    // Merged bitfields will have a name such as: `fooAndBar`.
                    let name = names.join("And");

                    output_members.push(registry::TypeMember {
                        name,
                        ty: ty.to_string(),
                        optional: None,
                        comment: None,
                        text: None,
                        en: None,
                    });
                }

                // Stats.
                collapsed_bitfields += 1;
            } else {
                output_members.push(member.clone());
            }
        }
        ensure!(pending_names.is_empty());
        ensure!(pending_bits.is_empty());
        ensure!(remaining_bits == 0);

        Ok((output_members, collapsed_bitfields))
    } else {
        // Passthrough for structures without bitfields.
        Ok((members.iter().map(Clone::clone).collect(), 0))
    }
}
