use crate::models::BuilderOp;
use std::fs;
use std::path::PathBuf;

pub fn get_builder_path() -> PathBuf {
    let mut builder_dir = dirs::home_dir().expect("Failed to get home dir");
    builder_dir.push(".builder");
    builder_dir
}

pub fn load_builder_config() -> BuilderOp {
    let mut conf_path = get_builder_path();
    conf_path.push("config.json");
    let conf_content = fs::read_to_string(conf_path).expect("Failed to read config file");
    let op: BuilderOp = serde_json::from_str(&conf_content).expect("Failed to parse config file");
    println!("Config loaded");
    op
}

pub fn get_project_path(name: &String) -> PathBuf {
    let cfg = load_builder_config();
    let mut proj_dir = PathBuf::from(cfg.projects_dir.clone());
    proj_dir.push(name.clone());

    proj_dir
}
