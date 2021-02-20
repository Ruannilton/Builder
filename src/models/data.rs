use crate::utils;
use chrono::NaiveTime;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
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

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Platform {
    pub name: String,
    pub arch: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub proj_type: String,
    pub desc: Option<String>,
    pub platform: Vec<Platform>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub lib_type: String,
    pub desc: Option<String>,
    pub platform: Vec<Platform>,
}

impl ProjectLog {
    pub fn save(&self) {
        let mut op = Project::get_project_root();
        op.push(self.name.clone());
        let content = serde_json::to_string_pretty(self).expect("Failed to parse Project");
        if op.is_dir() {
            op.push("log.json");
            fs::write(op, content).expect("Failed to save Project");
        } else {
            println!("Project folder doesn´t exist");
        }
    }
    pub fn load(name: String) -> Option<ProjectLog> {
        let mut op = match Project::load(&name, None).unwrap().get_path().parent(){
            Some(val)=>PathBuf::from(val),
            None=>{ println!("Could not resolve log path"); return None;}
        };
        if op.is_dir() {
            op.push("log.json");
            let content = fs::read_to_string(op.to_owned()).expect(format!("failed to load project config at {:?}",op).as_str());
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
    //Save a project to disk
    pub fn save(&self) {
        let mut op = self.get_path().to_owned();
        let content = toml::to_string_pretty(self).expect("Failed to parse Project");
        if op.is_dir() {
            op.push("conf.toml");
            fs::write(op, content).expect("Failed to save Project");
        } else {
            println!("Project folder doesn´t exist");
        }
    }

    //Parse project configuration to hash map
    fn parse_conf_helper(plat: &Platform)-> HashMap<String, Vec<Dependencie>> {
        let mut hm = HashMap::<String, Vec<Dependencie>>::new();
        //ITERATE ON ARCHS
       let archs = plat.arch.clone();
        for arch in archs {
            let arc= arch.clone();
            if hm.contains_key(&arc) {
                if let Some(deps) = hm.get_mut(&arc) {
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
                hm.insert(arc.clone(), tmp_vec.clone());
            }
        }
        hm
    }

    pub fn parse_conf(&self) -> HashMap<&str, HashMap<String, Vec<Dependencie>>> {
        let mut parsed = HashMap::<&str, HashMap<String, Vec<Dependencie>>>::new();
        
        //ITERATE ON PLATAFORMS
        for plat in self.platform.iter() {
            parsed
                .entry(&plat.name)
                .and_modify(|hm| {
                    //ITERATE ON ARCHS
                    for arch in plat.arch.iter() {
                        let arc:&str = arch.as_ref();
                        if hm.contains_key(arc) {
                            if let Some(deps) = hm.get_mut(arc) {
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
                .or_insert(Project::parse_conf_helper(plat));
        }
        parsed
    }

    //Try get the project path
    pub fn get_path(&self) -> PathBuf {
        Project::make_path(&self.name, Some(self.version.as_ref()))
    }

    pub fn get_versions(&self) -> Vec<Version> {
        Project::version_list(&self.name)
    }

    pub fn get_dependencie(&self, platforms: Vec<&str>, archs: Vec<&str>) -> Vec<Dependencie>{
        let dependencie_tree = self.parse_conf();
       
        let mut universal_deps = Vec::<Dependencie>::new();
        if let Some(all_plat) = dependencie_tree.get("all") {
            if let Some(all_deps) = all_plat.get("all") {
                universal_deps = all_deps.clone();
            }
        }

        let mut vec : Vec<Dependencie>= Vec::new();
        for platform in platforms{
            if let Some(plat) = dependencie_tree.get(platform){
                let plat: &HashMap<String, Vec<Dependencie>> = plat;
                for archtecture in archs.iter(){
                    if let Some(arch_deps) = plat.get(archtecture.to_owned()){
                        
                        vec.extend(universal_deps.clone());
                        if let Some(deps) = plat.get("all"){
                            vec.extend(deps.clone());
                        }
                        if let Some(all) = dependencie_tree.get("all"){
                            if let Some(deps) = all.get(archtecture.to_owned()){
                                vec.extend(deps.clone())
                            }
                        }
                        vec.extend(arch_deps.clone());
                    }
                }
            }
        }
        vec
    }

    pub fn list() -> Vec<Project> {
        let proj_path = Project::get_project_root();
        let mut vec: Vec<Project> = Vec::new();
        for f in proj_path.read_dir().unwrap() {
            if let Ok(folder) = f {
                let p: PathBuf = folder.path();
                if p.is_dir() {
                    let name = p.file_name().unwrap().to_str().unwrap().to_owned();
                    if Project::exist(&name, None) {
                        if let Some(proj) = Project::load(&name, None) {
                            vec.push(proj);
                        }
                    }
                }
            }
        }
        vec
    }

    //Try load a project from name
    pub fn load(name: &String, version: Option<&str>) -> Option<Project> {
        let mut op = Project::make_path(name, version);
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

    pub fn exist(name: &String, version: Option<&str>) -> bool {
        match version {
            Some(v) => {
                let mut path = Project::make_path(name, Some(v));
                path.push("conf.toml");
                path.is_file()
            }
            None => {
                let mut proj_dir = Project::get_project_root();
                proj_dir.push(name.clone());
                proj_dir.push("log.json");
                proj_dir.is_file()
            }
        }
    }

    pub fn get_project_root() -> PathBuf {
        let cfg = &crate::BUILDER_DATA;
        PathBuf::from(cfg.projects_dir.clone())
    }
    
    pub fn version_list(name: &String) -> Vec<Version> {
        let mut proj_dir = Project::get_project_root();
        proj_dir.push(name.clone());
        let mut versions = Vec::new();
        if !proj_dir.is_dir() {
            return versions;
        }
        for f in proj_dir.read_dir().unwrap() {
            if let Ok(folder) = f {
                let p: PathBuf = folder.path();
                let folder_name = p.file_name().unwrap().to_str().unwrap();
                let spl: Vec<&str> = folder_name.split('_').collect();
                if spl[0] == name {
                    match Version::parse(spl[1]) {
                        Ok(v) => versions.push(v),
                        Err(_) => println!("{} has no valid version", folder_name),
                    }
                };
            }
        }
        versions.sort_unstable_by(|a, b| a.cmp(b));
        versions
    }

    pub fn make_path(name: &String, version: Option<&str>) -> PathBuf {
        let cfg = &crate::BUILDER_DATA;
        let mut proj_dir = PathBuf::from(cfg.projects_dir.clone());
        proj_dir.push(name.clone());
        let v :String = match version {
            Some(v) => String::from(v.clone()),
            None => {
                let vec = Project::version_list(name);
                let vers = vec.last();
                match vers {
                    Some(ver) => {
                        format!("{}.{}.{}", ver.major, ver.minor, ver.patch)
                },
                    None => String::from("1.0.0"),
                }
            }
        };
        let mut folder_name = name.clone();
        folder_name.push('_');
        folder_name.push_str(v.as_ref());

        proj_dir.push(folder_name);
        proj_dir
    }
}

impl Library {
    pub fn parse_conf(&self) -> HashMap<String, HashMap<String, Vec<Dependencie>>> {
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
        for plat in self.platform.iter() {
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

    pub fn get_path(&self) -> PathBuf {
        Library::make_path(&self.name, Some(self.version.clone()))
    }

    pub fn get_versions(&self) -> Vec<Version> {
        Library::version_list(&self.name)
    }

    pub fn list() -> Vec<Library> {
        let proj_path = Library::get_library_root();
        let mut vec: Vec<Library> = Vec::new();
        for f in proj_path.read_dir().unwrap() {
            if let Ok(folder) = f {
                let p: PathBuf = folder.path();
                if p.is_dir() {
                    let name = p.file_name().unwrap().to_str().unwrap().to_owned();
                    if Library::exist(&name, None) {
                        if let Some(lib) = Library::load(&name, None) {
                            vec.push(lib);
                        }
                    }
                }
            }
        }
        vec
    }

    //Try load a project from name
    pub fn load(name: &String, version: Option<String>) -> Option<Library> {
        let mut op = Library::make_path(name, version);
        if op.is_dir() {
            op.push("conf.toml");
            let content = fs::read_to_string(op).expect("failed to load project config");
            let proj: Library = toml::from_str(&content).expect("failed to parse project config");
            Some(proj)
        } else {
            println!("Project folder doesn´t exist");
            None
        }
    }

    pub fn exist(name: &String, version: Option<String>) -> bool {
        match version {
            Some(v) => {
                let mut path = Library::make_path(name, Some(v));
                path.push("conf.toml");
                path.is_file()
            }
            None => {
                let mut proj_dir = Library::get_library_root();
                proj_dir.push(name.clone());
                proj_dir.push("log.json");
                proj_dir.is_file()
            }
        }
    }
    pub fn get_library_root() -> PathBuf {
        let mut proj_dir = utils::get_builder_path();
        proj_dir.push("libs");
        proj_dir
    }
    pub fn version_list(name: &String) -> Vec<Version> {
        let mut proj_dir = Library::get_library_root();
        proj_dir.push(name.clone());
        let mut versions = Vec::new();

        for f in proj_dir.read_dir().unwrap() {
            if let Ok(folder) = f {
                let p: PathBuf = folder.path();
                let folder_name = p.file_name().unwrap().to_str().unwrap();
                if folder_name.starts_with("name") {
                    let spl: Vec<&str> = folder_name.split('_').collect();
                    match Version::parse(spl[1]) {
                        Ok(v) => versions.push(v),
                        Err(_) => println!("{} has no valid version", folder_name),
                    }
                };
            }
        }
        versions.sort_unstable_by(|a, b| a.cmp(b));
        versions
    }
    pub fn make_path(name: &String, version: Option<String>) -> PathBuf {
        let mut proj_dir = Library::get_library_root();
        proj_dir.push(name.clone());
        let v = match version {
            Some(v) => v.clone(),
            None => {
                let vec = Library::version_list(name);
                let ver: &Version = vec.last().unwrap();
                String::from(format!("{}.{}.{}", ver.major, ver.minor, ver.patch))
            }
        };
        let mut folder_name = name.clone();
        folder_name.push('_');
        folder_name.push_str(v.as_ref());

        proj_dir.push(folder_name);
        proj_dir
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
