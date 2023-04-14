use super::*;

pub use c_types::*;
pub use commands::*;
pub use enums::*;
pub use extensions::*;
pub use features::*;
pub use requires::*;
pub use types::*;

mod c_types;
mod commands;
mod enums;
mod extensions;
mod features;
mod requires;
mod types;
mod xml;

#[derive(serde::Serialize, Debug)]
pub struct Registry {
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
    pub commands: Vec<Command>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
}

impl Registry {
    pub fn parse(vk_xml: &str) -> Result<Self> {
        let xml_doc = roxmltree::Document::parse(vk_xml)?;
        let xml_root = xml::Node::from(xml_doc.root());
        let xml_registry = xml_root.child("registry").context("Missing registry")?;
        let types = parse_types(xml_registry.children_under_parent("types", "type"))?;
        let enums = parse_enums(xml_registry.children("enums"))?;
        let commands = parse_commands(xml_registry.children_under_parent("commands", "command"))?;
        let features = parse_features(xml_registry.children("feature"))?;
        let extensions =
            parse_extensions(xml_registry.children_under_parent("extensions", "extension"))?;
        Ok(Self {
            types,
            enums,
            commands,
            features,
            extensions,
        })
    }

    pub fn extended(self, manifest: &Manifest) -> Result<Self> {
        let mut registry = self;
        let enum_name_index_map = enum_name_index_map(&registry.enums);
        for feature in &registry.features {
            let extend_count = extend_enums_with_requires(
                &enum_name_index_map,
                &feature.requires,
                None,
                &mut registry.enums,
            )
            .with_context(|| format!("Extending feature name={}", feature.name))?;
            info!(
                "Feature name={} extended enums {} times",
                feature.name, extend_count
            );
        }
        for extension in registry
            .extensions
            .iter()
            .filter(|ext| manifest.extensions.contains(&ext.name))
        {
            let extend_count = extend_enums_with_requires(
                &enum_name_index_map,
                &extension.requires,
                Some(extension.number.as_str()),
                &mut registry.enums,
            )
            .with_context(|| format!("Extending extension name={}", extension.name))?;
            info!(
                "Extension name={} extended enums {} times",
                extension.name, extend_count
            );
        }
        Ok(registry)
    }

    pub fn filtered(self, manifest: &Manifest) -> Result<Self> {
        let commands = self
            .commands
            .into_iter()
            .filter(|cmd| manifest.commands.contains(cmd.name.as_str()))
            .collect::<Vec<_>>();
        let types =
            filter_types(self.types, &commands, &manifest.structures).context("Filtering types")?;
        let enums = filter_enums(self.enums, &types);
        Ok(Self {
            types,
            enums,
            commands,
            features: vec![],
            extensions: vec![],
        })
    }
}

fn extend_enums_with_requires(
    enum_name_index_map: &EnumNameIndexMap,
    requires: &[Require],
    external_extension_number: Option<&str>,
    enums: &mut [Enum],
) -> Result<usize> {
    const BASE_VALUE: i32 = 1_000_000_000_i32;
    const RANGE_SIZE: i32 = 1000_i32;

    let mut extend_count = 0;
    for entry in requires.iter().flat_map(|require| &require.entries) {
        let RequireEntry::Enum { name, offset, bitpos, extends, extnumber, value, dir } = entry else {
            continue;
        };

        let Some(extends) = extends else {
            continue;
        };

        let dir = match dir {
            Some(dir) => {
                assert_eq!(dir, "-");
                -1
            }
            None => 1,
        };
        let enum_index = *enum_name_index_map.get(extends.as_str()).unwrap();
        let members = &mut enums[enum_index].members;
        if let (Some(extnumber), Some(offset)) = (extnumber, offset) {
            let extension_number: i32 = extnumber.parse()?;
            assert!(extension_number > 0);
            let offset: i32 = offset.parse()?;
            let enum_offset = BASE_VALUE + (extension_number - 1) * RANGE_SIZE + offset;
            let enum_offset = dir * enum_offset;
            debug!("Extended {extends}::{name} = {enum_offset}");
            members.push(EnumMember {
                name: name.clone(),
                ty: None,
                value: Some(format!("{enum_offset}")),
                bitpos: None,
                comment: None,
                alias: None,
            });
            extend_count += 1;
        } else if let (Some(extnumber), Some(offset)) = (external_extension_number, offset) {
            let extension_number: i32 = extnumber.parse()?;
            assert!(extension_number > 0);
            let offset: i32 = offset.parse()?;
            let enum_offset = BASE_VALUE + (extension_number - 1) * RANGE_SIZE + offset;
            let enum_offset = dir * enum_offset;
            debug!("Extended {extends}::{name} = {enum_offset}");
            members.push(EnumMember {
                name: name.clone(),
                ty: None,
                value: Some(format!("{enum_offset}")),
                bitpos: None,
                comment: None,
                alias: None,
            });
            extend_count += 1;
        } else if let Some(bitpos) = bitpos {
            assert_eq!(dir, 1);
            debug!("Extended {extends}::{name} = {bitpos}");
            members.push(EnumMember {
                name: name.clone(),
                ty: None,
                value: None,
                bitpos: Some(bitpos.clone()),
                comment: None,
                alias: None,
            });
            extend_count += 1;
        } else if let Some(value) = value {
            assert_eq!(dir, 1);
            debug!("Extended {extends}::{name} = {value}");
            members.push(EnumMember {
                name: name.clone(),
                ty: None,
                value: Some(value.clone()),
                bitpos: None,
                comment: None,
                alias: None,
            });
            extend_count += 1;
        }
    }

    Ok(extend_count)
}

fn filter_types<'a>(
    types: Vec<Type>,
    commands: &[Command],
    structures: impl IntoIterator<Item = &'a String>,
) -> Result<Vec<Type>> {
    // C types.
    let c_type_map = c_type_map();

    // Type index map.
    let type_index_map = type_index_map(&types);

    // Prepare stack.
    let mut stack = vec![];
    for command in commands {
        stack.push(command.return_type.as_str());
        for param in &command.params {
            stack.push(&param.ty);
        }
    }
    for structure in structures {
        stack.push(structure.as_str());
    }

    // Visit.
    let mut visited_types: HashSet<&str> = HashSet::new();
    let mut type_indices: HashSet<usize> = HashSet::new();
    while let Some(curr_type) = stack.pop() {
        // Already visited.
        if visited_types.contains(curr_type) {
            continue;
        }

        // Ignore C types.
        if c_type_map.contains_key(curr_type) {
            continue;
        }

        // Get type.
        let type_index = *type_index_map
            .get(curr_type)
            .with_context(|| format!("Missing type={curr_type}"))?;
        let ty = &types[type_index];

        // Visit type members, if applicable.
        #[allow(clippy::match_same_arms)]
        match &ty.category {
            TypeCategory::Basetype { ty, .. } => {
                if let Some(ty) = ty {
                    stack.push(ty.as_str());
                }
            }
            TypeCategory::Funcpointer { types, .. } => {
                for ty in types {
                    stack.push(ty.as_str());
                }
            }
            TypeCategory::Bitmask {
                requires,
                bitvalues,
                ..
            } => {
                if let Some(requires) = requires {
                    stack.push(requires.as_str());
                }
                if let Some(bitvalues) = bitvalues {
                    stack.push(bitvalues.as_str());
                }
            }
            TypeCategory::Struct { members, .. } => {
                for member in members {
                    stack.push(member.ty.as_str());
                }
            }
            TypeCategory::Union { members, .. } => {
                for member in members {
                    stack.push(member.ty.as_str());
                }
            }
            _ => {}
        }

        // Update state.
        type_indices.insert(type_index);
        visited_types.insert(curr_type);
    }

    Ok(types
        .into_iter()
        .enumerate()
        .filter_map(|(type_index, ty)| {
            if type_indices.contains(&type_index) {
                return Some(ty);
            }
            None
        })
        .collect())
}

fn filter_enums(enums: Vec<Enum>, types: &[Type]) -> Vec<Enum> {
    let mut enum_names = HashSet::new();
    for ty in types {
        match &ty.category {
            TypeCategory::Enum {} => {
                enum_names.insert(ty.name.as_str());
            }
            TypeCategory::Bitmask { requires, .. } => {
                if let Some(requires) = requires {
                    enum_names.insert(requires.as_str());
                }
            }
            _ => {}
        }
    }

    enums
        .into_iter()
        .filter_map(|r#enum| {
            if r#enum.name == "API Constants" {
                return Some(r#enum);
            }
            if enum_names.contains(r#enum.name.as_str()) {
                return Some(r#enum);
            }
            None
        })
        .collect()
}
