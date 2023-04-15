#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::collapsible_match,
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::unnecessary_wraps,
    clippy::wildcard_imports
)]

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{bail, ensure, Context, Result};
use log::{debug, info, warn};
use manifest::Manifest;
use registry::Registry;

mod codegen;
mod docs;
mod manifest;
mod registry;

//
// Main
//

fn main() -> Result<()> {
    // Timing.
    let start_time = Instant::now();

    // Logging.
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()?;

    // Parse vkspec.adoc.
    let vkspec = docs::Vkspec::parse(&vulkan_docs_dir()).context("Parsing vkspec.adoc")?;
    std::fs::write(
        work_dir_or_create()?.join("docs.ron"),
        ron::ser::to_string_pretty(&vkspec, ron::ser::PrettyConfig::default())?,
    )?;

    // Load vk.xml.
    let vk_xml_file = vk_xml_file();
    ensure!(vk_xml_file.exists());
    ensure!(vk_xml_file.is_file());
    let vk_xml = std::fs::read_to_string(&vk_xml_file).context("Failed to read vk.xml")?;

    // Load manifest.ron.
    let manifest_file = manifest_file();
    ensure!(manifest_file.exists());
    ensure!(manifest_file.is_file());
    let manifest =
        std::fs::read_to_string(&manifest_file).context("Failed to read manifest.ron")?;
    let manifest: Manifest = ron::de::from_str(&manifest)?;

    // Parse registry.
    let registry = Registry::parse(&vk_xml).context("Parsing Vulkan registry")?;
    std::fs::write(
        work_dir_or_create()?.join("everything.ron"),
        ron::ser::to_string_pretty(&registry, ron::ser::PrettyConfig::default())?,
    )?;

    // Extend enum definitions with features and extensions.
    let registry = registry
        .extended(&manifest)
        .context("Extending enum definitions")?;
    std::fs::write(
        work_dir_or_create()?.join("extended.ron"),
        ron::ser::to_string_pretty(&registry, ron::ser::PrettyConfig::default())?,
    )?;

    // Filter registry.
    let registry = registry.filtered(&manifest).context("Filtering registry")?;
    std::fs::write(
        work_dir_or_create()?.join("filtered.ron"),
        ron::ser::to_string_pretty(&registry, ron::ser::PrettyConfig::default())?,
    )?;

    // Ordering registry.
    let registry = registry.ordered(&vkspec).context("Ordering registry")?;
    std::fs::write(
        work_dir_or_create()?.join("ordered.ron"),
        ron::ser::to_string_pretty(&registry, ron::ser::PrettyConfig::default())?,
    )?;

    // Codegen.
    let vulk_lib_dir = vulk_lib_dir();
    ensure!(vulk_lib_dir.exists());
    ensure!(vulk_lib_dir.is_dir());
    codegen::generate(&registry, &vkspec, &vulk_lib_dir).context("Code generating")?;

    // Execution time.
    info!(
        "Execution took {} seconds",
        start_time.elapsed().as_secs_f64()
    );

    Ok(())
}

//
// Utilities
//

fn vulkan_docs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("external")
        .join("Vulkan-Docs")
        .canonicalize()
        .unwrap()
}

fn vk_xml_file() -> PathBuf {
    vulkan_docs_dir().join("xml").join("vk.xml")
}

fn manifest_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("manifest.ron")
}

fn vulk_lib_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .unwrap()
        .join("vulk")
        .join("src")
}

fn work_dir_or_create() -> Result<PathBuf> {
    let work_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("work");
    if !work_dir.exists() {
        std::fs::create_dir_all(&work_dir)?;
    }
    Ok(work_dir)
}
