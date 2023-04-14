use super::*;

const TEMPLATE: &str = r#"
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub type {{rs_ident}} = unsafe extern "C" fn(
    {{rs_params}}
) {{rs_return}};
"#;

const TEMPLATE_PARAM: &str = r#"{{rs_param_ident}}: {{rs_param_type}}, //"#;

pub fn generate(
    registry: &Registry,
    c_type_map: &CtypeMap,
    description_map: &DescriptionMap,
) -> Result<String> {
    let mut str = String::new();

    for command in &registry.commands {
        let vk_ident = &command.name;
        let vk_desc = &description_map.get(vk_ident).context("Missing desc")?.desc;
        let vk_doc = docs::reference_url(vk_ident);
        let rs_ident = translation::vk_simple_function(vk_ident)?;
        let mut rs_params = String::new();
        for param in &command.params {
            let vk_param_ident = &param.name;
            let rs_param_ident = translation::vk_simple_ident(vk_param_ident)?;
            let vk_param_type = &param.ty;
            let rs_param_type =
                translation::vk_complex_type(c_type_map, vk_param_type, &param.text, &None, false)?;
            writeln!(
                rs_params,
                "{}",
                TEMPLATE_PARAM
                    .replace("{{rs_param_ident}}", &rs_param_ident)
                    .replace("{{rs_param_type}}", &rs_param_type)
            )?;
        }
        let vk_return_type = &command.return_type;
        let rs_return_type =
            translation::vk_complex_type(c_type_map, vk_return_type, &None, &None, false)?;
        let rs_return = if rs_return_type == "c_void" {
            String::new()
        } else {
            format!("-> {rs_return_type}")
        };
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_desc}}", vk_desc)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_params}}", &rs_params)
                .replace("{{rs_return}}", &rs_return)
        )?;
    }

    Ok(str)
}
