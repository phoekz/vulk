use super::*;

mod api_constants;
mod attributes;
mod base_types;
mod bitmasks;
mod commands;
mod enumerations;
mod function_pointers;
mod handles;
mod outputs;
mod structures;
mod toc;
mod translation;
mod unions;

use registry::CtypeMap;
use std::fmt::Write;

pub struct GeneratorContext<'a> {
    registry: &'a Registry,
    c_type_map: &'a CtypeMap,
    provided_by_map: &'a ProvidedByMap,
    vkspec: &'a docs::Vkspec,
}

pub fn generate(registry: &Registry, vkspec: &docs::Vkspec, vulk_lib_dir: &Path) -> Result<()> {
    // Generate.
    let c_type_map = registry::c_type_map();
    let provided_by_map = ProvidedByMap::new(registry);
    let command_groups = commands::analysis::group_by_loader(registry);
    let ctx = GeneratorContext {
        registry,
        c_type_map: &c_type_map,
        provided_by_map: &provided_by_map,
        vkspec,
    };
    let api_constants = api_constants::generate(&ctx).context("Generating api_constants")?;
    let base_types = base_types::generate(&ctx).context("Generating base_types")?;
    let function_pointers =
        function_pointers::generate(&ctx).context("Generating function_pointers")?;
    let handles = handles::generate(&ctx).context("Generating handles")?;
    let enumerations = enumerations::generate(&ctx).context("Generating enumerations")?;
    let bitmasks = bitmasks::generate(&ctx).context("Generating bitmasks")?;
    let structures = structures::generate(&ctx).context("Generating structures")?;
    let unions = unions::generate(&ctx).context("Generating unions")?;
    let command_types = commands::types::generate(&ctx).context("Generating commands::types")?;
    let command_loaders = commands::loaders::generate(&ctx, &command_groups)
        .context("Generating commands::loaders")?;
    let command_wrappers = commands::wrappers::generate(&ctx, &command_groups)
        .context("Generating commands::wrappers")?;
    let toc = toc::generate(&ctx).context("Generating toc")?;

    // Render.
    let lib_rs = outputs::lib::TEMPLATE.replace("{{toc}}", &toc);
    let loader_rs = outputs::loader::TEMPLATE
        .replace("{{init::wrappers}}", &command_wrappers.init_wrappers)
        .replace(
            "{{instance::wrappers}}",
            &command_wrappers.instance_wrappers,
        )
        .replace("{{device::wrappers}}", &command_wrappers.device_wrappers)
        .replace(
            "{{init::struct_members}}",
            &command_loaders.init_struct_members,
        )
        .replace(
            "{{instance::struct_members}}",
            &command_loaders.instance_struct_members,
        )
        .replace(
            "{{device::struct_members}}",
            &command_loaders.device_struct_members,
        )
        .replace("{{init::loaders}}", &command_loaders.init_loaders)
        .replace("{{instance::loaders}}", &command_loaders.instance_loaders)
        .replace("{{device::loaders}}", &command_loaders.device_loaders);
    let vk_rs = outputs::header::TEMPLATE
        .replace("{{vk::command_types}}", &command_types)
        .replace("{{vk::api_constants}}", &api_constants)
        .replace("{{vk::base_types}}", &base_types)
        .replace("{{vk::function_pointers}}", &function_pointers)
        .replace("{{vk::handles}}", &handles)
        .replace("{{vk::enumerations}}", &enumerations)
        .replace("{{vk::bitmasks}}", &bitmasks)
        .replace("{{vk::structures}}", &structures)
        .replace("{{vk::unions}}", &unions);
    let tests_rs = outputs::tests::TEMPLATE.to_string();

    // Formatting.
    let lib_rs = rustfmt(&lib_rs).context("Failed to format 'lib.rs' with rustfmt")?;
    let loader_rs = rustfmt(&loader_rs).context("Failed to format 'loader.rs' with rustfmt")?;
    let vk_rs = rustfmt(&vk_rs).context("Failed to format 'vk.rs' with rustfmt")?;
    let tests_rs = rustfmt(&tests_rs).context("Failed to format 'tests.rs' with rustfmt")?;

    // Write.
    std::fs::write(vulk_lib_dir.join("lib.rs"), lib_rs)?;
    std::fs::write(vulk_lib_dir.join("loader.rs"), loader_rs)?;
    std::fs::write(vulk_lib_dir.join("vk.rs"), vk_rs)?;
    std::fs::write(vulk_lib_dir.join("tests.rs"), tests_rs)?;

    Ok(())
}

//
// Utilities
//

fn rustfmt(input: &str) -> Result<String> {
    // Temp.
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path().join("vk.rs");
    let temp_path = temp_path.as_os_str().to_string_lossy();
    let temp_path = temp_path.as_ref();

    // Write.
    std::fs::write(temp_path, input)?;

    // Format.
    std::process::Command::new("rustfmt")
        .args(["--config", "max_width=200", temp_path])
        .output()?;

    // Read.
    let output = std::fs::read_to_string(temp_path)?;

    Ok(output)
}

pub struct ProvidedByMap(HashMap<String, String>);

impl ProvidedByMap {
    pub fn new(registry: &Registry) -> Self {
        let mut map = HashMap::new();
        for feature in &registry.features {
            for require in &feature.requires {
                for require_entry in &require.entries {
                    match require_entry {
                        registry::RequireEntry::Type { name, .. }
                        | registry::RequireEntry::Enum { name, .. }
                        | registry::RequireEntry::Command { name } => {
                            assert!(!map.contains_key(name));
                            map.insert(name.to_string(), feature.name.clone());
                        }
                    }
                }
            }
        }
        for extension in &registry.extensions {
            for require in &extension.requires {
                for require_entry in &require.entries {
                    match require_entry {
                        registry::RequireEntry::Type { name, .. }
                        | registry::RequireEntry::Enum { name, .. }
                        | registry::RequireEntry::Command { name } => {
                            if !map.contains_key(name) {
                                map.insert(name.to_string(), extension.name.clone());
                            }
                        }
                    }
                }
            }
        }
        Self(map)
    }

    pub fn get(&self, ident: impl AsRef<str>) -> &str {
        self.0
            .get(ident.as_ref())
            .unwrap_or_else(|| panic!("Getting {} from provided by map", ident.as_ref()))
    }
}
