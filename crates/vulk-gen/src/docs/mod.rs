use super::*;

use walkdir::WalkDir;

#[derive(serde::Serialize, Debug)]
pub struct Description {
    pub ty: String,
    pub desc: String,
    pub alias: Option<String>,
}

pub type DescriptionMap = HashMap<String, Description>;

pub fn parse_descriptions(chapters_dir: &Path) -> Result<DescriptionMap> {
    let mut descriptions = HashMap::new();
    for entry in WalkDir::new(chapters_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let file = std::fs::read_to_string(entry.path())?;
        for line in file.lines() {
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
                },
            );
        }
    }
    Ok(descriptions)
}

pub fn reference_url(ident: &str) -> String {
    format!("https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{ident}.html")
}
