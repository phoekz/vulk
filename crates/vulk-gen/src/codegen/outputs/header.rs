pub const TEMPLATE: &str = r#"
//
// Imports
//

use std::ffi::{c_char, c_void};
use bitflags::bitflags;

//
// Defines
//

#[must_use]
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

//
// API Constants
//

{{vk::api_constants}}

//
// Base types
//

{{vk::base_types}}

//
// Function pointers
//

{{vk::function_pointers}}

//
// Handles
//

{{vk::handles}}

//
// Enumerations
//

{{vk::enumerations}}

//
// Bitmasks
//

{{vk::bitmasks}}

//
// Structures
//

{{vk::structures}}

//
// Unions
//

{{vk::unions}}

//
// Command types
//

{{vk::command_types}}
"#;
