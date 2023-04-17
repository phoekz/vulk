//
// Lints
//

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

//
// Imports
//

use std::{
    borrow::Cow,
    ffi::CStr,
    mem::{size_of, zeroed, MaybeUninit},
    ptr::{addr_of, addr_of_mut, null, null_mut},
    time::Instant,
};

use anyhow::{ensure, Context, Result};
use gpu::Gpu;
use log::{info, log, warn};
use vulk::vk;

//
// Modules
//

mod command;
mod demos;
mod gpu;
mod resource;
mod shader;

//
// Main
//

fn main() -> Result<()> {
    // Logging.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .try_init()?;

    // Vulkan.
    unsafe {
        let gpu = Gpu::create().context("Creating Gpu")?;
        run_demo::<demos::ComputeDemo>(&gpu)?;
        gpu.destroy();
    };

    Ok(())
}

//
// Demo
//

pub trait DemoCallbacks {
    const NAME: &'static str;
    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized;
    unsafe fn execute(gpu: &Gpu, state: &Self) -> Result<()>;
    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()>;
}

pub unsafe fn run_demo<Demo>(gpu: &Gpu) -> Result<()>
where
    Demo: DemoCallbacks,
{
    let time = Instant::now();
    let name = Demo::NAME;
    info!("Running demo::{name}");
    let state = Demo::create(gpu).with_context(|| format!("Creating demo::{name}"))?;
    Demo::execute(gpu, &state).with_context(|| format!("Executing demo::{name}"))?;
    Demo::destroy(gpu, state).with_context(|| format!("Destroying demo::{name}"))?;
    info!("demo::{name} took {} seconds", time.elapsed().as_secs_f64());
    Ok(())
}
