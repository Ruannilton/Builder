use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs};

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

pub fn load_project_config(path: &str) -> Result<Project, &str> {
    let proj_file = fs::read_to_string(path);

    match proj_file {
        Err(_) => Err("Failed to load file"),
        Ok(file) => {
            let proj: Result<Project, toml::de::Error> = toml::from_str(file.as_str());

            match proj {
                Err(_) => Err("Failed to parse project file"),
                Ok(t) => Ok(t),
            }
        }
    }
}
