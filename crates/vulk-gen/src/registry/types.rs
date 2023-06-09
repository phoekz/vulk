use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Type {
    pub name: String,
    pub category: TypeCategory,
}

#[derive(serde::Serialize, Debug)]
pub enum TypeCategory {
    Basetype {
        ty: Option<String>,
    },
    Bitmask {
        ty: String,
        requires: Option<String>,
        bitvalues: Option<String>,
    },
    BitmaskAlias {
        alias: String,
    },
    Handle {
        parent: Option<String>,
        objtypeenum: String,
        ty: String,
    },
    HandleAlias {
        alias: String,
    },
    Enum {},
    EnumAlias {
        alias: String,
    },
    Funcpointer {
        text: String,
        types: Vec<String>,
    },
    Struct {
        alias: Option<String>,
        structextends: Vec<String>,
        members: Vec<TypeMember>,
    },
    Union {
        alias: Option<String>,
        members: Vec<TypeMember>,
    },
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct TypeMember {
    pub name: String,
    pub ty: String,
    pub optional: Option<String>,
    pub comment: Option<String>,
    pub text: Option<String>,
    pub en: Option<String>,
}

pub(super) fn parse_types<'a>(nodes: impl Iterator<Item = xml::Node<'a>>) -> Result<Vec<Type>> {
    let mut output = vec![];

    for node in nodes {
        // Ignore "types" without category. These are typically system headers
        // such as "windows.h".
        let Some(category) = node.attribute("category") else {
            continue;
        };

        // Parse based on category.
        match category.as_str() {
            "basetype" => {
                if let Some(name) = node.child_text("name") {
                    output.push(Type {
                        name,
                        category: TypeCategory::Basetype {
                            ty: node.child_text("type"),
                        },
                    });
                }
            }
            "bitmask" => {
                let name = if let Some(name) = node.attribute("name") {
                    name
                } else if let Some(name) = node.child_text("name") {
                    name
                } else {
                    bail!("name is required");
                };

                let api = node.attribute("api");
                if let Some(api) = api {
                    if api == "vulkansc" {
                        debug!("Ignoring bitmask: name={name}, api=vulkansc");
                        continue;
                    }
                }

                if let Some(alias) = node.attribute("alias") {
                    output.push(Type {
                        name,
                        category: TypeCategory::BitmaskAlias { alias },
                    });
                } else {
                    output.push(Type {
                        name,
                        category: TypeCategory::Bitmask {
                            ty: node.required_child_text("type"),
                            requires: node.attribute("requires"),
                            bitvalues: node.attribute("bitvalues"),
                        },
                    });
                }
            }
            "handle" => {
                if let Some(alias) = node.attribute("alias") {
                    output.push(Type {
                        name: node.required_attribute("name"),
                        category: TypeCategory::HandleAlias { alias },
                    });
                } else {
                    output.push(Type {
                        name: node.required_child_text("name"),
                        category: TypeCategory::Handle {
                            parent: node.attribute("parent"),
                            objtypeenum: node.required_attribute("objtypeenum"),
                            ty: node.required_child_text("type"),
                        },
                    });
                }
            }
            "enum" => {
                if let Some(alias) = node.attribute("alias") {
                    output.push(Type {
                        name: node.required_attribute("name"),
                        category: TypeCategory::EnumAlias { alias },
                    });
                } else {
                    output.push(Type {
                        name: node.required_attribute("name"),
                        category: TypeCategory::Enum {},
                    });
                }
            }
            "funcpointer" => {
                output.push(Type {
                    name: node.required_child_text("name"),
                    category: TypeCategory::Funcpointer {
                        text: node.joined_children_text().unwrap(),
                        types: node.children_text("type").collect(),
                    },
                });
            }
            "struct" => {
                let name = node.required_attribute("name");
                output.push(Type {
                    name: name.clone(),
                    category: TypeCategory::Struct {
                        alias: node.attribute("alias"),
                        structextends: if let Some(structextends) = node.attribute("structextends")
                        {
                            structextends.split(',').map(ToString::to_string).collect()
                        } else {
                            vec![]
                        },
                        members: node
                            .children_elements("member")
                            .filter_map(|node| {
                                let member = TypeMember {
                                    name: node.required_child_text("name"),
                                    ty: node.required_child_text("type"),
                                    optional: node.attribute("optional"),
                                    comment: node.child_text("comment"),
                                    text: node.joined_children_text(),
                                    en: node.child_text("enum"),
                                };

                                let api = node.attribute("api");
                                if let Some(api) = api {
                                    if api == "vulkansc" {
                                        debug!(
                                            "Ignoring struct member: parent={} name={}, api=vulkansc",
                                            name, member.name
                                        );
                                        return None;
                                    }
                                }

                                Some(member)
                            })
                            .collect(),
                    },
                });
            }
            "union" => {
                ensure!(
                    node.attribute("structextends").is_none(),
                    "Unions can't have structextends"
                );

                output.push(Type {
                    name: node.required_attribute("name"),
                    category: TypeCategory::Union {
                        alias: node.attribute("alias"),
                        members: node
                            .children_elements("member")
                            .map(|node| {
                                let name = node.required_child_text("name");
                                let ty = node.required_child_text("type");
                                let optional = node.attribute("optional");
                                let comment = node.child_text("comment");
                                let text = node.joined_children_text();
                                let en = node.child_text("enum");
                                TypeMember {
                                    name,
                                    ty,
                                    optional,
                                    comment,
                                    text,
                                    en,
                                }
                            })
                            .collect(),
                    },
                });
            }
            "include" | "define" => {
                // Ignored categories.
            }
            _ => {
                warn!("Unknown type category={category}");
            }
        }
    }

    Ok(output)
}

pub type TypeNameIndexMap<'a> = HashMap<&'a str, usize>;

pub(super) fn type_index_map<'a, I>(types: I) -> TypeNameIndexMap<'a>
where
    I: IntoIterator<Item = &'a Type>,
{
    let mut map = TypeNameIndexMap::new();
    for (index, ty) in types.into_iter().enumerate() {
        assert!(
            !map.contains_key(ty.name.as_str()),
            "map already contains {}",
            ty.name
        );
        map.insert(ty.name.as_str(), index);
    }
    map
}

pub type FlagBitsMap<'a> = HashMap<&'a str, &'a str>;

pub(super) fn flag_bits_map<'a, I>(types: I) -> FlagBitsMap<'a>
where
    I: IntoIterator<Item = &'a Type>,
{
    let mut map = FlagBitsMap::new();
    for ty in types {
        let types::TypeCategory::Bitmask { requires, .. } = &ty.category else {
                continue;
            };

        let Some(requires) = requires else {
                continue;
            };

        map.insert(requires.as_str(), ty.name.as_str());
    }
    map
}
