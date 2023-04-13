use super::*;

const TEMPLATE: &str = r#"
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub struct {{rs_ident}}(u64);

impl {{rs_ident}} {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}
"#;

pub fn generate(registry: &Registry, _translator: &Translator) -> Result<String> {
    let mut str = String::new();

    for registry_type in &registry.types {
        let registry::TypeCategory::Handle { .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_doc = doc::reference_url(vk_ident);
        let rs_ident = Translator::vk_simple_type(vk_ident)?;
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
