use super::*;

// Wrapper requirements
// - Must be `pub` to be accessible outside the library.
// - Must be `unsafe` because all Vulkan functions are unsafe.
// - Must include `&self` as the first parameter because the function pointers are stored in loaders.

const TEMPLATE_PARAM: &str = r#"{{rs_param_ident}}: {{rs_param_type}}"#;
const TEMPLATE_PARAM_IDENT: &str = r#"{{rs_param_ident}}"#;
const TEMPLATE_IDENTITY: &str = r#"{{vk_attr}}
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> {{rs_return_type}} {
    (self.fns.{{rs_ident}})({{rs_params_idents}})
}
"#;
const TEMPLATE_IDENTITY_VOID: &str = r#"{{vk_attr}}
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) {
    (self.fns.{{rs_ident}})({{rs_params_idents}});
}
"#;
const TEMPLATE_UNIT_RESULT: &str = r#"{{vk_attr}}
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> Result<(), Error> {
    match (self.fns.{{rs_ident}})({{rs_params_idents}}) {
        vk::Result::Success => Ok(()),
        result => Err(Error::Vulkan(result)),
    }
}
"#;
const TEMPLATE_OUTPUT_RESULT: &str = r#"{{vk_attr}}
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> Result<{{rs_output_type}}, Error> {
    let mut {{rs_output_ident}} = std::mem::MaybeUninit::uninit();
    match (self.fns.{{rs_ident}})({{rs_params_idents}}, {{rs_output_ident}}.as_mut_ptr()) {
        vk::Result::Success => Ok({{rs_output_ident}}.assume_init()),
        result => Err(Error::Vulkan(result)),
    }
}
"#;
const TEMPLATE_OUTPUT: &str = r#"{{vk_attr}}
pub unsafe fn {{rs_ident}}(&self, {{rs_params}}) -> {{rs_output_type}} {
    let mut {{rs_output_ident}} = std::mem::MaybeUninit::uninit();
    (self.fns.{{rs_ident}})({{rs_params_idents}}, {{rs_output_ident}}.as_mut_ptr());
    {{rs_output_ident}}.assume_init()
}
"#;

pub struct Rendered {
    pub init_wrappers: String,
    pub instance_wrappers: String,
    pub device_wrappers: String,
}

pub fn generate(ctx: &GeneratorContext<'_>, groups: &analysis::CommandGroups) -> Result<Rendered> {
    let mut base_type_map = HashSet::new();
    let mut handle_map = HashSet::new();
    let mut s_type_map = HashSet::new();
    for registry_ty in &ctx.registry.types {
        let name = registry_ty.name.as_str();
        match &registry_ty.category {
            registry::TypeCategory::Basetype { .. } => {
                base_type_map.insert(name);
            }
            registry::TypeCategory::Handle { .. } => {
                handle_map.insert(name);
            }
            registry::TypeCategory::Struct { members, .. } => {
                for member in members {
                    if member.name == "sType" {
                        s_type_map.insert(name);
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    let init_wrappers = generate_wrappers(
        ctx,
        &base_type_map,
        &handle_map,
        &s_type_map,
        &groups.init,
        false,
    )?;
    let instance_wrappers = generate_wrappers(
        ctx,
        &base_type_map,
        &handle_map,
        &s_type_map,
        &groups.instance,
        true,
    )?;
    let device_wrappers = generate_wrappers(
        ctx,
        &base_type_map,
        &handle_map,
        &s_type_map,
        &groups.device,
        true,
    )?;

    Ok(Rendered {
        init_wrappers,
        instance_wrappers,
        device_wrappers,
    })
}

fn generate_wrappers(
    ctx: &GeneratorContext<'_>,
    base_type_map: &HashSet<&str>,
    handle_map: &HashSet<&str>,
    s_type_map: &HashSet<&str>,
    commands: &[&registry::Command],
    can_inline_handles: bool,
) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let vk_attr = attributes::Builder::new()
            .inline()
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_simple_function(vk_ident)?;
        let rs_ident = translation::vk_simple_ident(&rs_ident)?;
        let vk_return_type = &command.return_type;
        let rs_return_type =
            translation::vk_complex_type(ctx.c_type_map, vk_return_type, &None, &None, true)?;

        let inline_handles = {
            let first_type = &command.params[0].ty;
            let function = &command.name;
            let first_type_is_special = first_type == "VkInstance" || first_type == "VkDevice";
            let function_is_special =
                function == "vkGetInstanceProcAddr" || function == "vkGetDeviceProcAddr";
            if can_inline_handles {
                if function_is_special {
                    false
                } else {
                    first_type_is_special
                }
            } else {
                false
            }
        };

        let (rs_params, vk_params_types, rs_params_idents) = {
            let mut rs_params = vec![];
            let mut vk_params_types = vec![];
            let mut rs_params_idents = vec![];
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

                vk_params_types.push((param.ty.clone(), param.text.clone()));

                // Special: hide allocation callbacks from parameter list,
                // always call the function with std::ptr::null().
                if param.ty == "VkAllocationCallbacks" {
                    rs_params_idents.push("std::ptr::null()".to_string());
                } else {
                    rs_params.push(
                        TEMPLATE_PARAM
                            .replace("{{rs_param_ident}}", &rs_param_ident)
                            .replace("{{rs_param_type}}", &rs_param_type),
                    );
                    rs_params_idents
                        .push(TEMPLATE_PARAM_IDENT.replace("{{rs_param_ident}}", &rs_param_ident));
                }
            }
            if inline_handles {
                rs_params.remove(0);
                vk_params_types.remove(0);
                rs_params_idents.remove(0);
                rs_params_idents.insert(0, "self.handle".to_string());
            }
            (rs_params, vk_params_types, rs_params_idents)
        };

        let rs_params_lhs = if let Some((_, rs_params_lhs)) = rs_params.split_last() {
            Some(rs_params_lhs.join(","))
        } else {
            None
        };
        let rs_params = rs_params.join(",");
        let (rs_params_idents_last, rs_params_idents_lhs) = rs_params_idents.split_last().unwrap();
        let rs_params_idents_lhs = rs_params_idents_lhs.join(",");
        let rs_params_idents = rs_params_idents.join(",");
        let vk_params_type_last =
            if let Some((vk_params_type_last, _)) = vk_params_types.split_last() {
                Some(vk_params_type_last)
            } else {
                None
            };

        if let Some(attr) = command_only_targets_windows(ctx, vk_ident) {
            writeln!(str, "{attr}")?;
        }
        match analysis::wrapper_type(
            ctx.c_type_map,
            base_type_map,
            handle_map,
            s_type_map,
            command,
        )? {
            analysis::WrapperType::Identity => {
                let vk_attr = attributes::Builder::new().must_use().raw(vk_attr).build();
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_IDENTITY
                        .replace("{{vk_attr}}", &vk_attr)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_params_idents}}", &rs_params_idents)
                        .replace("{{rs_return_type}}", &rs_return_type)
                )?;
            }
            analysis::WrapperType::IdentityVoid => {
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_IDENTITY_VOID
                        .replace("{{vk_attr}}", &vk_attr)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_params_idents}}", &rs_params_idents)
                )?;
            }
            analysis::WrapperType::UnitResult => {
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_UNIT_RESULT
                        .replace("{{vk_attr}}", &vk_attr)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params)
                        .replace("{{rs_params_idents}}", &rs_params_idents)
                )?;
            }
            analysis::WrapperType::OutputResult => {
                let (vk_type, vk_text) = vk_params_type_last.unwrap();
                let vk_text = vk_text_dereference_mut_ptr(vk_text)?;
                let rs_output_type =
                    translation::vk_complex_type(ctx.c_type_map, vk_type, &vk_text, &None, true)?;
                let rs_output_ident = rs_params_idents_last;
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_OUTPUT_RESULT
                        .replace("{{vk_attr}}", &vk_attr)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params_lhs.unwrap())
                        .replace("{{rs_params_idents}}", &rs_params_idents_lhs)
                        .replace("{{rs_output_type}}", &rs_output_type)
                        .replace("{{rs_output_ident}}", rs_output_ident)
                )?;
            }
            analysis::WrapperType::Output => {
                let vk_attr = attributes::Builder::new().must_use().raw(vk_attr).build();
                let (vk_type, vk_text) = vk_params_type_last.unwrap();
                let vk_text = vk_text_dereference_mut_ptr(vk_text)?;
                let rs_output_type =
                    translation::vk_complex_type(ctx.c_type_map, vk_type, &vk_text, &None, true)?;
                let rs_output_ident = rs_params_idents_last;
                writeln!(
                    str,
                    "{}",
                    TEMPLATE_OUTPUT
                        .replace("{{vk_attr}}", &vk_attr)
                        .replace("{{vk_ident}}", vk_ident)
                        .replace("{{rs_ident}}", &rs_ident)
                        .replace("{{rs_params}}", &rs_params_lhs.unwrap())
                        .replace("{{rs_params_idents}}", &rs_params_idents_lhs)
                        .replace("{{rs_output_type}}", &rs_output_type)
                        .replace("{{rs_output_ident}}", rs_output_ident)
                )?;
            }
        }
    }

    Ok(str)
}

fn vk_text_dereference_mut_ptr(vk_text: &Option<String>) -> Result<Option<String>> {
    let vk_text = if let Some(vk_text) = vk_text {
        ensure!(vk_text.chars().all(|c| c == '*'));
        ensure!((1..=2).contains(&vk_text.len()));
        let vk_text = vk_text.strip_prefix('*').unwrap();
        if vk_text.is_empty() {
            None
        } else {
            Some(vk_text.to_string())
        }
    } else {
        None
    };
    Ok(vk_text)
}
