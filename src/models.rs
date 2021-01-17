use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuilderOp {
    pub editor_dir: String,
    pub projects_dir: String,
    pub project_type: String,
    pub arch: Vec<String>,
    pub author: Option<Vec<String>>,
    pub create_git: bool,
    pub configure_on_create: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub proj_type: String,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub name: String,
    pub arch: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub project: ProjectInfo,
    pub platform: Vec<Platform>,
}

impl Project {
    pub fn save(&self) {
        let mut op =
            utils::get_project_path(&self.project.name, Some(self.project.version.clone()));
        let content = toml::to_string_pretty(self).expect("Failed to parse Project");
        if op.is_dir() {
            op.push("conf.toml");
            fs::write(op, content).expect("Failed to save Project");
        } else {
            println!("Project folder doesn´t exist");
        }
    }
    pub fn load(name: String, version: Option<String>) -> Option<Project> {
        let mut op = utils::get_project_path(&name, version);
        if op.is_dir() {
            op.push("conf.toml");
            let content = fs::read_to_string(op).expect("failed to load project config");
            let proj = toml::from_str(&content).expect("failed to parse project config");
            Some(proj)
        } else {
            println!("Project folder doesn´t exist");
            None
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = toml::to_string_pretty(self);
        match res {
            Err(_) => write!(f, "Failed to pretty parse"),
            Ok(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = toml::to_string_pretty(self);
        match res {
            Err(_) => write!(f, "Failed to pretty parse"),
            Ok(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for ProjectInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = toml::to_string_pretty(self);
        match res {
            Err(_) => write!(f, "Failed to pretty parse"),
            Ok(v) => write!(f, "{}", v),
        }
    }
}
