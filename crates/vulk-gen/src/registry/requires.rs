use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Require {
    pub entries: Vec<RequireEntry>,
}

#[derive(serde::Serialize, Debug)]
pub enum RequireEntry {
    Type {
        name: String,
        alias: Option<String>,
    },
    Enum {
        name: String,
        offset: Option<String>,
        bitpos: Option<String>,
        extends: Option<String>,
        extnumber: Option<String>,
        value: Option<String>,
        dir: Option<String>,
    },
    Command {
        name: String,
    },
}

pub(super) fn parse_require<'a>(
    nodes: impl Iterator<Item = xml::Node<'a>>,
) -> Result<Vec<Require>> {
    let mut output = vec![];

    for node in nodes {
        let mut entries = vec![];
        for entry in node.children_any_elements() {
            match entry.tag() {
                "type" => {
                    entries.push(RequireEntry::Type {
                        name: entry.required_attribute("name"),
                        alias: entry.attribute("alias"),
                    });
                }

                "enum" => {
                    entries.push(RequireEntry::Enum {
                        name: entry.required_attribute("name"),
                        offset: entry.attribute("offset"),
                        bitpos: entry.attribute("bitpos"),
                        extends: entry.attribute("extends"),
                        extnumber: entry.attribute("extnumber"),
                        value: entry.attribute("value"),
                        dir: entry.attribute("dir"),
                    });
                }

                "command" => {
                    entries.push(RequireEntry::Command {
                        name: entry.required_attribute("name"),
                    });
                }

                "comment" => {
                    // Ignored.
                }

                tag => {
                    warn!("unknown tag={tag}");
                }
            }
        }
        output.push(Require { entries });
    }

    Ok(output)
}
