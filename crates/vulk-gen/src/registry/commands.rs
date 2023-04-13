use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Command {
    pub name: String,
    pub return_type: String,
    pub params: Vec<CommandParam>,
    pub successcodes: Vec<String>,
    pub errorcodes: Vec<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct CommandParam {
    pub name: String,
    pub ty: String,
    pub optional: Option<String>,
    pub text: Option<String>,
    pub len: Option<String>,
}

pub(super) fn parse_commands<'a>(
    nodes: impl Iterator<Item = xml::Node<'a>>,
) -> Result<Vec<Command>> {
    let mut output = vec![];

    for node in nodes {
        let Some(proto) = node.child("proto") else {
            continue;
        };
        let name = proto.required_child_text("name");
        if let Some(api) = node.attribute("api") {
            if api == "vulkansc" {
                debug!("Ignoring command: name={name}, api=vulkansc");
                continue;
            }
        }
        output.push(Command {
            name,
            return_type: proto.required_child_text("type"),
            params: node
                .children_elements("param")
                .map(|node| {
                    let name = node.required_child_text("name");
                    let ty = node.required_child_text("type");
                    let optional = node.attribute("optional");
                    let text = node.joined_children_text();
                    let len = node.attribute("len");
                    CommandParam {
                        name,
                        ty,
                        optional,
                        text,
                        len,
                    }
                })
                .collect(),
            successcodes: if let Some(successcodes) = node.attribute("successcodes") {
                successcodes
                    .split(',')
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            } else {
                vec![]
            },
            errorcodes: if let Some(errorcodes) = node.attribute("errorcodes") {
                errorcodes
                    .split(',')
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            } else {
                vec![]
            },
        });
    }

    Ok(output)
}
