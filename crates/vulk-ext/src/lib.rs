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
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

//
// Imports
//

use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::mem::{size_of, size_of_val, zeroed, MaybeUninit};
use std::ptr::{addr_of, addr_of_mut, null, null_mut};

use anyhow::{bail, ensure, Context, Result};
use log::{debug, error, log};
use vulk::vk;

//
// Modules
//

pub mod vkx;

//
// Utilities
//

fn aligned_size(size: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    assert!(alignment.is_power_of_two());
    (size + alignment - 1) & !(alignment - 1)
}
