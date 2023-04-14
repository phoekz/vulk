use super::*;

pub fn generate(
    registry: &Registry,
    _c_type_map: &CtypeMap,
    description_map: &DescriptionMap,
) -> Result<String> {
    let mut str = String::new();

    let mut chapter_order = HashMap::new();
    for command in &registry.commands {
        let desc = description_map.get(&command.name).context("Missing desc")?;
        chapter_order.insert(desc.title.as_str(), desc.title_order);
    }
    let mut chapter_order = chapter_order.into_iter().collect::<Vec<_>>();
    chapter_order.sort_by_key(|&(_, index)| index);

    let mut grouped_commands: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for command in &registry.commands {
        let desc = description_map.get(&command.name).context("Missing desc")?;
        if let Some(vec) = grouped_commands.get_mut(desc.title.as_str()) {
            vec.push((desc.line_index, command.name.as_str()));
        } else {
            grouped_commands.insert(
                desc.title.as_str(),
                vec![(desc.line_index, command.name.as_str())],
            );
        }
    }
    for vec in grouped_commands.values_mut() {
        vec.sort_unstable();
    }

    for (chapter_name, _) in &chapter_order {
        writeln!(str, "//! ## {chapter_name}")?;
        let commands = grouped_commands.get(chapter_name).unwrap();
        for (_, command) in commands {
            let desc = &description_map.get(*command).context("Missing desc")?;
            let command = translation::vk_simple_function(command)?;
            writeln!(str, "//! - [`vk::{command}`] {}", desc.desc)?;
        }
    }

    Ok(str)
}
