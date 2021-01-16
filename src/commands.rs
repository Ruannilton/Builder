use crate::models::{BuilderOp, Platform, Project, ProjectInfo};
use crate::utils;
use std::fs;
use std::io::Write;

fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_owned()),
        Err(_) => None,
    }
}

pub fn cmd_create_project(op: &BuilderOp, name: String, conf: bool) {
    let proj_dir = utils::get_project_path(&name);
    if proj_dir.is_dir() {
        println!("Project folder already exists");
    } else {
        match fs::create_dir(proj_dir) {
            Ok(_) => {
                let proj_info: ProjectInfo;
                let proj_plat: Platform;

                if op.configure_on_create && conf == false {
                    print!("Project Description: ");
                    std::io::stdout().flush().unwrap();
                    let desc = read_line();
                    proj_info = ProjectInfo {
                        name: name.clone(),
                        version: "1.0".to_owned(),
                        authors: op.author.clone(),
                        proj_type: op.project_type.clone(),
                        desc: desc,
                    };
                    proj_plat = Platform {
                        name: "all".to_owned(),
                        arch: op.arch.clone(),
                        dependencies: None,
                    }
                } else {
                    print!("Project Type [\"app\" or \"lib\"]: ");
                    std::io::stdout().flush().unwrap();
                    let ptype = match read_line() {
                        Some(val) => val,
                        None => "app".to_owned(),
                    };

                    print!("Project Description: ");
                    std::io::stdout().flush().unwrap();
                    let desc = read_line();

                    proj_info = ProjectInfo {
                        name: name.clone(),
                        version: "1.0".to_owned(),
                        authors: None,
                        proj_type: ptype,
                        desc: desc,
                    };

                    print!("Project Platform [ex: \"windos or linux or macos\"]: ");
                    std::io::stdout().flush().unwrap();
                    let plat = match read_line() {
                        Some(val) => {
                            if val.len() > 0 {
                                val
                            } else {
                                String::from("all")
                            }
                        }
                        None => String::from("all"),
                    };

                    print!("Project Archtecture [ex: \"x86;x64 or x86 or x64\"]: ");
                    std::io::stdout().flush().unwrap();

                    let arch = match read_line() {
                        Some(val) => {
                            if val.len() > 0 {
                                let mut vec = Vec::new();
                                let spl = val.split(';');
                                for v in spl {
                                    vec.push(v.to_owned())
                                }
                                vec
                            } else {
                                vec![String::from("all")]
                            }
                        }
                        None => vec![String::from("all")],
                    };

                    proj_plat = Platform {
                        name: plat,
                        arch: arch,
                        dependencies: None,
                    }
                }

                let proj = Project {
                    project: proj_info,
                    platform: vec![proj_plat],
                };
                proj.save();
            }
            Err(_) => {
                println!("Failed to creat project");
            }
        }
    }
}

// pub fn cmd_open_project(op: &BuilderOp, name: String) {}
