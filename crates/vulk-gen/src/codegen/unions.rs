use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
pub union {{rs_ident}} {
    {{rs_members}}
}

impl std::fmt::Debug for {{rs_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("{{rs_ident}}").finish()
    }
}
"#;

const TEMPLATE_MEMBER: &str = r#"pub {{rs_member_ident}}: {{rs_member_type}},"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Union { members, .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_attr = attributes::Builder::new()
            .repr("C")
            .derive("Clone, Copy")
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let mut rs_members = String::new();
        for member in members {
            let vk_member_ident = &member.name;
            let vk_member_type = &member.ty;
            let rs_member_ident = translation::vk_simple_ident(vk_member_ident)?;
            let rs_member_type = translation::vk_complex_type(
                ctx.c_type_map,
                vk_member_type,
                &member.text,
                &member.en,
                false,
            )?;
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
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;
    }

    Ok(str)
}
