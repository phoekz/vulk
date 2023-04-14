use super::*;

const TEMPLATE: &str = r#"
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub enum {{rs_ident}} {
    {{rs_members}}
}
"#;

const TEMPLATE_MEMBER: &str =
    r#"#[doc = "Translated from: `{{vk_ident}}`"] {{rs_member_ident}} = {{rs_member_value}},"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_enum in &ctx.registry.enums {
        let registry::EnumType::Enum = registry_enum.ty else {
                continue;
        };

        let vk_ident = &registry_enum.name;
        let vk_desc = ctx.vkspec.type_desc(vk_ident).context("Missing desc")?;
        let vk_doc = docs::reference_url(vk_ident);
        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let mut rs_members = String::new();
        for member in &registry_enum.members {
            let vk_member_ident = &member.name;
            let vk_member_value = member.value.as_ref().context("Missing type")?;
            let rs_member_ident = translation::vk_enum_member(vk_ident, vk_member_ident)?;
            let rs_member_value = vk_member_value;
            writeln!(
                rs_members,
                "{}",
                TEMPLATE_MEMBER
                    .replace("{{vk_ident}}", vk_member_ident)
                    .replace("{{rs_member_ident}}", &rs_member_ident)
                    .replace("{{rs_member_value}}", rs_member_value)
            )?;
        }
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_desc}}", vk_desc)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;
    }

    Ok(str)
}
