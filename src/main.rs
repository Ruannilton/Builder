mod commands;
mod models;
mod utils;

use crate::models::*;
use clap::ArgMatches;
use clap::{load_yaml, App};
use models::BuilderOp;

fn main() {
    let version = "1.0";
    let config = utils::load_builder_config();
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("version") {
        println!("{}", version);
    }

    new(&matches, &config);
    open(&matches, &config);
    build(&matches, &config);
    show(&matches, &config);
    rm(&matches, &config);
    nv(&matches, &config);
    list(&matches, &config);
}

fn new(matches: &ArgMatches, config: &BuilderOp) {
    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            let args = NewArgs {
                name: matches.value_of("name").unwrap(),
                conf: matches.is_present("conf"),
            };

            commands::cmd_create_project(config, args);
        }
    }
}

fn open(matches: &ArgMatches, config: &BuilderOp) {
    if let Some(matches) = matches.subcommand_matches("open") {
        if matches.is_present("name") {
            let args = OpenArgs {
                name: matches.value_of("name").unwrap(),
                version: matches.value_of("version"),
            };

            commands::cmd_open_project(config, args);
        }
    }
}

fn build(matches: &ArgMatches, config: &BuilderOp) {}

fn show(matches: &ArgMatches, config: &BuilderOp) {}

fn rm(matches: &ArgMatches, config: &BuilderOp) {}

fn nv(matches: &ArgMatches, config: &BuilderOp) {}

fn list(matches: &ArgMatches, config: &BuilderOp) {}
