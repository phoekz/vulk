use super::*;

const TEMPLATE: &str = r#"
#[doc = "Chapter: **{{vk_chapter}}**"]
#[doc = "<br>"]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub type {{rs_ident}} = *const c_void;
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Funcpointer { .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_chapter = ctx.vkspec.type_chapter(vk_ident);
        let vk_desc = ctx.vkspec.type_desc(vk_ident);
        let vk_doc = docs::reference_url(vk_ident);
        let rs_ident = translation::vk_function_pointer(vk_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_chapter}}", vk_chapter)
                .replace("{{vk_desc}}", vk_desc)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
        )?;
    }

    Ok(str)
}
