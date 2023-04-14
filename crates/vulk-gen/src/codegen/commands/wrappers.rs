use super::*;

const TEMPLATE_PARAM: &str = r#"{{rs_param_ident}}: {{rs_param_type}}"#;
const TEMPLATE_PARAM_IDENT: &str = r#"{{rs_param_ident}}"#;
const TEMPLATE_DEFAULT: &str = r#"
#[must_use]
#[inline]
#[doc = "Chapter: **{{vk_chapter}}**"]
#[doc = "<br>"]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> {{rs_return_type}} {
    (self.{{rs_ident}})({{rs_param_idents}})
}
"#;
const TEMPLATE_VOID: &str = r#"
#[inline]
#[doc = "Chapter: **{{vk_chapter}}**"]
#[doc = "<br>"]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) {
    (self.{{rs_ident}})({{rs_param_idents}});
}
"#;
const TEMPLATE_VOID_RESULT: &str = r#"
#[inline]
#[doc = "Chapter: **{{vk_chapter}}**"]
#[doc = "<br>"]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> Result<(), Error> {
    match (self.{{rs_ident}})({{rs_param_idents}}) {
        vk::Result::Success => Ok(()),
        result => Err(Error::Vulkan(result)),
    }
}
"#;
const TEMPLATE_HANDLE_RESULT: &str = r#"
#[inline]
#[doc = "Chapter: **{{vk_chapter}}**"]
#[doc = "<br>"]
#[doc = "Description: {{vk_desc}}"]
#[doc = "<br>"]
#[doc = "Reference: [`{{vk_ident}}`]({{vk_doc}})"]
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> Result<{{rs_handle_type}}, Error> {
    let mut {{rs_handle_ident}} = std::mem::MaybeUninit::uninit();
    match (self.{{rs_ident}})({{rs_param_idents}}, {{rs_handle_ident}}.as_mut_ptr()) {
        vk::Result::Success => Ok({{rs_handle_ident}}.assume_init()),
        result => Err(Error::Vulkan(result)),
    }
}
"#;

pub struct Rendered {
    pub loader_wrappers: String,
    pub instance_wrappers: String,
    pub device_wrappers: String,
}

pub fn generate(ctx: &GeneratorContext<'_>, groups: &analysis::CommandGroups) -> Result<Rendered> {
    let mut handle_map = HashSet::new();
    for registry_ty in &ctx.registry.types {
        let registry::TypeCategory::Handle { .. } = &registry_ty.category else {
            continue;
        };
        handle_map.insert(registry_ty.name.as_str());
    }

    let loader_wrappers = generate_wrappers(ctx, &handle_map, &groups.loader)?;
    let instance_wrappers = generate_wrappers(ctx, &handle_map, &groups.instance)?;
    let device_wrappers = generate_wrappers(ctx, &handle_map, &groups.device)?;

    Ok(Rendered {
        loader_wrappers,
        instance_wrappers,
        device_wrappers,
    })
}

fn generate_wrappers(
    ctx: &GeneratorContext<'_>,
    handle_map: &HashSet<&str>,
    commands: &[&registry::Command],
) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let vk_chapter = ctx.vkspec.type_chapter(vk_ident);
        let vk_desc = ctx.vkspec.type_desc(vk_ident);
        let vk_doc = docs::reference_url(vk_ident);
        let rs_ident = translation::vk_simple_function(vk_ident)?;
        let rs_ident = translation::vk_simple_ident(&rs_ident)?;
        let vk_return_type = &command.return_type;
        let rs_return_type =
            translation::vk_complex_type(ctx.c_type_map, vk_return_type, &None, &None, true)?;

        let mut rs_params = vec![];
        let mut rs_params_types = vec![];
        for param in &command.params {
            let vk_param_ident = &param.name;
            let rs_param_ident = translation::vk_simple_ident(vk_param_ident)?;
            let vk_param_type = &param.ty;
            let rs_param_type = translation::vk_complex_type(
                ctx.c_type_map,
                vk_param_type,
                &param.text,
                &None,
                true,
            )?;
            rs_params.push(
                TEMPLATE_PARAM
                    .replace("{{rs_param_ident}}", &rs_param_ident)
                    .replace("{{rs_param_type}}", &rs_param_type),
            );
            rs_params_types.push(param.ty.clone());
        }
        let (_, rs_params_lhs) = rs_params.split_last().unwrap();
        let rs_params_lhs = rs_params_lhs.join(",");
        let rs_params = rs_params.join(",");

        let mut rs_param_idents = vec![];
        for param in &command.params {
            let vk_param_ident = &param.name;
            let rs_param_ident = translation::vk_simple_ident(vk_param_ident)?;
            rs_param_idents
                .push(TEMPLATE_PARAM_IDENT.replace("{{rs_param_ident}}", &rs_param_ident));
        }
        let (rs_param_idents_last, rs_param_idents_lhs) = rs_param_idents.split_last().unwrap();
        let rs_param_idents_lhs = rs_param_idents_lhs.join(",");
        let rs_param_idents = rs_param_idents.join(",");

        let (rs_params_type_last, _) = rs_params_types.split_last().unwrap();

        match analysis::wrapper_type(ctx.c_type_map, handle_map, command)? {
            analysis::WrapperType::Default => {
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_DEFAULT
                        .replace("{{vk_chapter}}", vk_chapter)
                        .replace("{{vk_desc}}", vk_desc)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{vk_doc}}", &vk_doc)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_param_idents}}", &rs_param_idents)
                        .replace("{{rs_return_type}}", &rs_return_type)
                )?;
            }
            analysis::WrapperType::Void => {
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_VOID
                        .replace("{{vk_chapter}}", vk_chapter)
                        .replace("{{vk_desc}}", vk_desc)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{vk_doc}}", &vk_doc)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_param_idents}}", &rs_param_idents)
                )?;
            }
            analysis::WrapperType::VoidResult => {
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_VOID_RESULT
                        .replace("{{vk_chapter}}", vk_chapter)
                        .replace("{{vk_desc}}", vk_desc)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{vk_doc}}", &vk_doc)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_param_idents}}", &rs_param_idents)
                )?;
            }
            analysis::WrapperType::HandleResult => {
                let rs_handle_type = translation::vk_complex_type(
                    ctx.c_type_map,
                    rs_params_type_last,
                    &None,
                    &None,
                    true,
                )?;
                let rs_handle_ident = rs_param_idents_last;
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_HANDLE_RESULT
                        .replace("{{vk_chapter}}", vk_chapter)
                        .replace("{{vk_desc}}", vk_desc)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{vk_doc}}", &vk_doc)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params_lhs)
                        .replace("{{rs_param_idents}}", &rs_param_idents_lhs)
                        .replace("{{rs_handle_type}}", &rs_handle_type)
                        .replace("{{rs_handle_ident}}", rs_handle_ident)
                )?;
            }
        }
    }

    Ok(str)
}
