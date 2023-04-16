pub const TEMPLATE: &str = r#"//! # `vulk`
{{toc}}
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::too_many_arguments,
    clippy::unreadable_literal
)]

//
// Modules
//

pub mod loader;
pub mod vk;

//
// Re-exports
//

pub use loader::Init;
pub use loader::Instance;
pub use loader::Device;

//
// Error
//

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to load dynamic library {0}")]
    LibraryLoad(std::borrow::Cow<'static, str>),
    #[error("failed to load function {0}")]
    FunctionLoad(std::borrow::Cow<'static, str>),
    #[error("vulkan error {0:?}")]
    Vulkan(vk::Result),
}

//
// Utilities
//

pub unsafe fn read_to_vec<F, T>(f: F) -> Result<Vec<T>, Error>
where
    F: Fn(*mut u32, *mut T) -> Result<(), Error>,
{
    let mut len = 0_u32;
    f(&mut len, std::ptr::null_mut())?;
    let mut vec = Vec::with_capacity(len as _);
    f(&mut len, vec.as_mut_ptr())?;
    vec.set_len(len as _);
    Ok(vec)
}
"#;
