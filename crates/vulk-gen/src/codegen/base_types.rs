use super::*;

const TEMPLATE: &str = r#"
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub type {{rs_ident}} = {{rs_type}};
"#;

pub fn generate(registry: &Registry, translator: &Translator) -> Result<String> {
    let mut str = String::new();

    for registry_type in &registry.types {
        let registry::TypeCategory::Basetype { ty, .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_doc = doc::reference_url(vk_ident);
        let rs_ident = Translator::vk_simple_type(vk_ident)?;
        let rs_type = translator.c_type(ty.as_ref().context("Missing type")?)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_type}}", rs_type)
        )?;
    }

    Ok(str)
}
