use crate::utils;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuilderOp {
    pub editor_cmd: Option<String>,
    pub projects_dir: String,
    pub project_type: String,
    pub arch: Vec<String>,
    pub plats: Vec<String>,
    pub author: Option<Vec<String>>,
    pub create_git: bool,
    pub configure_on_create: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectLog {
    pub name: String,
    pub last_opened: String,
    pub last_version: String,
    pub last_time: NaiveTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependencie {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub name: String,
    pub arch: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub proj_type: String,
    pub desc: Option<String>,
    pub platform: Vec<Platform>,
}

impl ProjectLog {
    pub fn save(&self) {
        let mut op = Project::get_path_from(&self.name.to_owned(), None);
        let content = serde_json::to_string_pretty(self).expect("Failed to parse Project");
        if op.is_dir() {
            op.push("log.json");
            fs::write(op, content).expect("Failed to save Project");
        } else {
            println!("Project folder doesn´t exist");
        }
    }
    pub fn load<'a>(name: String) -> Option<ProjectLog> {
        let mut op = Project::get_path_from(&name, None);
        if op.is_dir() {
            op.push("log.json");
            let content = fs::read_to_string(op).expect("failed to load project config");
            let proj: ProjectLog =
                serde_json::from_str(&content).expect("failed to parse project config");
            Some(proj)
        } else {
            println!("Project folder doesn´t exist");
            None
        }
    }
}

impl Project {
    pub fn save(&self) {
        let mut op = Project::get_path_from(&self.name, Some(self.version.clone()));
        let content = toml::to_string_pretty(self).expect("Failed to parse Project");
        if op.is_dir() {
            op.push("conf.toml");
            fs::write(op, content).expect("Failed to save Project");
        } else {
            println!("Project folder doesn´t exist");
        }
    }
    pub fn load(name: &String, version: Option<String>) -> Option<Project> {
        let mut op = Project::get_path_from(name, version);
        if op.is_dir() {
            op.push("conf.toml");
            let content = fs::read_to_string(op).expect("failed to load project config");
            let proj: Project = toml::from_str(&content).expect("failed to parse project config");
            Some(proj)
        } else {
            println!("Project folder doesn´t exist");
            None
        }
    }
    pub fn get_path_from(name: &String, version: Option<String>) -> PathBuf {
        let cfg = utils::load_builder_config();
        let mut proj_dir = PathBuf::from(cfg.projects_dir.clone());
        proj_dir.push(name.clone());
        match version {
            Some(val) => {
                let mut v = val.clone();
                v.insert(0, 'v');
                v = v.replace(".", "_");
                proj_dir.push(v);
            }
            None => {
                let mut p_dir = proj_dir.to_owned();
                let log = {
                    if p_dir.is_dir() {
                        p_dir.push("log.json");
                        let content =
                            fs::read_to_string(p_dir).expect("failed to load project config");
                        let proj: ProjectLog =
                            serde_json::from_str(&content).expect("failed to parse project config");
                        Some(proj)
                    } else {
                        println!("Project folder doesn´t exist");
                        None
                    }
                };
                let mut v = log.expect("failed to get project log").last_version;
                v.insert(0, 'v');
                v = v.replace(".", "_");
                proj_dir.push(v);
            }
        };
        proj_dir
    }
    pub fn parse_conf_from(
        project: &String,
        version: Option<String>,
    ) -> HashMap<String, HashMap<String, Vec<Dependencie>>> {
        let proj = Project::load(&project, version).unwrap();
        let mut parsed = HashMap::<String, HashMap<String, Vec<Dependencie>>>::new();
        let cloj = |plat: &Platform| -> HashMap<String, Vec<Dependencie>> {
            let mut hm = HashMap::<String, Vec<Dependencie>>::new();
            //ITERATE ON ARCHS
            for arch in plat.arch.iter() {
                if hm.contains_key(arch) {
                    if let Some(deps) = hm.get_mut(arch) {
                        if let Some(dependencies) = plat.dependencies.as_ref() {
                            //ITERATE ON DEPS
                            for (dep, ver) in dependencies.iter() {
                                deps.push(Dependencie {
                                    name: dep.clone(),
                                    version: ver.clone(),
                                });
                            }
                        }
                    }
                } else {
                    let mut tmp_vec: Vec<Dependencie> = Vec::new();
                    if let Some(dependencies) = plat.dependencies.as_ref() {
                        //ITERATE ON DEPS
                        for (dep, ver) in dependencies.iter() {
                            tmp_vec.push(Dependencie {
                                name: dep.clone(),
                                version: ver.clone(),
                            });
                        }
                    }
                    hm.insert(arch.clone(), tmp_vec);
                }
            }
            hm
        };
        //ITERATE ON PLATAFORMS
        for plat in proj.platform.iter() {
            parsed
                .entry(plat.name.clone())
                .and_modify(|hm| {
                    //ITERATE ON ARCHS
                    for arch in plat.arch.iter() {
                        if hm.contains_key(arch) {
                            if let Some(deps) = hm.get_mut(arch) {
                                let dependencies = plat.dependencies.as_ref().unwrap();
                                //ITERATE ON DEPS
                                for (dep, ver) in dependencies.iter() {
                                    deps.push(Dependencie {
                                        name: dep.clone(),
                                        version: ver.clone(),
                                    });
                                }
                            }
                        } else {
                            let mut tmp_vec: Vec<Dependencie> = Vec::new();
                            let dependencies = plat.dependencies.as_ref().unwrap();
                            //ITERATE ON DEPS
                            for (dep, ver) in dependencies.iter() {
                                tmp_vec.push(Dependencie {
                                    name: dep.clone(),
                                    version: ver.clone(),
                                });
                            }
                            hm.insert(arch.clone(), tmp_vec);
                        }
                    }
                })
                .or_insert(cloj(plat));
        }
        parsed
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
