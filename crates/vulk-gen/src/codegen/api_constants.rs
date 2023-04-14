use super::*;

const TEMPLATE: &str = r#"{{c_attr}}
pub const {{rs_ident}}: {{rs_type}} = {{rs_value}};
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    let api_constants = ctx
        .registry
        .enums
        .iter()
        .find(|en| en.name == "API Constants")
        .context("Missing API Constants")?;
    for member in &api_constants.members {
        if member.alias.is_some() {
            continue;
        }

        let c_ident = &member.name;
        let c_attr = attributes::Builder::new()
            .doc_desc(ctx.vkspec.type_desc(c_ident))
            .doc_br()
            .doc_ref(c_ident)
            .build();
        let rs_ident = translation::c_define(c_ident)?;
        let rs_type =
            translation::c_type(ctx.c_type_map, member.ty.as_ref().context("Missing type")?)?;
        let rs_value = translation::c_value(member.value.as_ref().context("Missing value")?)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{c_attr}}", &c_attr)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_type}}", rs_type)
                .replace("{{rs_value}}", &rs_value)
        )?;
    }

    Ok(str)
}
