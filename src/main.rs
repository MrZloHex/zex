use std::io;

use clap::{App, load_yaml};

use std::io::Read;

use tui::{
    backend::TermionBackend,
    Terminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders}
};
use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen
};

mod file;
use file::File;



fn main() -> Result<(), io::Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let filename = if let Some(fname) = matches.value_of("file") {
        fname
    } else {
        panic!();
    };

    let file = File::new(filename);






    // tui

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut asi = async_stdin();
    
    terminal.clear()?;

    loop {

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(95),
                        Constraint::Percentage(5),
                    ]
                    .as_ref()
                )
                .split(frame.size());
            
            let command_block = Block::default()
                .title("Command")
                .borders(Borders::ALL);

            frame.render_widget(command_block, chunks[1]);


            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(50),
                        Constraint::Percentage(40)
                    ]
                    .as_ref()
                )
                .split(chunks[0]);

            let address_block = Block::default()
                .title("Address")
                .borders(Borders::ALL);
            frame.render_widget(address_block, chunks[0]);

            let raw_view_block = Block::default()
                .title("HEX View")
                .borders(Borders::ALL);
            frame.render_widget(raw_view_block, chunks[1]);

            let char_view_block = Block::default()
                .title("ASCII View")
                .borders(Borders::ALL);
            frame.render_widget(char_view_block, chunks[2]);


        })?;


        for key in asi.by_ref().keys() {
            match key.unwrap() {
                // If any of them is q, quit
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                _ => (),
            }
        }
    }



}
