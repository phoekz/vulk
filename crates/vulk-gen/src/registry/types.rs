use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Type {
    pub name: String,
    pub category: TypeCategory,
}

#[derive(serde::Serialize, Debug)]
pub enum TypeCategory {
    Ctype {
        ty: String,
    },
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

#[derive(serde::Serialize, Debug)]
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

    // Built-in C types.
    {
        macro_rules! add {
            ($map:ident, $name:literal, $ty:literal) => {
                output.push(Type {
                    name: $name.to_owned(),
                    category: TypeCategory::Ctype { ty: $ty.to_owned() },
                });
            };
        }
        add!(output, "void", "c_void");
        add!(output, "char", "c_char");
        add!(output, "int", "c_int");
        add!(output, "size_t", "usize");

        add!(output, "float", "f32");
        add!(output, "double", "f64");

        add!(output, "int32_t", "i32");
        add!(output, "int64_t", "i64");

        add!(output, "uint8_t", "u8");
        add!(output, "uint16_t", "u16");
        add!(output, "uint32_t", "u32");
        add!(output, "uint64_t", "u64");
    }

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
                output.push(Type {
                    name: node.required_attribute("name"),
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
                            .map(|node| TypeMember {
                                name: node.required_child_text("name"),
                                ty: node.required_child_text("type"),
                                optional: node.attribute("optional"),
                                comment: node.child_text("comment"),
                                text: node.joined_children_text(),
                                en: node.child_text("enum"),
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

pub type TypeNameIndexMap = HashMap<String, usize>;

pub(super) fn type_name_index_map<'a, I>(types: I) -> TypeNameIndexMap
where
    I: IntoIterator<Item = &'a Type>,
{
    let mut map = TypeNameIndexMap::new();
    for (index, ty) in types.into_iter().enumerate() {
        assert!(
            !map.contains_key(&ty.name),
            "map already contains {}",
            ty.name
        );
        map.insert(ty.name.clone(), index);
    }
    map
}
