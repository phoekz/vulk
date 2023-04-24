use super::*;

pub mod analysis;
pub mod loaders;
pub mod types;
pub mod wrappers;

fn command_only_targets_windows(ctx: &GeneratorContext<'_>, vk_ident: &str) -> Option<String> {
    let provided_by = ctx.provided_by_map.get(vk_ident);
    let platform = ctx.extension_platform_map.get(provided_by);
    if let Some(platform) = platform {
        if platform == "win32" {
            let attr = attributes::Builder::new()
                .cfg_target_family("windows")
                .build();
            Some(attr)
        } else {
            None
        }
    } else {
        None
    }
}
