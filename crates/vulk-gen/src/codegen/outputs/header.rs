pub const TEMPLATE: &str = r#"
//
// Imports
//

use std::ffi::{c_char, c_void};

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

//
// Internal
//

fn display_flag_bits_u32<FlagBit>(f: &mut std::fmt::Formatter<'_>, flags: u32, flag_bits: &[FlagBit]) -> std::fmt::Result
    where FlagBit: std::fmt::Debug + Into<u32> + Copy
{
    let mut count = 0;
    let mut remaining = flags;
    for &flag_bit in flag_bits {
        if flags & flag_bit.into() > 0 {
            if count > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{flag_bit:?}")?;
            remaining &= !flag_bit.into();
            count += 1;
        }
    }
    if remaining != 0 {
        if count > 0 {
            write!(f, " | ")?;
        }
        write!(f, "0b{remaining:b}")?;
    }
    Ok(())
}

fn display_flag_bits_u64<FlagBit>(f: &mut std::fmt::Formatter<'_>, flags: u64, flag_bits: &[FlagBit]) -> std::fmt::Result
    where FlagBit: std::fmt::Debug + Into<u64> + Copy
{
    let mut count = 0;
    let mut remaining = flags;
    for &flag_bit in flag_bits {
        if flags & flag_bit.into() > 0 {
            if count > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{flag_bit:?}")?;
            remaining &= !flag_bit.into();
            count += 1;
        }
    }
    if remaining != 0 {
        if count > 0 {
            write!(f, " | ")?;
        }
        write!(f, "0b{remaining:b}")?;
    }
    Ok(())
}
"#;
