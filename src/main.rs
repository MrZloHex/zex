use clap::{App, load_yaml};

use std::fs::{self, File};
use std::io::{self, Read};
use tui::{
    backend::TermionBackend,
    Terminal
};
use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::IntoRawMode
};

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let filename = if let Some(file) = matches.value_of("file") {
        file
    } else {
        panic!();
    };

    // for char in get_file_as_byte_vec(filename) {
    //     println!("{:X}", char);
    // }






    // tui

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut asi = async_stdin();

    

}
