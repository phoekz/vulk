use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
pub enum {{rs_ident}} {
    {{rs_members}}
}
"#;

const TEMPLATE_MEMBER: &str = r#"{{vk_member_attr}}{{rs_member_ident}} = {{rs_member_value}},"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

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
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let mut rs_members = String::new();
        for member in &registry_enum.members {
            let vk_member_ident = &member.name;
            let vk_member_attr = attributes::Builder::new()
                .doc_translated(vk_member_ident)
                .build();
            let vk_member_value = member.value.as_ref().context("Missing type")?;
            let rs_member_ident = translation::vk_enum_member(vk_ident, vk_member_ident)?;
            let rs_member_value = vk_member_value;
            writeln!(
                rs_members,
                "{}",
                TEMPLATE_MEMBER
                    .replace("{{vk_member_attr}}", &vk_member_attr)
                    .replace("{{rs_member_ident}}", &rs_member_ident)
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
    }

    Ok(str)
}
