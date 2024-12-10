use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Framework {
    Cargo,
}

#[derive(Debug, Deserialize)]
pub struct Cleanfile {
    pub frameworks: Option<Vec<Framework>>,

    #[serde(default = "default_recurse_depth")]
    pub recurse_depth: u8,

    #[serde(default)]
    pub docker_prune_all: bool,
}

fn default_recurse_depth() -> u8 {
    0
}
