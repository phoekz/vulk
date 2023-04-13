use super::*;

const TEMPLATE: &str = r#"
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
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

pub fn generate(registry: &Registry, translator: &Translator) -> Result<String> {
    let mut str = String::new();

    for registry_type in &registry.types {
        let registry::TypeCategory::Union { members, .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_doc = doc::reference_url(vk_ident);
        let rs_ident = Translator::vk_simple_type(vk_ident)?;
        let mut rs_members = String::new();
        for member in members {
            let vk_member_ident = &member.name;
            let vk_member_type = &member.ty;
            let rs_member_ident = Translator::vk_simple_ident(vk_member_ident)?;
            let rs_member_type =
                translator.vk_complex_type(vk_member_type, &member.text, &member.en, false)?;
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
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;
    }

    Ok(str)
}
