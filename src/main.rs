use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(file) = matches.value_of("file") {
        println!("File for open is {}", file);
    };
}
