use super::*;

const TEMPLATE: &str = r#"
{{instance_attrs}}
pub const REQUIRED_INSTANCE_EXTENSIONS: [*const std::ffi::c_char; {{instance_count}}] = [
    {{instances}}
];

{{device_attrs}}
pub const REQUIRED_DEVICE_EXTENSIONS: [*const std::ffi::c_char; {{device_count}}] = [
    {{devices}}
];
"#;

const TEMPLATE_EXTENSION_STRING: &str = r#"b"{{name}}\0".as_ptr().cast(),"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    // Group extensions by type.
    let mut instance_extensions = vec![];
    let mut device_extensions = vec![];
    for extension in &ctx.registry.extensions {
        if !ctx.manifest.extensions.contains(&extension.name) {
            continue;
        }
        let Some(ty) = &extension.ty else {
            continue;
        };

        match ty.as_str() {
            "instance" => {
                instance_extensions.push(extension);
            }
            "device" => {
                device_extensions.push(extension);
            }
            ty => {
                bail!("Unknown extension type={ty}");
            }
        }
    }

    // Generate.
    let mut str = String::new();
    {
        let mut instance_attrs = String::new();
        let mut instances = String::new();
        for &extension in &instance_extensions {
            let vk_ident = &extension.name;
            let attrs = attributes::Builder::new()
                .doc_includes_ext(vk_ident)
                .doc_br()
                .build();
            write!(instance_attrs, "{attrs}")?;
            writeln!(
                instances,
                "{}",
                TEMPLATE_EXTENSION_STRING.replace("{{name}}", &extension.name)
            )?;
        }

        let mut device_attrs = String::new();
        let mut devices = String::new();
        for &extension in &device_extensions {
            let vk_ident = &extension.name;
            let attrs = attributes::Builder::new()
                .doc_includes_ext(vk_ident)
                .doc_br()
                .build();
            write!(device_attrs, "{attrs}")?;
            writeln!(
                devices,
                "{}",
                TEMPLATE_EXTENSION_STRING.replace("{{name}}", &extension.name)
            )?;
        }

        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{instance_attrs}}", &instance_attrs)
                .replace("{{instances}}", &instances)
                .replace(
                    "{{instance_count}}",
                    &format!("{}", instance_extensions.len())
                )
                .replace("{{device_attrs}}", &device_attrs)
                .replace("{{devices}}", &devices)
                .replace("{{device_count}}", &format!("{}", device_extensions.len()))
        )?;
    }
    Ok(str)
}
