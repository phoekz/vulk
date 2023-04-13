use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct Manifest {
    pub commands: HashSet<String>,
    pub structures: HashSet<String>,
    pub extensions: HashSet<String>,
}
