use super::*;

const TEMPLATE: &str = r#"
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub type {{rs_ident}} = *const c_void;
"#;

pub fn generate(registry: &Registry, _translator: &Translator) -> Result<String> {
    let mut str = String::new();

    for registry_type in &registry.types {
        let registry::TypeCategory::Funcpointer { .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_doc = doc::reference_url(vk_ident);
        let rs_ident = Translator::vk_function_pointer(vk_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
        )?;
    }

    Ok(str)
}
