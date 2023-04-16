use super::*;

const TEMPLATE: &str = r#"{{vk_attr}}
pub struct {{rs_ident}}(u64);

impl {{rs_ident}} {
    #[must_use]
    pub const fn null() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for {{rs_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

impl std::fmt::Debug for {{rs_ident}} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("{{rs_ident}}").field(&format_args!("{self}")).finish()
    }
}
"#;

pub fn generate(ctx: &GeneratorContext<'_>) -> Result<String> {
    let mut str = String::new();

    for registry_type in &ctx.registry.types {
        let registry::TypeCategory::Handle { .. } = &registry_type.category else {
            continue;
        };

        let vk_ident = &registry_type.name;
        let vk_attr = attributes::Builder::new()
            .repr("transparent")
            .derive("Clone, Copy")
            .doc_chapter(ctx.vkspec.type_chapter(vk_ident))
            .doc_br()
            .doc_desc(ctx.vkspec.type_desc(vk_ident))
            .doc_br()
            .doc_provided(ctx.provided_by_map.get(vk_ident))
            .doc_br()
            .doc_ref(vk_ident)
            .build();
        let rs_ident = translation::vk_simple_type(vk_ident)?;
        writeln!(
            str,
            "{}",
            TEMPLATE
                .replace("{{vk_attr}}", &vk_attr)
                .replace("{{vk_ident}}", vk_ident)
                .replace("{{rs_ident}}", &rs_ident)
        )?;
    }

    Ok(str)
}
