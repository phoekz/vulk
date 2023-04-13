use super::*;

pub struct CommandGroups<'a> {
    pub loader: Vec<&'a registry::Command>,
    pub instance: Vec<&'a registry::Command>,
    pub device: Vec<&'a registry::Command>,
}

pub fn group_by_loader(registry: &Registry) -> CommandGroups<'_> {
    // Find parents.
    let mut parent_relations = HashMap::new();
    for registry_type in &registry.types {
        let registry::TypeCategory::Handle { parent, .. } = &registry_type.category else {
                    continue;
                };

        parent_relations.insert(&registry_type.name, parent);
    }

    // Group commands.
    let mut loader = vec![];
    let mut instance = vec![];
    let mut device = vec![];
    for command in &registry.commands {
        assert!(!command.params.is_empty());
        let first_param = command.params.first().unwrap();
        if parent_relations.contains_key(&first_param.ty) {
            // Assumption: parent-relations must not have circular
            // dependencies.
            //
            // Special: vkGetInstanceProcAddr and vkGetDeviceProcAddr have a
            // parent that is one level higher than their first parameter
            // suggests.
            let mut curr = match command.name.as_str() {
                "vkGetInstanceProcAddr" => {
                    loader.push(command);
                    continue;
                }
                "vkGetDeviceProcAddr" => {
                    instance.push(command);
                    continue;
                }
                _ => first_param.ty.clone(),
            };
            loop {
                match curr.as_str() {
                    "VkInstance" => {
                        instance.push(command);
                        break;
                    }
                    "VkDevice" => {
                        device.push(command);
                        break;
                    }
                    _ => {
                        let parent = *parent_relations.get(&curr).unwrap();
                        assert!(parent.is_some());
                        curr = parent.clone().unwrap();
                    }
                }
            }
        } else {
            // Assumption: commands with an unrecognizable first param type
            // are assumed to be loadable by the loader-group.
            loader.push(command);
        }
    }

    CommandGroups {
        loader,
        instance,
        device,
    }
}

pub enum WrapperType {
    Default,
    Void,
    VoidResult,
    HandleResult,
}

pub fn wrapper_type(
    translator: &Translator,
    handle_map: &HashSet<&str>,
    command: &registry::Command,
) -> Result<WrapperType> {
    let mutable_params = {
        let mut mutable_params = 0;

        for param in &command.params {
            let param_type = translator.vk_complex_type(&param.ty, &param.text, &None, false)?;
            if param_type.contains("*mut") {
                mutable_params += 1;
            }
        }
        mutable_params
    };

    let output_param = {
        let param = command
            .params
            .last()
            .context("Command must have at least 1 parameter")?;
        let param_type = translator.vk_complex_type(&param.ty, &param.text, &None, false)?;
        let is_handle = handle_map.contains(param.ty.as_str());
        param_type.contains("*mut") && is_handle
    };

    let output_param_is_complex = {
        let param = command
            .params
            .last()
            .context("Command must have at least 1 parameter")?;
        param.len.is_some()
    };

    if command.return_type == "VkResult" {
        if mutable_params == 1 && output_param && !output_param_is_complex {
            Ok(WrapperType::HandleResult)
        } else {
            Ok(WrapperType::VoidResult)
        }
    } else if command.return_type == "void" {
        Ok(WrapperType::Void)
    } else {
        Ok(WrapperType::Default)
    }
}
