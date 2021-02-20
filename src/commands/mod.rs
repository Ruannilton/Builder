pub mod args;

use fs_extra::dir::CopyOptions;
use crate::models::data::*;
use crate::utils;
use args::*;
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;
use fs_extra;

pub fn cmd_create_project(args: NewArgs) {
    let op = &crate::BUILDER_DATA;

    let mut proj_dir = Project::make_path(&args.name.to_owned(), None);
    if proj_dir.is_dir() {
        println!("Project folder already exists");
        println!("If you pretends create a new version of this project check the nv command");
    } else {
        match fs::create_dir_all(&proj_dir) {
            Ok(_) => {
                let name: String = args.name.to_owned().clone();
                let version: String = "1.0.0".to_owned();
                let mut authors: Option<Vec<String>> = op.author.clone();
                let mut proj_type: String = match args.p_type{ Some(v)=>v.to_owned(),None=>op.project_type.clone()};
                let desc: Option<String>;
                let proj_plat: Platform;

                if op.configure_on_create && args.conf == false {
                    desc = utils::promp("Project Description: ", false, Some(""));

                    proj_plat = Platform {
                        name: "all".to_owned(),
                        arch: op.arch.clone(),
                        dependencies: None,
                    }
                } else {
                    proj_type =
                        utils::promp("Project Type [\"app\" or \"lib\"]: ", false, Some("app"))
                            .unwrap();
                    desc = utils::promp("Project Description: ", false, Some(""));

                    authors = utils::promp_vec("Project authors: ", false, ';', Some(vec![""]));
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
                let mut src_dir = proj_dir.to_owned();
                let mut head_dir = proj_dir.to_owned();
                let mut build_dir = proj_dir.to_owned();
                let mut dependencies = proj_dir.to_owned();
                let mut assets_dir = proj_dir.to_owned();
                
                src_dir.push("source");
                head_dir.push("header");
                build_dir.push("build");
                dependencies.push("dependencies");
                assets_dir.push("assets");

                fs::create_dir(&src_dir).or_else(|_:std::io::Error|->Result<(),()>{println!("Failed to create source dir");Ok(())}).unwrap();
                fs::create_dir(&head_dir).or_else(|_:std::io::Error|->Result<(),()>{println!("Failed to create header dir");Ok(())}).unwrap();
                fs::create_dir(build_dir).or_else(|_:std::io::Error|->Result<(),()>{println!("Failed to create build dir");Ok(())}).unwrap();
                fs::create_dir(dependencies).or_else(|_:std::io::Error|->Result<(),()>{println!("Failed to create dependencies dir");Ok(())}).unwrap();
                fs::create_dir(assets_dir).or_else(|_:std::io::Error|->Result<(),()>{println!("Failed to create assets dir");Ok(())}).unwrap();

                src_dir.push("main.c");
                head_dir.push(format!{"{}.h",name.clone()});
                let mut header_guard = name.clone().to_uppercase();
                header_guard.push_str("_H");
                fs::write(src_dir, format!{"#include \"{}.h\"\n\nint main(){{\nreturn 0;\n}}\n",name.clone(),});
                fs::write(head_dir, format!{"#ifndef {}\n#define {}\n#include <stdio.h>\n#endif",header_guard,header_guard});
                {
                    Project {
                        name: name.clone(),
                        version: version.clone(),
                        authors: authors,
                        proj_type: proj_type,
                        desc: desc,
                        platform: vec![proj_plat],
                    }
                    .save();
                }

                ProjectLog {
                    name: name,
                    last_opened: version,
                    last_version: "1.0.0".to_owned(),
                    last_time: Utc::now().time(),
                }
                .save();

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
}

pub fn cmd_open_project(args: OpenArgs) {
    let mut log = match ProjectLog::load(args.name.to_owned()){
        Some(val)=> val,
        None =>{
            println!("Could not find project log");
            return;
        }
    };
    let v: Option<&str> = match args.version {
        Some(r) => Some(r),
        None => Some(log.last_opened.as_ref()),
    };
    let o_proj = Project::load(&args.name.to_owned(), v.clone());

    match o_proj {
        Some(proj) => {
            let path = proj.get_path();
            let op = &crate::BUILDER_DATA;
            match &op.editor_cmd {
                Some(val) => {
                    let program = match which::which(val.clone()){
                        Ok(val)=>val,
                        Err(e)=>{
                            println!("{:?}",e);
                            println!("Could not find editor configured");
                            return;
                        }
                    };
                    match Command::new(program).arg(path).output() {
                        Ok(_) => {
                            match v {
                                Some(ver) => {
                                    log.last_opened = String::from(ver);
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
        None => {
            println!("Project not found");
        }
    }
}

pub fn cmd_list(args: ListArgs) {
    match args.ptype {
        "proj" => {
            println!("Projects:\n");
            let projects = Project::list();
            for p in projects {
                println!("  {}: {}", p.name, p.version);
            }
        }
        "lib" => {
            println!("Libraries:\n");
            let libs = Library::list();
            for l in libs {
                println!("  {}: {}", l.name, l.version);
            }
        }
        _ => {
            println!("Libraries:\n");
            let libs = Library::list();
            for l in libs {
                println!("  {}: {}", l.name, l.version);
            }
            println!("Projects:\n");
            let projects = Project::list();
            for p in projects {
                println!("  {}: {}", p.name, p.version);
            }
        }
    }
}

pub fn cmd_build_project(args: BuildArgs) {
    let op :&BuilderOp = &crate::BUILDER_DATA;
    
    match args.name{
        Some(name)=>{
           match Project::load(&String::from(name),args.version){
               Some(proj)=>{
                let proj: Project = proj;
                let mut build_dir = proj.get_path();
                build_dir.push("build");

                if args.release{
                    build_dir.push("release");
                }else{
                    build_dir.push("debug");
                }

                fs::create_dir_all(&build_dir);

                let platforms: Vec<&str> = match args.platform{
                    Some(val) => val,
                    None => op.plats.iter().map(|x|x.as_ref()).collect::<Vec<&str>>()
                };

                let archs: Vec<&str> = match args.archtecture{
                    Some(val)=>val,
                    None => op.arch.iter().map(|x| x.as_ref()).collect::<Vec<&str>>()
                };
                
                for plat in platforms.iter(){
                    for arch in archs.iter(){
                        compile_project(plat,arch,&build_dir,&proj,args.verbose);
                    }
                }
               },
               None=>{}
           }
        },
        None=>{}
    };
}

pub fn cmd_show_project(arg: ShowArgs){
 
    let namae = arg.name.to_owned();
    let proj = Project::load(&namae, arg.version);

    match proj{
        Some(project)=>{
            let project: Project = project;
            println!("{}",format!("Type: {}",project.proj_type));

            let mut all_v: String = String::new();
            for version in project.get_versions(){
                all_v.push_str(format!("{}.{}.{}",version.major,version.minor,version.patch).as_str());
                all_v.push_str(", ");
            }
            let all_v = &all_v[0..all_v.len()-2];
            if let Some(v) = arg.version{
                println!("{}",format!("Version:\n   Current: {}\n   All: {}",v,all_v));
            }else{
                println!("{}",format!("Version:\n   Current: {}\n   All: {}",project.version,all_v));
            }
            if let Some(a) = project.authors{
                let a: Vec<String> = a;
                let mut st: String = String::new();
                for name in a{
                    st.push_str(name.as_str());
                    st.push_str(", ");
                }
                let st = &st[0..st.len()-2];
                println!("{}",format!("Authors: {}",st));
            }
            if let Some(d) = project.desc{
                println!("{}",format!("Description: {}",d));
            }
            for plat in project.platform{
                println!("");
                if let Some(dep_h) = plat.dependencies{
                   
                    let mut st: String = String::new();
                    for name in plat.arch{
                        st.push_str(name.as_str());
                        st.push_str(", ");
                    }
                    let st = &st[0..st.len()-2];
                    println!("{}",format!("{} {}",plat.name,st));
                    for dep in dep_h{
                        println!("{}",format!("   {} {}",dep.0,dep.1)); 
                     }
                }
                
            }
        },
        None=>{
            println!("Project not found!");
        }
    }
}

pub fn cmd_nv_project(arg: NvArgs){
    match Project::load(&arg.name.to_owned(), arg.from){
        Some(proj)=>{
            let proj: Project = proj;
            let from = match arg.from{
                Some(f)=>semver::Version::parse(f).unwrap(),
                None=>semver::Version::parse(proj.version.as_str()).unwrap()
            };
            let mut to = from.clone();

            match arg.u_type{
                "major"=>{
                    match arg.to{
                        Some(val)=>{
                            to.major = val.parse::<u64>().unwrap();
                        },
                        None => {to.increment_major();}
                    };
                },
                "minor"=>{
                    match arg.to{
                        Some(val)=>{
                            to.minor = val.parse::<u64>().unwrap();
                        },
                        None => {to.increment_minor();}
                    };
                },
                "patch"=>{
                    match arg.to{
                        Some(val)=>{
                            to.patch = val.parse::<u64>().unwrap();
                        },
                        None => {to.increment_patch();}
                    };
                }
                _=>{
                    println!("Should especify if the update is major, minor or patch");
                }
            }
            let to = format!("{}.{}.{}",to.major,to.minor,to.patch);
            let new_path = Project::make_path(&arg.name.to_owned(), Some(to.as_str()));
            
            match fs::create_dir(&new_path){
               Ok(_)=>{
                let mut cp_dir = proj.get_path();
                let mut cp_op = CopyOptions::new();
                cp_op.content_only = true;
                 match fs_extra::dir::copy(cp_dir, new_path, &cp_op){
                   Ok(_)=>{
                    let mut proj = Project::load(&arg.name.to_owned(), Some(to.as_str()));
                    if let Some(mut p)= proj{
                        p.version = to;
                        p.save();
                    }
                   },
                   Err(_)=>{println!("Failed to create new project content")}
               }},
               Err(_)=>{println!("Failed to create new project folder");}
            };
        },
        None=>{
            println!("Project not found");
        }
    }
}

fn generate_objects(gcc: &PathBuf, path: &PathBuf, output: &PathBuf) -> Vec<String> {
    let sources = utils::find_files(path.to_owned(), "/*.c");
    let mut obj_dirs = Vec::new();

    let res = Command::new(gcc.to_owned())
        .current_dir(path)
        .arg("-c")
        .arg("-fpic")
        .arg("-Iheader")
        .args(sources.clone()).output()
        .expect("Failed to compile");
    
        std::io::stdout().write_all(&res.stdout).unwrap();
        std::io::stderr().write_all(&res.stderr).unwrap();

    for file in fs::read_dir(path).unwrap() {
        if let Ok(f) = file {
            let pf = f.path();
            let path_file: &str = pf.to_str().unwrap();

            if !pf.is_dir() {
                if path_file.ends_with(".o") {
                    let tmp: Vec<&str> = path_file.split('\\').collect();
                    let file_name: &str = tmp.last().unwrap();
                    let mut file_to = output.to_owned();
                    file_to.push(file_name);
                    obj_dirs.push(file_to.to_owned());
                    fs::rename(path_file, file_to).expect("Failed to copy file");
                }
            }
        }
    }
    obj_dirs
        .iter()
        .map(|x| x.to_str().unwrap().to_owned())
        .collect()
}

fn generate_executable(gcc: &PathBuf, path: &PathBuf, out: &PathBuf,sources:Vec<String>) {
    
    println!("Generating executable");
    let _res = Command::new(gcc.to_owned())
        .current_dir(path)
        .arg("-Iheader")
        .arg("-o")
        .arg(out.to_str().unwrap())
        .args(sources)
        .status()
        .expect("Failed to compile");
}

fn generate_library(gcc: &PathBuf, path: &PathBuf, out: &PathBuf, sources:Vec<String>) {

    println!("Generating lib");
    let _res = Command::new(gcc.to_owned())
        .current_dir(path)
        .arg("-shared")
        .arg("-Iheader")
        .arg("-o")
        .arg(out)
        .args(sources)
        .status()
        .expect("Failed to compile");
}

fn compile_project(platform: &str,archtecture:&str,out_dir:&PathBuf,proj:&Project,verbose:bool){
    let deps = proj.get_dependencie(vec![platform], vec![archtecture]);
    let log = |txt: String| { if verbose { println!("{}", txt); }};

    log(format!("Building {} for [{}][{}]\n",proj.name,platform, archtecture));
    log("Looking for dependencies:".to_owned());
    for dep in deps.iter() {
        log(format!("Adding dependencie: {} {}", dep.name, dep.version));
    }

    match which::which("gcc"){
        Ok(gcc)=>{
            let sources = generate_objects(&gcc,&proj.get_path(),&out_dir);
            let mut file_name = out_dir.to_owned();
          
            match proj.proj_type.as_str(){
                "app"=>{
                    match platform{
                        "windows" => file_name.push(proj.name.clone()+".exe"),
                        _=>file_name.push(proj.name.clone())
                    };
                    generate_executable(&gcc, &proj.get_path(), &file_name, sources);
                },
                "lib"=>{
                    match platform{
                        "windows" => file_name.push(proj.name.clone()+".dll"),
                        _=>file_name.push(proj.name.clone()+".so")
                    };
                    generate_library(&gcc,&proj.get_path(),&file_name,sources);
                },
                _=>{}
            }
        },
        Err(e)=>println!("GCC not found!\n{:?}",e)
    };

}

