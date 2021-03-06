mod commands;
mod models;
mod utils;

use clap::ArgMatches;
use clap::{load_yaml, App};
use commands::args::*;
use models::data::*;


use lazy_static::lazy_static;
lazy_static! {
    static ref BUILDER_DATA: BuilderOp = utils::load_builder();
}

fn main() {
    let version = "1.0";
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("version") {
        println!("{}", version);
    }

    new(&matches);
    open(&matches);
    build(&matches);
    show(&matches);
    rm(&matches);
    nv(&matches);
    list(&matches);
}

/// Creates a new project
fn new(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            let args = NewArgs {
                name: matches.value_of("name").unwrap(),
                conf: matches.is_present("conf"),
                p_type: matches.value_of("type")
            };

            commands::cmd_create_project(args);
        }
    }
}

/// Open a project
fn open(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("open") {
        if matches.is_present("name") {
            let args = OpenArgs {
                name: matches.value_of("name").unwrap(),
                version: matches.value_of("version"),
            };

            commands::cmd_open_project(args);
        }
    }
}

/// Build a project
fn build(matches: &ArgMatches) {
    let mut args = BuildArgs {
        name: None,
        platform: None,
        archtecture: None,
        version: None,
        release: false,
        verbose: false,
    };

    if let Some(matches) = matches.subcommand_matches("build") {
        args.name = matches.value_of("name");
        args.platform = match matches.value_of("platform") {
            Some(values) => {
                let spl = values.split(';');
                let mut res = Vec::new();
                for i in spl {
                    res.push(i);
                }
                Some(res)
            }
            None => None,
        };
        args.archtecture = match matches.value_of("archtecture") {
            Some(values) => {
                let spl = values.split(';');
                let mut res = Vec::new();
                for i in spl {
                    res.push(i);
                }
                Some(res)
            }
            None => None,
        };
        args.version = matches.value_of("version");
        if matches.is_present("release") {
            args.release = true
        }
        if matches.is_present("verbose") {
            args.verbose = true
        }

        commands::cmd_build_project(args);
    }
}

/// List projects
fn list(matches: &ArgMatches) {
    let mut args = ListArgs {
        ptype: "all",
        show_versions: false,
        show_deps: false,
    };

    if let Some(mat) = matches.subcommand_matches("list") {
        if let Some(val) = mat.value_of("type") {
            let tmpstrval = val.to_ascii_lowercase();
            let strval = tmpstrval.as_str();
            match strval {
                "project" => {
                    args.ptype = "project";
                }
                "lib" => {
                    args.ptype = "lib";
                }
                _ => {
                    args.ptype = "all";
                }
            }
        } else {
            args.ptype = "all";
        }
        if mat.is_present("show_version") {
            args.show_versions = true
        }
        if mat.is_present("show_dependencies") {
            args.show_versions = true
        }
        commands::cmd_list(args);
    }
}

/// Show Projects
fn show(matches: &ArgMatches) {
    let mut args = ShowArgs{
        name: "",
        version: None
    };
    if let Some(mat) = matches.subcommand_matches("show"){
        
        match mat.value_of("name"){
            Some(v)=>{
                args.version = mat.value_of("version");
                args.name = v;
                commands::cmd_show_project(args);
            },
            None=>{
                println!("Should especify project name");
            }
        }
    }
    

}

fn rm(matches: &ArgMatches) {
   if let Some(rm) = matches.subcommand_matches("rm"){
       let args = RmArgs{
        name: rm.value_of("name").unwrap(),
        recursive: rm.is_present("force"),
        version: rm.value_of("version"),
        force: rm.is_present("force"),
       };
       commands::cmd_rm_project(args);
   }
}

fn nv(matches: &ArgMatches) {
    if let Some(nv) = matches.subcommand_matches("nv"){
        let args = NvArgs{
            name: nv.value_of("name").unwrap(),
            u_type: nv.value_of("type").unwrap(),
            from: nv.value_of("from"),
            to: nv.value_of("to")
        };
        commands::cmd_nv_project(args);
    }
}
