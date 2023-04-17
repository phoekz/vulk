use super::*;

#[derive(serde::Serialize, Debug)]
pub enum Component {
    R,
    G,
    B,
    A,
    S,
    D,
}

#[derive(serde::Serialize, Debug)]
pub enum Plane {
    P0,
    P1,
    P2,
}

#[derive(serde::Serialize, Debug)]
pub struct Format {
    pub name: String,
    pub block_size: u32,
    pub components: Vec<Component>,
    pub planes: Vec<Plane>,
}

pub(super) fn parse_formats<'a>(nodes: impl Iterator<Item = xml::Node<'a>>) -> Result<Vec<Format>> {
    let mut output = vec![];

    for node in nodes {
        let name = node.required_attribute("name");
        let block_size = node.required_attribute("blockSize");
        let block_size: u32 = block_size.parse()?;

        let mut components = vec![];
        for component in node.children_elements("component") {
            let component_name = component.required_attribute("name");
            let component = match component_name.as_str() {
                "R" => Component::R,
                "G" => Component::G,
                "B" => Component::B,
                "A" => Component::A,
                "S" => Component::S,
                "D" => Component::D,
                component => {
                    bail!("Unknown component={component}");
                }
            };
            components.push(component);
        }

        let mut planes = vec![];
        for plane in node.children_elements("plane") {
            let index = plane.required_attribute("index");
            let index = match index.as_str() {
                "0" => Plane::P0,
                "1" => Plane::P1,
                "2" => Plane::P2,
                index => {
                    bail!("Unknown plane index={index}");
                }
            };
            planes.push(index);
        }

        output.push(Format {
            name,
            block_size,
            components,
            planes,
        });
    }

    Ok(output)
}
