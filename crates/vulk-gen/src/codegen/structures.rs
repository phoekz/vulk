use super::*;

const TEMPLATE: &str = r#"
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
{{vk_structextends_docs}}
#[doc = "<br>"]
#[doc = "Initialization template:"]
#[doc = {{rs_init_template}}]
pub struct {{rs_ident}} {
    {{rs_members}}
}
"#;

const TEMPLATE_MEMBER: &str = r#"pub {{rs_member_ident}}: {{rs_member_type}},"#;

const TEMPLATE_STRUCTEXTENDS: &str = r#"#[doc = "<br>"]
#[doc = "Extendable by: [`{{vk_structextends_ident}}`]({{vk_structextends_doc}})"]"#;

const TEMPLATE_RAW_STRING: &str = "r#\"```{{}}```\"#";

const TEMPLATE_INIT: &str = r#"
let {{rs_init_ident}} = vk::{{rs_ident}} {
{{rs_init_members}}
};
"#;

const TEMPLATE_INIT_MEMBER: &str = r#"    {{rs_init_member_ident}}: {{rs_init_member_value}},"#;

pub fn generate(
    registry: &Registry,
    c_type_map: &CtypeMap,
    description_map: &DescriptionMap,
) -> Result<String> {
    let mut extend_map: HashMap<_, Vec<_>> = HashMap::new();
    for registry_type in &registry.types {
        let registry::TypeCategory::Struct { structextends, .. } = &registry_type.category else {
            continue;
        };

        for structextend in structextends {
            if let Some(vec) = extend_map.get_mut(structextend.as_str()) {
                vec.push(registry_type.name.as_str());
            } else {
                extend_map.insert(structextend.as_str(), vec![registry_type.name.as_str()]);
            }
        }
    }

    let mut str = String::new();
    for registry_type in &registry.types {
        let registry::TypeCategory::Struct { members, .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_desc = &description_map.get(vk_ident).context("Missing desc")?.desc;
        let vk_doc = docs::reference_url(vk_ident);
        let mut vk_structextends_docs = String::new();
        if let Some(structextends) = extend_map.get(vk_ident.as_str()) {
            for structextend in structextends {
                let vk_structextends_ident = structextend;
                let vk_structextends_doc = &docs::reference_url(vk_structextends_ident);
                writeln!(
                    vk_structextends_docs,
                    "{}",
                    TEMPLATE_STRUCTEXTENDS
                        .replace("{{vk_structextends_ident}}", vk_structextends_ident)
                        .replace("{{vk_structextends_doc}}", vk_structextends_doc)
                )?;
            }
        }

        let rs_ident = translation::vk_simple_type(vk_ident)?;
        let rs_init_template = {
            let rs_init_ident = translation::vk_simple_ident(&rs_ident)?;
            let mut rs_init_members = String::new();
            for member in members {
                let vk_init_member_ident = &member.name;
                let rs_init_member_ident = translation::vk_simple_ident(vk_init_member_ident)?;
                let rs_init_member_value = match rs_init_member_ident.as_str() {
                    "s_type" => {
                        format!("vk::StructureType::{rs_ident}")
                    }
                    "p_next" => {
                        let vk_text = member.text.as_ref().context("Missing text")?;
                        let rs_specifier = translation::c_specifier(vk_text)?;
                        match rs_specifier.as_str() {
                            "*const" => "null()".to_string(),
                            "*mut" => "null_mut()".to_string(),
                            specifier => bail!("Unknown specifier {specifier}"),
                        }
                    }
                    _ => {
                        let vk_type = &member.ty;
                        let rs_type = translation::vk_complex_type(
                            c_type_map,
                            vk_type,
                            &member.text,
                            &member.en,
                            false,
                        )?;
                        format!("todo!(\"{rs_type}\")")
                    }
                };
                writeln!(
                    rs_init_members,
                    "{}",
                    TEMPLATE_INIT_MEMBER
                        .replace("{{rs_init_member_ident}}", &rs_init_member_ident)
                        .replace("{{rs_init_member_value}}", &rs_init_member_value)
                )?;
            }
            let rs_init_members = rs_init_members.trim_end();
            TEMPLATE_RAW_STRING.replace(
                "{{}}",
                &TEMPLATE_INIT
                    .replace("{{rs_init_ident}}", &rs_init_ident)
                    .replace("{{rs_ident}}", &rs_ident)
                    .replace("{{rs_init_members}}", rs_init_members),
            )
        };

        let mut rs_members = String::new();
        for member in members {
            let vk_member_ident = &member.name;
            let rs_member_ident = translation::vk_simple_ident(vk_member_ident)?;
            let vk_member_type = &member.ty;
            let rs_member_type = translation::vk_complex_type(
                c_type_map,
                vk_member_type,
                &member.text,
                &member.en,
                false,
            )?;
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
                .replace("{{vk_desc}}", vk_desc)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{vk_doc}}", &vk_doc)
                .replace("{{vk_structextends_docs}}", &vk_structextends_docs)
                .replace("{{rs_init_template}}", &rs_init_template)
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_members}}", &rs_members)
        )?;
    }

    Ok(str)
}
