use super::*;

const TEMPLATE_STRUCT_MEMBER: &str = r#"pub {{rs_ident}}: Option<vk::{{rs_type}}>,"#;
const TEMPLATE_LOADER: &str =
    r#"{{rs_ident}}: load(b"{{vk_ident}}\0").map(|f| std::mem::transmute(f)),"#;

pub struct Rendered {
    pub init_struct_members: String,
    pub instance_struct_members: String,
    pub device_struct_members: String,
    pub init_loaders: String,
    pub instance_loaders: String,
    pub device_loaders: String,
}

pub fn generate(ctx: &GeneratorContext<'_>, groups: &analysis::CommandGroups) -> Result<Rendered> {
    let init_struct_members = generate_struct_members(ctx, &groups.init)?;
    let instance_struct_members = generate_struct_members(ctx, &groups.instance)?;
    let device_struct_members = generate_struct_members(ctx, &groups.device)?;
    let init_loaders = generate_loaders(ctx, &groups.init)?;
    let instance_loaders = generate_loaders(ctx, &groups.instance)?;
    let device_loaders = generate_loaders(ctx, &groups.device)?;
    Ok(Rendered {
        init_struct_members,
        instance_struct_members,
        device_struct_members,
        init_loaders,
        instance_loaders,
        device_loaders,
    })
}

fn generate_struct_members(
    _ctx: &GeneratorContext<'_>,
    commands: &[&registry::Command],
) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let rs_ident = translation::vk_simple_function(vk_ident)?;
        let rs_ident = translation::vk_simple_ident(&rs_ident)?;
        let rs_type = translation::vk_simple_function(vk_ident)?;
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

fn generate_loaders(ctx: &GeneratorContext<'_>, commands: &[&registry::Command]) -> Result<String> {
    let mut str = String::new();

    for command in commands {
        let vk_ident = &command.name;
        let rs_ident = translation::vk_simple_function(vk_ident)?;
        let rs_ident = translation::vk_simple_ident(&rs_ident)?;
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
