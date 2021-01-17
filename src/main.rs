mod commands;
mod models;
mod utils;

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
            if matches.is_present("conf") {
                commands::cmd_create_project(
                    config,
                    matches.value_of("name").unwrap().to_owned(),
                    true,
                );
            } else {
                commands::cmd_create_project(
                    config,
                    matches.value_of("name").unwrap().to_owned(),
                    false,
                );
            }
        }
    }
}

fn open(matches: &ArgMatches, config: &BuilderOp) {}

fn build(matches: &ArgMatches, config: &BuilderOp) {}

fn show(matches: &ArgMatches, config: &BuilderOp) {}

fn rm(matches: &ArgMatches, config: &BuilderOp) {}

fn nv(matches: &ArgMatches, config: &BuilderOp) {}

fn list(matches: &ArgMatches, config: &BuilderOp) {}
