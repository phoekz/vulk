//
// Lints
//

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::needless_range_loop,
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

//
// Imports
//

use std::{
    mem::{size_of, transmute, zeroed, MaybeUninit},
    path::{Path, PathBuf},
    ptr::null,
    time::{Duration, Instant, SystemTime},
};

use anyhow::{ensure, Context, Result};
use log::info;
use vkx::prelude::*;
use vulk::vk;
use vulk_ext::vkx;

//
// Modules
//

mod demos;

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
        run_demo::<demos::ClearDemo>(&gpu)?;
        run_demo::<demos::TriangleDemo>(&gpu)?;
        run_demo::<demos::CubeDemo>(&gpu)?;
        run_demo::<demos::GuiDemo>(&gpu)?;
        run_demo::<demos::RaytracingDemo>(&gpu)?;
        #[cfg(target_family = "windows")]
        run_demo::<demos::WindowDemo>(&gpu)?;
        gpu.destroy();
    };

    info!("Done");

    Ok(())
}

//
// Gpu
//

pub struct Gpu {
    pub instance: vkx::Instance,
    pub physical_device: vkx::PhysicalDevice,
    pub device: vkx::Device,
}

impl Gpu {
    pub unsafe fn create() -> Result<Self> {
        let instance = vkx::Instance::create(&vkx::InstanceCreateInfo {
            validation_layers: true,
            ..Default::default()
        })
        .context("Creating instance")?;
        let physical_device =
            vkx::PhysicalDevice::create(&instance).context("Creating physical device")?;
        let device =
            vkx::Device::create(&instance, &physical_device, None).context("Creating device")?;

        Ok(Self {
            instance,
            physical_device,
            device,
        })
    }

    pub unsafe fn destroy(self) {
        self.device.destroy();
        self.instance.destroy();
    }
}

//
// Demo
//

trait DemoCallbacks {
    const NAME: &'static str;
    unsafe fn create(gpu: &Gpu) -> Result<Self>
    where
        Self: Sized;
    unsafe fn execute(gpu: &Gpu, state: &mut Self) -> Result<()>;
    unsafe fn destroy(gpu: &Gpu, state: Self) -> Result<()>;
}

unsafe fn run_demo<Demo>(gpu: &Gpu) -> Result<()>
where
    Demo: DemoCallbacks,
{
    let time = Instant::now();
    let name = Demo::NAME;
    info!("Running demo::{name}");
    let mut state = Demo::create(gpu).with_context(|| format!("Creating demo::{name}"))?;
    Demo::execute(gpu, &mut state).with_context(|| format!("Executing demo::{name}"))?;
    Demo::destroy(gpu, state).with_context(|| format!("Destroying demo::{name}"))?;
    info!("demo::{name} took {} seconds", time.elapsed().as_secs_f64());
    Ok(())
}

//
// Gpu resources
//

trait GpuResource {
    type CreateInfo<'a>;
    unsafe fn create(gpu: &Gpu, create_info: &Self::CreateInfo<'_>) -> Result<Self>
    where
        Self: Sized;
    unsafe fn destroy(self, gpu: &Gpu);
}

//
// Utilities
//

fn work_dir_or_create() -> Result<PathBuf> {
    let work_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("work");
    if !work_dir.exists() {
        std::fs::create_dir_all(&work_dir)?;
    }
    Ok(work_dir)
}
