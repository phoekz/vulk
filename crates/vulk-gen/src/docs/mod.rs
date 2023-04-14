use super::*;

use walkdir::WalkDir;

#[derive(serde::Serialize, Debug)]
pub struct Description {
    pub ty: String,
    pub desc: String,
    pub alias: Option<String>,
    pub title: String,
    pub title_order: usize,
    pub file: String,
    pub line_index: usize,
}

pub type DescriptionMap = HashMap<String, Description>;

pub fn parse_descriptions(docs_dir: &Path) -> Result<DescriptionMap> {
    let chapter_order_map = {
        // Todo: a more robust way to determine the ordering of commands is to
        // collapse all .adoc files into vkspec.adoc and parsing that instead.
        let vkspec = docs_dir.join("vkspec.adoc");
        let vkspec = std::fs::read_to_string(vkspec)?;
        let mut chapter_order_map = HashMap::new();
        for line in vkspec.lines() {
            if !line.starts_with("include::{chapters}/") {
                continue;
            }
            let last = line.split('/').last().unwrap();
            let file = last.trim_end_matches("[]");
            chapter_order_map.insert(file.to_string(), chapter_order_map.len());
        }
        chapter_order_map
    };

    let chapters_dir = docs_dir.join("chapters");
    let mut descriptions = HashMap::new();
    for entry in WalkDir::new(chapters_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let filename = entry.path().file_name().unwrap();
        let filename = filename.to_string_lossy();
        let file = std::fs::read_to_string(entry.path())?;
        let mut title: Option<String> = None;
        for (line_index, line) in file.lines().enumerate() {
            if title.is_none() && line.starts_with('=') {
                let line = line.trim().trim_matches('=').trim();
                if !line.is_empty() {
                    title = Some(line.to_owned());
                }
            }
            if !line.starts_with("[open") {
                continue;
            }
            let line = line.trim_start_matches('[');
            let line = line.trim_end_matches(']');

            let mut refpage: Option<String> = None;
            let mut ty: Option<String> = None;
            let mut desc: Option<String> = None;
            let mut alias: Option<String> = None;
            for part in line.split(',') {
                let Some((lhs,rhs)) = part.split_once('=') else {
                        continue;
                    };
                let lhs = lhs.trim();
                let rhs = rhs.trim_matches('\'').to_owned();
                match lhs {
                    "refpage" => {
                        refpage = Some(rhs);
                    }
                    "type" => {
                        ty = Some(rhs);
                    }
                    "desc" => {
                        desc = Some(rhs);
                    }
                    "alias" => {
                        alias = Some(rhs);
                    }
                    "xrefs" => {
                        // Ignored.
                    }
                    _ => {
                        warn!("unknown field={lhs}");
                    }
                }
            }

            let name = refpage.unwrap();
            ensure!(!descriptions.contains_key(&name));
            descriptions.insert(
                name,
                Description {
                    ty: ty.unwrap(),
                    desc: desc.unwrap(),
                    alias,
                    title: title.clone().unwrap_or("<unknown>".to_string()),
                    title_order: *chapter_order_map.get(&*filename).unwrap_or(&usize::MAX),
                    file: format!("{filename}"),
                    line_index,
                },
            );
        }
    }
    Ok(descriptions)
}

pub fn reference_url(ident: &str) -> String {
    format!("https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{ident}.html")
}
