use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Extension {
    pub name: String,
    pub number: String,
    pub ty: Option<String>,
    pub requires: Vec<Require>,
    pub promotedto: Option<String>,
}

pub(super) fn parse_extensions<'a>(
    nodes: impl Iterator<Item = xml::Node<'a>>,
) -> Result<Vec<Extension>> {
    let mut output = vec![];

    for node in nodes {
        let name = node.required_attribute("name");
        if node.required_attribute("supported") == "disabled" {
            debug!("Ignoring extension: name={name}, supported=disabled");
        }
        output.push(Extension {
            name,
            number: node.required_attribute("number"),
            ty: node.attribute("type"),
            requires: parse_require(node.children_elements("require"))?,
            promotedto: node.attribute("promotedto"),
        });
    }

    Ok(output)
}
