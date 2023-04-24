use super::*;

const TEMPLATE: &str = r#"
#[cfg(target_family = "windows")]
pub const REQUIRED_INSTANCE_EXTENSION_COUNT: usize = {{instance_count}} + {{instance_windows_count}};
#[cfg(target_family = "windows")]
pub const REQUIRED_DEVICE_EXTENSION_COUNT: usize = {{device_count}} + {{device_windows_count}};
#[cfg(not(target_family = "windows"))]
pub const REQUIRED_INSTANCE_EXTENSION_COUNT: usize = {{instance_count}};
#[cfg(not(target_family = "windows"))]
pub const REQUIRED_DEVICE_EXTENSION_COUNT: usize = {{device_count}};

{{instance_attrs}}
pub const REQUIRED_INSTANCE_EXTENSIONS: [*const std::ffi::c_char; REQUIRED_INSTANCE_EXTENSION_COUNT] = [
    {{instance_exts}}
];

{{device_attrs}}
pub const REQUIRED_DEVICE_EXTENSIONS: [*const std::ffi::c_char; REQUIRED_DEVICE_EXTENSION_COUNT] = [
    {{device_exts}}
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
        let mut instance_exts = String::new();
        let mut instance_count = 0;
        let mut instance_windows_count = 0;
        for &extension in &instance_extensions {
            let vk_ident = &extension.name;
            let attrs = attributes::Builder::new()
                .doc_includes_ext(vk_ident)
                .doc_br()
                .build();
            write!(instance_attrs, "{attrs}")?;
            if let Some(attr) = extension_only_targets_windows(ctx, vk_ident) {
                writeln!(instance_exts, "{attr}")?;
                instance_windows_count += 1;
            } else {
                instance_count += 1;
            }
            writeln!(
                instance_exts,
                "{}",
                TEMPLATE_EXTENSION_STRING.replace("{{name}}", &extension.name)
            )?;
        }

        let mut device_attrs = String::new();
        let mut device_exts = String::new();
        let mut device_count = 0;
        let mut device_windows_count = 0;
        for &extension in &device_extensions {
            let vk_ident = &extension.name;
            let attrs = attributes::Builder::new()
                .doc_includes_ext(vk_ident)
                .doc_br()
                .build();
            write!(device_attrs, "{attrs}")?;
            if let Some(attr) = extension_only_targets_windows(ctx, vk_ident) {
                writeln!(instance_exts, "{attr}")?;
                device_windows_count += 1;
            } else {
                device_count += 1;
            }
            writeln!(
                device_exts,
                "{}",
                TEMPLATE_EXTENSION_STRING.replace("{{name}}", &extension.name)
            )?;
        }

        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{instance_attrs}}", &instance_attrs)
                .replace("{{instance_exts}}", &instance_exts)
                .replace("{{instance_count}}", &format!("{instance_count}"))
                .replace(
                    "{{instance_windows_count}}",
                    &format!("{instance_windows_count}")
                )
                .replace("{{device_attrs}}", &device_attrs)
                .replace("{{device_exts}}", &device_exts)
                .replace("{{device_count}}", &format!("{device_count}"))
                .replace(
                    "{{device_windows_count}}",
                    &format!("{device_windows_count}")
                )
        )?;
    }
    Ok(str)
}

fn extension_only_targets_windows(ctx: &GeneratorContext<'_>, vk_ident: &str) -> Option<String> {
    let platform = ctx.extension_platform_map.get(vk_ident);
    if let Some(platform) = platform {
        if platform == "win32" {
            let attr = attributes::Builder::new()
                .cfg_target_family("windows")
                .build();
            Some(attr)
        } else {
            None
        }
    } else {
        None
    }
}
