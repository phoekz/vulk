use super::*;

const TEMPLATE: &str = r#"
{{required_extensions}}
{{win32_extensions}}
{{debugging_extensions}}
"#;

const TEMPLATE_EXTENSIONS: &str = r#"
{{instance_attrs}}
pub const {{array_name}}_INSTANCE_EXTENSIONS: [*const std::ffi::c_char; {{instance_count}}] = [
    {{instance_members}}
];

{{device_attrs}}
pub const {{array_name}}_DEVICE_EXTENSIONS: [*const std::ffi::c_char; {{device_count}}] = [
    {{device_members}}
];
"#;

const TEMPLATE_EXTENSION_STRING: &str = r#"c"{{name}}".as_ptr().cast(),"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let test_win32_extension = |ext: &registry::Extension| -> bool {
        if let Some(platform) = &ext.platform {
            platform == "win32"
        } else {
            false
        }
    };
    let test_debugging_extension = |ext: &registry::Extension| -> bool {
        let None = &ext.platform else {
            return false;
        };
        if let Some(specialuse) = &ext.specialuse {
            specialuse == "debugging"
        } else {
            false
        }
    };

    let mut str = String::new();
    writeln!(
        str,
        "{}",
        TEMPLATE
            .replace(
                "{{required_extensions}}",
                &generate_extension_arrays(ctx, "REQUIRED", |ext| {
                    if test_win32_extension(ext) {
                        return false;
                    }
                    if test_debugging_extension(ext) {
                        return false;
                    }
                    true
                })?
            )
            .replace(
                "{{win32_extensions}}",
                &generate_extension_arrays(ctx, "WIN32", test_win32_extension)?
            )
            .replace(
                "{{debugging_extensions}}",
                &generate_extension_arrays(ctx, "DEBUGGING", test_debugging_extension)?
            )
    )?;
    Ok(str)
}

fn generate_extension_arrays<F>(
    ctx: &GeneratorContext<'_>,
    array_name: &str,
    allowed_extension: F,
) -> Result<String>
where
    F: Fn(&registry::Extension) -> bool,
{
    // Group by type.
    let mut instance_extensions = vec![];
    let mut device_extensions = vec![];
    for extension in &ctx.registry.extensions {
        if !ctx.manifest.extensions.contains(&extension.name) {
            continue;
        }
        if !allowed_extension(extension) {
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

    let generate = |exts: &[&registry::Extension]| {
        let mut attrs = String::new();
        let count: String = format!("{}", exts.len());
        let mut members = String::new();
        for &ext in exts {
            let vk_ident = &ext.name;
            let attr = attributes::Builder::new()
                .doc_includes_ext(vk_ident)
                .doc_br()
                .build();
            write!(attrs, "{attr}")?;

            writeln!(
                members,
                "{}",
                TEMPLATE_EXTENSION_STRING.replace("{{name}}", vk_ident)
            )?;
        }
        Ok::<_, anyhow::Error>((attrs, count, members))
    };
    let (instance_attrs, instance_count, instance_members) = generate(&instance_extensions)?;
    let (device_attrs, device_count, device_members) = generate(&device_extensions)?;

    // Generate.
    let mut str = String::new();
    writeln!(
        str,
        "{}",
        TEMPLATE_EXTENSIONS
            .replace("{{array_name}}", array_name)
            .replace("{{instance_attrs}}", &instance_attrs)
            .replace("{{instance_count}}", &instance_count)
            .replace("{{instance_members}}", &instance_members)
            .replace("{{device_attrs}}", &device_attrs)
            .replace("{{device_count}}", &device_count)
            .replace("{{device_members}}", &device_members)
    )?;
    Ok(str)
}
