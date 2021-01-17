use crate::models::BuilderOp;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
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
        None => {}
    };
    proj_dir
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
