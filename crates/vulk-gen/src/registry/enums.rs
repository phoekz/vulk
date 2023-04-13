use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Enum {
    pub name: String,
    pub ty: EnumType,
    pub members: Vec<EnumMember>,
}

#[derive(serde::Serialize, Debug)]
pub enum EnumType {
    Constants,
    Enum,
    Bitmask,
}

#[derive(serde::Serialize, Debug)]
pub struct EnumMember {
    pub name: String,
    pub ty: Option<String>,
    pub value: Option<String>,
    pub bitpos: Option<String>,
    pub comment: Option<String>,
    pub alias: Option<String>,
}

pub(super) fn parse_enums<'a>(nodes: impl Iterator<Item = xml::Node<'a>>) -> Result<Vec<Enum>> {
    let mut output = vec![];

    for node in nodes {
        let name = node.required_attribute("name");
        let ty = if let Some(ty) = node.attribute("type") {
            match ty.as_str() {
                "enum" => EnumType::Enum,
                "bitmask" => EnumType::Bitmask,
                ty => {
                    debug!("Ignoring enum: name={name}, type={ty}");
                    continue;
                }
            }
        } else {
            EnumType::Constants
        };
        output.push(Enum {
            name,
            ty,
            members: node
                .children_elements("enum")
                .map(|node| EnumMember {
                    name: node.required_attribute("name"),
                    ty: node.attribute("type"),
                    value: node.attribute("value"),
                    bitpos: node.attribute("bitpos"),
                    comment: node.attribute("comment"),
                    alias: node.attribute("alias"),
                })
                .collect(),
        });
    }

    Ok(output)
}

pub type EnumNameIndexMap = HashMap<String, usize>;

pub(super) fn enum_name_index_map<'a, I>(enums: I) -> EnumNameIndexMap
where
    I: IntoIterator<Item = &'a Enum>,
{
    let mut map = EnumNameIndexMap::new();
    for (index, en) in enums.into_iter().enumerate() {
        assert!(
            !map.contains_key(&en.name),
            "map already contains {}",
            en.name
        );
        map.insert(en.name.clone(), index);
    }
    map
}
