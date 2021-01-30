use crate::models::{BuilderOp, Dependencie, Platform, Project};
use crate::ProjectLog;
use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::convert::From;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::string::ToString;

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
    op
}

pub fn get_project_path(name: &String, version: Option<String>) -> PathBuf {
    let cfg = load_builder_config();
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
                    let content = fs::read_to_string(p_dir).expect("failed to load project config");
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

pub fn get_project_conf(name: &String, version: Option<String>) -> Project {
    let mut proj_conf = get_project_path(name, version);
    proj_conf.push("conf.toml");
    let str_conf = std::fs::read_to_string(proj_conf).expect("Failed to read project conf");
    toml::from_str(str_conf.as_ref()).expect("Failed to parse project config file")
}

pub fn init_git(path: PathBuf) {
    let program = which::which("git").unwrap();
    match Command::new(program.to_owned())
        .current_dir(path)
        .arg("init")
        .output()
    {
        Ok(_) => {
            println!("Git repository created");
        }
        Err(e) => {
            println!("Failed to create git repository\n{}", e);
        }
    }
}

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_owned()),
        Err(_) => None,
    }
}

pub fn promp(message: &str, break_line: bool, default: Option<impl ToString>) -> Option<String> {
    if break_line {
        println!("{}", message);
        let inp = match read_line() {
            Some(val) => Some(val),
            None => match default {
                Some(val) => Some(val.to_string()),
                None => None,
            },
        };
        inp
    } else {
        print!("{}", message);
        std::io::stdout().flush().unwrap();
        let inp = match read_line() {
            Some(val) => Some(val),
            None => match default {
                Some(val) => Some(val.to_string()),
                None => None,
            },
        };
        inp
    }
}

pub fn promp_vec(
    message: &str,
    break_line: bool,
    delimitier: char,
    default: Option<Vec<impl ToString>>,
) -> Option<Vec<String>> {
    if break_line {
        println!("{}", message);
        let inp = match read_line() {
            Some(val) => {
                if val.len() > 0 {
                    let mut vec = Vec::new();
                    let spl = val.split(delimitier);
                    for v in spl {
                        vec.push(v.trim().to_owned())
                    }
                    Some(vec)
                } else {
                    None
                }
            }
            None => match default {
                Some(val) => Some(val.iter().map(|x| x.to_string()).collect::<Vec<String>>()),
                None => None,
            },
        };
        inp
    } else {
        print!("{}", message);
        std::io::stdout().flush().unwrap();
        let inp = match read_line() {
            Some(val) => {
                if val.len() > 0 {
                    let mut vec = Vec::new();
                    let spl = val.split(delimitier);
                    for v in spl {
                        vec.push(v.trim().to_owned())
                    }
                    Some(vec)
                } else {
                    None
                }
            }
            None => match default {
                Some(val) => Some(val.iter().map(|x| x.to_string()).collect::<Vec<String>>()),
                None => None,
            },
        };
        inp
    }
}

pub fn parse_project_conf(
    project: String,
    version: Option<String>,
) -> HashMap<String, HashMap<String, Vec<Dependencie>>> {
    let proj = get_project_conf(&project, version);
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

pub fn find_files(start_path: PathBuf, regex: &str) -> Vec<String> {
    let mut dir_queue = VecDeque::<PathBuf>::new();
    let mut found = LinkedList::<String>::new();
    let reg = Regex::new(regex).expect(format!("failed to compile regex: {}", regex).as_str());
    dir_queue.push_back(start_path.to_owned());

    while dir_queue.len() > 0 {
        let dir = dir_queue.pop_front().unwrap();
        for f in fs::read_dir(&dir)
            .expect(format!("failed to read directory {:?}", dir.to_owned()).as_str())
        {
            if let Ok(file) = f {
                let path: PathBuf = file.path();

                if path.is_dir() {
                    dir_queue.push_back(path);
                } else {
                    let short = path.strip_prefix(start_path.to_owned()).unwrap();
                    let short_str = short.to_str().unwrap();
                    if reg.is_match(short_str) {
                        found.push_back(short_str.to_owned());
                    }
                }
            }
        }
    }

    found.iter().map(|x| x.clone()).collect()
}
