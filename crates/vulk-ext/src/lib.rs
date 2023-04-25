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
use std::ffi::CString;
use std::mem::zeroed;
use std::ptr::{addr_of, addr_of_mut, null, null_mut};

use anyhow::{bail, ensure, Context, Result};
use log::{error, log};
use vulk::vk;

//
// Modules
//

pub mod vkx;
