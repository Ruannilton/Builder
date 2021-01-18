use crate::models::*;
use crate::utils;
use chrono::Utc;
use std::fs;
use std::process::Command;

pub fn cmd_create_project(op: &BuilderOp, args: NewArgs) {
    let mut proj_dir = utils::get_project_path(&args.name.to_owned(), None);
    if proj_dir.is_dir() {
        println!("Project folder already exists");
        println!("If you pretends create a new version of this project check the nv command");
    } else {
        match fs::create_dir(&proj_dir) {
            Ok(_) => {
                proj_dir.push("v1_0");
                match fs::create_dir(&proj_dir) {
                    Ok(_) => {
                        let proj_info: ProjectInfo;
                        let proj_plat: Platform;
                        if op.configure_on_create && args.conf == false {
                            let desc = utils::promp("Project Description: ", false, Some(""));
                            proj_info = ProjectInfo {
                                name: args.name.to_owned().clone(),
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
                            let ptype = utils::promp(
                                "Project Type [\"app\" or \"lib\"]: ",
                                false,
                                Some("app"),
                            )
                            .unwrap();
                            let desc = utils::promp("Project Description: ", false, Some(""));

                            let authors =
                                utils::promp_vec("Project authors: ", false, ';', Some(vec![""]));
                            proj_info = ProjectInfo {
                                name: args.name.to_owned().clone(),
                                version: "1.0".to_owned(),
                                authors: authors,
                                proj_type: ptype,
                                desc: desc,
                            };
                            let plat = utils::promp(
                                "Project Platform [ex: \"windos or linux or macos\"]: ",
                                false,
                                Some("all"),
                            )
                            .unwrap();
                            let arch = utils::promp_vec(
                                "Project Archtecture [ex: \"x86;x64 or x86 or x64\"]: ",
                                false,
                                ';',
                                Some(vec!["all"]),
                            )
                            .unwrap();
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
                        let log = ProjectLog {
                            name: proj.project.name,
                            last_opened: proj.project.version,
                            last_time: Utc::now().time(),
                        };
                        log.save();
                    }
                    Err(e) => {
                        println!("Failed to creat project");
                        println!("{}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to creat project");
                println!("{}", e);
            }
        }
    }
}

pub fn cmd_open_project(op: &BuilderOp, args: OpenArgs) {
    let mut log = ProjectLog::load(args.name.to_owned()).unwrap();

    let v = match args.version {
        Some(r) => Some(r.to_owned()),
        None => Some(log.last_opened),
    };
    let path = utils::get_project_path(&args.name.to_owned(), v.clone());

    match &op.editor_cmd {
        Some(val) => {
            let program = which::which(val.clone()).unwrap();
            match Command::new(program.to_owned()).arg(path).output() {
                Ok(_) => {
                    println!("try save");
                    match v {
                        Some(ver) => {
                            println!("saved");
                            log.last_opened = ver;
                            log.last_time = Utc::now().time();
                            log.save();
                        }
                        None => {}
                    };
                }
                Err(e) => {
                    println!("Failed to execute command\n{}", e);
                }
            }
        }
        None => {
            println!("An text editor command was not provided");
        }
    };
}
