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
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

//
// Modules
//

pub mod loader;
pub mod vk;
#[cfg(test)]
mod tests;

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

pub unsafe fn read_to_vec<F, T>(f: F, s_type: Option<vk::StructureType>) -> Result<Vec<T>, Error>
where
    F: Fn(*mut u32, *mut T) -> Result<(), Error>,
{
    use std::alloc::alloc_zeroed;
    use std::alloc::Layout;
    use std::mem::size_of;
    use std::mem::size_of_val;
    use std::ptr::addr_of;

    // Query the number of elements.
    let mut len_u32 = 0_u32;
    f(&mut len_u32, std::ptr::null_mut())?;
    let len = len_u32 as usize;

    // Allocate.
    let layout = Layout::from_size_align(len * size_of::<T>(), 16).unwrap();
    let ptr = alloc_zeroed(layout);

    // Note: if the output type contains `s_type`, we have to write the
    // structure type on each of the output elements.
    if let Some(s_type) = s_type {
        for i in 0..len {
            let dst = ptr.add(i * size_of::<T>());
            dst.copy_from(addr_of!(s_type).cast(), size_of_val(&s_type));
        }
    }

    // Query elements.
    f(&mut len_u32, ptr.cast())?;

    // Build the Vec.
    let vec = Vec::from_raw_parts(ptr.cast::<T>(), len, len);

    Ok(vec)
}
"#;
