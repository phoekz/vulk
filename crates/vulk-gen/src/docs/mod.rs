use super::*;

#[derive(serde::Serialize, Debug)]
pub struct Vkspec {
    pub chapters: Vec<Chapter>,
    pub type_descriptions: HashMap<String, String>,
}

impl Vkspec {
    pub fn parse(docs_dir: &Path) -> Result<Self> {
        let doc_elements = {
            let mut out = vec![];
            let mut unique_headings = HashSet::new();
            parse_doc_into(
                &mut out,
                &mut unique_headings,
                docs_dir,
                &docs_dir.join("vkspec.adoc"),
            )?;
            out
        };

        let chapters = {
            let mut curr_heading = String::new();
            let mut curr_types = vec![];
            let mut chapters = vec![];
            for element in doc_elements {
                match element {
                    DocElement::Heading(heading) => {
                        if !curr_types.is_empty() {
                            chapters.push(Chapter {
                                heading: std::mem::take(&mut curr_heading),
                                types: std::mem::take(&mut curr_types),
                            });
                        }
                        curr_heading = heading;
                    }
                    DocElement::Open(open) => {
                        curr_types.push(Type {
                            name: open.refpage,
                            ty: open.ty,
                            desc: open.desc,
                        });
                    }
                }
            }
            if !curr_types.is_empty() {
                chapters.push(Chapter {
                    heading: std::mem::take(&mut curr_heading),
                    types: std::mem::take(&mut curr_types),
                });
            }
            chapters
        };

        let type_descriptions = {
            let mut map = HashMap::new();
            for chapter in &chapters {
                for ty in &chapter.types {
                    map.insert(ty.name.clone(), ty.desc.clone());
                }
            }
            map
        };

        let docs = Vkspec {
            chapters,
            type_descriptions,
        };

        Ok(docs)
    }

    pub fn type_desc(&self, name: &str) -> Option<&str> {
        let desc = self.type_descriptions.get(name)?;
        Some(desc.as_str())
    }
}

#[derive(serde::Serialize, Debug)]
pub struct Chapter {
    pub heading: String,
    pub types: Vec<Type>,
}

#[derive(serde::Serialize, Debug)]
pub struct Type {
    pub name: String,
    pub ty: String,
    pub desc: String,
}

#[derive(serde::Serialize, Debug)]
enum DocElement {
    Heading(String),
    Open(DocOpen),
}

#[derive(serde::Serialize, Debug)]
struct DocOpen {
    refpage: String,
    ty: String,
    desc: String,
}

fn parse_doc_into(
    out: &mut Vec<DocElement>,
    unique_headings: &mut HashSet<String>,
    docs_dir: &Path,
    path: &Path,
) -> Result<()> {
    let Ok(file) = std::fs::read_to_string(path) else {
        return Ok(());
    };

    for line in file.lines() {
        if let Some(line) = line.strip_prefix("= ") {
            if line.chars().next().unwrap().is_ascii_uppercase() {
                let heading = line.to_string();
                if !unique_headings.contains(&heading) {
                    unique_headings.insert(heading.clone());
                    out.push(DocElement::Heading(heading));
                    continue;
                }
            }
        }

        if line.starts_with("[open,refpage") {
            out.push(DocElement::Open(parse_doc_open(line)));
            continue;
        }

        let Some(line) = line.strip_prefix("include::{chapters}/") else {
            continue;
        };

        let Some(line) = line.strip_suffix("[]") else {
            continue;
        };

        parse_doc_into(
            out,
            unique_headings,
            docs_dir,
            &docs_dir.join("chapters").join(line),
        )?;
    }

    Ok(())
}

fn parse_doc_open(line: &str) -> DocOpen {
    let mut refpage: Option<String> = None;
    let mut ty: Option<String> = None;
    let mut desc: Option<String> = None;
    for part in line
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
    {
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
            "alias" | "xrefs" => {
                // Ignored.
            }
            _ => {
                warn!("unknown field={lhs}");
            }
        }
    }
    DocOpen {
        refpage: refpage.unwrap(),
        ty: ty.unwrap(),
        desc: desc.unwrap(),
    }
}

pub fn reference_url(ident: &str) -> String {
    format!("https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{ident}.html")
}
