use super::*;

const TEMPLATE_STRUCT_MEMBER: &str = r#"{{rs_ident}}: vk::{{rs_type}},"#;
const TEMPLATE_LOADER: &str = r#"{{rs_ident}}: std::mem::transmute(load(b"{{vk_ident}}\0")?),"#;

pub struct Rendered {
    pub loader_struct_members: String,
    pub instance_struct_members: String,
    pub device_struct_members: String,
    pub loader_loaders: String,
    pub instance_loaders: String,
    pub device_loaders: String,
}

pub fn generate(
    _registry: &Registry,
    _translator: &Translator,
    groups: &analysis::CommandGroups,
) -> Result<Rendered> {
    let loader_struct_members = generate_struct_members(&groups.loader)?;
    let instance_struct_members = generate_struct_members(&groups.instance)?;
    let device_struct_members = generate_struct_members(&groups.device)?;
    let loader_loaders = generate_loaders(&groups.loader)?;
    let instance_loaders = generate_loaders(&groups.instance)?;
    let device_loaders = generate_loaders(&groups.device)?;
    Ok(Rendered {
        loader_struct_members,
        instance_struct_members,
        device_struct_members,
        loader_loaders,
        instance_loaders,
        device_loaders,
    })
}

fn generate_struct_members(commands: &[&registry::Command]) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let rs_ident = Translator::vk_simple_function(vk_ident)?;
        let rs_ident = Translator::vk_simple_ident(&rs_ident)?;
        let rs_type = Translator::vk_simple_function(vk_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE_STRUCT_MEMBER
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{rs_type}}", &rs_type)
        )?;
    }

    Ok(str)
}

fn generate_loaders(commands: &[&registry::Command]) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let rs_ident = Translator::vk_simple_function(vk_ident)?;
        let rs_ident = Translator::vk_simple_ident(&rs_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE_LOADER
                .replace("{{rs_ident}}", &rs_ident)
                .replace("{{vk_ident}}", vk_ident)
        )?;
    }

    Ok(str)
}
