use super::*;

const TEMPLATE: &str = r#"
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub type {{rs_ident}} = {{rs_type}};
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Basetype { ty, .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_desc = ctx.vkspec.type_desc(vk_ident).context("Missing desc")?;
        let vk_doc = docs::reference_url(vk_ident);
        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let rs_type = translation::c_type(ctx.c_type_map, ty.as_ref().context("Missing type")?)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_desc}}", vk_desc)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_type}}", rs_type)
        )?;
    }

    Ok(str)
}
