use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
pub type {{rs_ident}} = *const c_void;
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Funcpointer { .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_attr = attributes::Builder::new()
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_function_pointer(vk_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_attr}}", &vk_attr)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{rs_ident}}", &rs_ident)
        )?;
    }

    Ok(str)
}
