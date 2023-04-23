use super::*;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    let mut commands = HashSet::new();
    for command in &ctx.registry.commands {
        commands.insert(command.name.as_str());
    }

    writeln!(str, "//! ## Commands")?;
    for chapter in &ctx.vkspec.chapters {
        let mut chapter_str = String::new();
        writeln!(chapter_str, "//! ### {}", chapter.heading)?;
        let mut command_count = 0;
        for ty in &chapter.types {
            if ty.ty != "protos" {
                continue;
            }

            let command = ty;
            if !commands.contains(command.name.as_str()) {
                continue;
            }

            let command = translation::vk_simple_function(&command.name)?;
            writeln!(chapter_str, "//! - [`vk::{command}`] {}", ty.desc)?;
            command_count += 1;
        }

        if command_count > 0 {
            write!(str, "{chapter_str}")?;
        }
    }

    writeln!(str, "//! ## Extensions")?;
    for extension in &ctx.registry.extensions {
        if !ctx.manifest.extensions.contains(&extension.name) {
            continue;
        }
        writeln!(
            str,
            "//! - [`{}`]({})",
            extension.name,
            docs::reference_url(&extension.name)
        )?;
    }

    Ok(str)
}
