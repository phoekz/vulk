use super::*;

mod api_constants;
mod base_types;
mod bitmasks;
mod commands;
mod enumerations;
mod function_pointers;
mod handles;
mod header_file;
mod lib_file;
mod loader_file;
mod structures;
mod toc;
mod translation;
mod unions;

use std::fmt::Write;
use translation::Translator;

pub fn generate(
    registry: &Registry,
    description_map: &DescriptionMap,
    vulk_lib_dir: &Path,
) -> Result<()> {
    // Generate.
    let translator = translation::Translator::new(registry);
    let api_constants = api_constants::generate(registry, &translator, description_map)?;
    let base_types = base_types::generate(registry, &translator, description_map)?;
    let function_pointers = function_pointers::generate(registry, &translator, description_map)?;
    let handles = handles::generate(registry, &translator, description_map)?;
    let enumerations = enumerations::generate(registry, &translator, description_map)?;
    let bitmasks = bitmasks::generate(registry, &translator, description_map)?;
    let structures = structures::generate(registry, &translator, description_map)?;
    let unions = unions::generate(registry, &translator, description_map)?;
    let command_types = commands::types::generate(registry, &translator, description_map)?;
    let command_groups = commands::analysis::group_by_loader(registry);
    let command_loaders =
        commands::loaders::generate(registry, &translator, description_map, &command_groups)?;
    let command_wrappers =
        commands::wrappers::generate(registry, &translator, description_map, &command_groups)?;
    let toc = toc::generate(registry, &translator, description_map)?;

    // Render.
    let lib_rs = lib_file::TEMPLATE.replace("{{toc}}", &toc);
    let loader_rs = loader_file::TEMPLATE
        .replace("{{loader::wrappers}}", &command_wrappers.loader_wrappers)
        .replace(
            "{{instance::wrappers}}",
            &command_wrappers.instance_wrappers,
        )
        .replace("{{device::wrappers}}", &command_wrappers.device_wrappers)
        .replace(
            "{{loader::struct_members}}",
            &command_loaders.loader_struct_members,
        )
        .replace(
            "{{instance::struct_members}}",
            &command_loaders.instance_struct_members,
        )
        .replace(
            "{{device::struct_members}}",
            &command_loaders.device_struct_members,
        )
        .replace("{{loader::loaders}}", &command_loaders.loader_loaders)
        .replace("{{instance::loaders}}", &command_loaders.instance_loaders)
        .replace("{{device::loaders}}", &command_loaders.device_loaders);
    let vk_rs = header_file::TEMPLATE
        .replace("{{vk::command_types}}", &command_types)
        .replace("{{vk::api_constants}}", &api_constants)
        .replace("{{vk::base_types}}", &base_types)
        .replace("{{vk::function_pointers}}", &function_pointers)
        .replace("{{vk::handles}}", &handles)
        .replace("{{vk::enumerations}}", &enumerations)
        .replace("{{vk::bitmasks}}", &bitmasks)
        .replace("{{vk::structures}}", &structures)
        .replace("{{vk::unions}}", &unions);

    // Formatting.
    let lib_rs = rustfmt(&lib_rs).context("Failed to format 'lib.rs' with rustfmt")?;
    let loader_rs = rustfmt(&loader_rs).context("Failed to format 'loader.rs' with rustfmt")?;
    let vk_rs = rustfmt(&vk_rs).context("Failed to format 'vk.rs' with rustfmt")?;

    // Write.
    std::fs::write(vulk_lib_dir.join("lib.rs"), lib_rs)?;
    std::fs::write(vulk_lib_dir.join("loader.rs"), loader_rs)?;
    std::fs::write(vulk_lib_dir.join("vk.rs"), vk_rs)?;

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
