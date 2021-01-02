mod project_parser;

fn main() {
    let proj = project_parser::load_project_config("teste.toml");
    match proj {
        Err(e) => println!("ERROR! {}", e),
        Ok(v) => println!("{}", v),
    }
}
