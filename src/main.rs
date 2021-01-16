mod commands;
mod models;
mod utils;

use clap::{load_yaml, App};

fn main() {
    let version = "1.0";
    let config = utils::load_builder_config();
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("version") {
        println!("{}", version);
    }

    if let Some(ref matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            if matches.is_present("conf") {
                commands::cmd_create_project(
                    &config,
                    matches.value_of("name").unwrap().to_owned(),
                    true,
                );
            } else {
                commands::cmd_create_project(
                    &config,
                    matches.value_of("name").unwrap().to_owned(),
                    false,
                );
            }
        }
    }
}
