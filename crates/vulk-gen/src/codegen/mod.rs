use super::*;

mod api_constants;
mod attributes;
mod base_types;
mod bitmasks;
mod commands;
mod enumerations;
mod extensions;
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
    empty_flag_bits_map: &'a EmptyFlagBitsMap,
    vkspec: &'a docs::Vkspec,
    manifest: &'a manifest::Manifest,
}

pub fn generate(
    registry: &Registry,
    vkspec: &docs::Vkspec,
    manifest: &manifest::Manifest,
    vulk_lib_dir: &Path,
) -> Result<()> {
    // Generate.
    let c_type_map = registry::c_type_map();
    let provided_by_map = ProvidedByMap::new(registry);
    let empty_flag_bits_map = EmptyFlagBitsMap::new(registry);
    let command_groups = commands::analysis::group_by_loader(registry);
    let ctx = GeneratorContext {
        registry,
        c_type_map: &c_type_map,
        provided_by_map: &provided_by_map,
        empty_flag_bits_map: &empty_flag_bits_map,
        vkspec,
        manifest,
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
    let extensions = extensions::generate(&ctx).context("Generating extensions")?;

    // Render.
    let lib_rs = outputs::lib::TEMPLATE
        .replace("{{toc}}", &toc)
        .replace("{{extensions}}", &extensions);
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

    // Write.
    std::fs::write(vulk_lib_dir.join("lib.rs"), lib_rs)?;
    std::fs::write(vulk_lib_dir.join("loader.rs"), loader_rs)?;
    std::fs::write(vulk_lib_dir.join("vk.rs"), vk_rs)?;
    std::fs::write(vulk_lib_dir.join("tests.rs"), tests_rs)?;

    // Formatting.
    rustfmt(&vulk_lib_dir.join("lib.rs"), 80).context("Failed to format 'lib.rs' with rustfmt")?;
    rustfmt(&vulk_lib_dir.join("loader.rs"), 200)
        .context("Failed to format 'loader.rs' with rustfmt")?;
    rustfmt(&vulk_lib_dir.join("vk.rs"), 200).context("Failed to format 'vk.rs' with rustfmt")?;
    rustfmt(&vulk_lib_dir.join("tests.rs"), 200)
        .context("Failed to format 'tests.rs' with rustfmt")?;

    Ok(())
}

//
// Utilities
//

fn rustfmt(path: &Path, max_width: u32) -> Result<()> {
    let path = path.to_string_lossy().to_string();
    let output = std::process::Command::new("rustfmt")
        .args([
            "--edition",
            "2021",
            "--config",
            &format!("max_width={max_width}"),
            path.as_str(),
        ])
        .output()?;
    let code = output.status.code().unwrap();
    ensure!(code == 0, "{output:?}");
    Ok(())
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
                            map.insert(name.clone(), feature.name.clone());
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
                                map.insert(name.clone(), extension.name.clone());
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

struct EmptyFlagBitsMap(HashSet<String>);

impl EmptyFlagBitsMap {
    pub fn new(registry: &Registry) -> Self {
        let mut map = HashSet::new();

        for en in &registry.enums {
            let registry::EnumType::Bitmask = en.ty else {
                continue;
            };

            if en.members.is_empty() {
                // Vk*FlagBits has no members.
                map.insert(en.name.clone());
            }
        }

        for ty in &registry.types {
            let registry::TypeCategory::Bitmask {
                requires,
                bitvalues,
                ..
            } = &ty.category
            else {
                continue;
            };

            match (requires, bitvalues) {
                (None, None) => {
                    // Vk*Flags has no references => Flags must be empty.
                    map.insert(ty.name.clone());
                }
                (Some(requires), None) => {
                    if map.contains(requires) {
                        // The reference is empty => Flags must be empty.
                        map.insert(ty.name.clone());
                    }
                }
                (None, Some(bitvalues)) => {
                    if map.contains(bitvalues) {
                        // The reference is empty => Flags must be empty.
                        map.insert(ty.name.clone());
                    }
                }

                _ => {}
            }
        }

        Self(map)
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.0.contains(name.as_ref())
    }
}
