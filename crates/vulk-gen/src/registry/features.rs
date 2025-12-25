use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Feature {
    pub api: String,
    pub requires: Vec<Require>,
    pub name: String,
}

pub(super) fn parse_features<'a>(
    nodes: impl Iterator<Item = xml::Node<'a>>,
) -> Result<Vec<Feature>> {
    let mut output = vec![];

    for node in nodes {
        let api = node.required_attribute("api");
        if api.split(',').any(|s| s == "vulkan") {
            output.push(Feature {
                api,
                requires: parse_require(node.children_elements("require"))?,
                name: node.required_attribute("name"),
            });
        } else {
            debug!("Ignoring feature: api={api}");
        }
    }

    Ok(output)
}
