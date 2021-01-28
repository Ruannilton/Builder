use crate::models::*;
use crate::utils;
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn cmd_create_project(op: &BuilderOp, args: NewArgs) {
    let mut proj_dir = utils::get_project_path(&args.name.to_owned(), None);
    if proj_dir.is_dir() {
        println!("Project folder already exists");
        println!("If you pretends create a new version of this project check the nv command");
    } else {
        match fs::create_dir(&proj_dir) {
            Ok(_) => {
                let mut proj_ver = proj_dir.to_owned();
                proj_ver.push("v1_0");
                match fs::create_dir(&proj_ver) {
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
                            last_opened: proj.project.version.clone(),
                            last_version: proj.project.version.clone(),
                            last_time: Utc::now().time(),
                        };
                        log.save();
                        if op.create_git {
                            utils::init_git(proj_dir.to_owned());
                            proj_dir.push(".gitignore");
                            match fs::write(proj_dir, "log.json") {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Failed to create .gitignore\n{}", e);
                                }
                            }
                        }
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

fn cmd_compile_project(
    platform: &String,
    arch: &String,
    path: &PathBuf,
    release: bool,
    deps: &Vec<Dependencie>,
) {
    println!("");
    if release {
        println!("Mode: Debug");
    } else {
        println!("Mode: Debug");
    }
    println!("Building project for [{}][{}]\n", platform, arch);

    for dep in deps.iter() {
        println!("Adding dependencie: {} {}", dep.name, dep.version);
    }
    println!("");
}

pub fn cmd_build_project(op: &BuilderOp, args: BuildArgs) {
    let v = match args.version {
        Some(r) => Some(r.to_owned()),
        None => None,
    };
    let n = match args.name {
        Some(r) => r.to_owned(),
        None => {
            println!("No project found");
            std::process::exit(0);
        }
    };
    let proj_path = utils::get_project_path(&n, v.clone());
    let conf_tree = utils::parse_project_conf(n, v.clone());
    let mut build_path = proj_path.to_owned();
    build_path.push("build");
    if !build_path.is_dir() {
        fs::create_dir(&build_path).expect("failed to create build directory");
    }

    let path: PathBuf;
    {
        if args.release {
            let mut release_path = build_path.to_owned();
            release_path.push("release");
            if !release_path.is_dir() {
                fs::create_dir(&release_path).expect("failed to create debug directory");
            }
            path = release_path;
        } else {
            let mut debug_path = build_path.to_owned();
            debug_path.push("debug");
            if !debug_path.is_dir() {
                fs::create_dir(&debug_path).expect("failed to create debug directory");
            }
            path = debug_path;
        }
    }

    let arg_platforms = args.platform.as_ref();
    let arg_archtectures = args.archtecture.as_ref();

    let platforms = match arg_platforms {
        Some(val) => val.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        None => op.plats.clone(),
    };
    let archtectures = match arg_archtectures {
        Some(val) => val.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        None => op.arch.clone(),
    };

    let mut universal_deps = Vec::<Dependencie>::new();
    if let Some(all_plat) = conf_tree.get(&String::from("all")) {
        if let Some(all_deps) = all_plat.get(&String::from("all")) {
            universal_deps = all_deps.clone();
        }
    }

    for plat_a in platforms {
        if let Some(plat) = conf_tree.get(&String::from(&plat_a)) {
            for arch_a in &archtectures {
                if let Some(deps) = plat.get(&String::from(arch_a)) {
                    let mut deps_vec = Vec::<Dependencie>::new();
                    deps_vec.extend(universal_deps.clone());
                    if let Some(u_deps) = plat.get(&String::from("all")) {
                        deps_vec.extend(u_deps.clone());
                    }
                    if let Some(all_plat) = conf_tree.get(&String::from("all")) {
                        if let Some(all_deps) = all_plat.get(&String::from(arch_a)) {
                            deps_vec.extend(all_deps.clone());
                        }
                    }
                    deps_vec.extend(deps.clone());

                    cmd_compile_project(&plat_a, arch_a, &path, args.release, &deps_vec);
                }
            }
        }
    }
}

pub fn cmd_build_lib(op: &BuilderOp, args: BuildArgs) {}
