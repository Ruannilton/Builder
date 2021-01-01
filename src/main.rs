use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    version: String,
    authors: Option<Vec<String>>,
    proj_type: String,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Platform {
    name: String,
    arch: Vec<String>,
    dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    project: Project,
    platform: Vec<Platform>,
}

fn main() {
    let proj_file = fs::read_to_string("teste.toml").expect("Failed to load project file");

    let proj: Config = toml::from_str(proj_file.as_str()).expect("Failed to parse project file");
    println!("{:?}", proj)
}
