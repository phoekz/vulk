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

pub mod loader;
pub mod vk;
"#;
