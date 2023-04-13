use super::*;

const TEMPLATE: &str = r#"
#[doc = "Description: {{c_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{c_ident}}`]({{c_doc}})"]
pub const {{rs_ident}}: {{rs_type}} = {{rs_value}};
"#;

pub fn generate(
    registry: &Registry,
    translator: &Translator,
    description_map: &DescriptionMap,
) -> Result<String> {
    let mut str = String::new();

    let api_constants = registry
        .enums
        .iter()
        .find(|en| en.name == "API Constants")
        .context("Missing API Constants")?;
    for member in &api_constants.members {
        if member.alias.is_some() {
            continue;
        }

        let c_ident = &member.name;
        let c_desc = &description_map.get(c_ident).context("Missing desc")?.desc;
        let c_doc = docs::reference_url(c_ident);
        let rs_ident = Translator::c_define(c_ident)?;
        let rs_type = translator.c_type(member.ty.as_ref().context("Missing type")?)?;
        let rs_value = Translator::c_value(member.value.as_ref().context("Missing value")?)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{c_ident}}", c_ident)
                .replace("{{c_desc}}", c_desc)
                .replace("{{c_doc}}", &c_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_type}}", rs_type)
                .replace("{{rs_value}}", &rs_value)
        )?;
    }

    Ok(str)
}
