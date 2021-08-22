use std::io;

use clap::{App, load_yaml};

use std::io::Read;

use tui::{
    backend::TermionBackend,
    Terminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Cell, Row, Table},
    text::Spans,
    style::{Color, Style}
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

mod stateful_widgets;



fn main() -> Result<(), io::Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let filename = if let Some(fname) = matches.value_of("file") {
        fname
    } else {
        panic!();
    };

    let mut file = File::new(filename);






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
                        Constraint::Percentage(96),
                        Constraint::Percentage(4),
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
                        Constraint::Length(12),
                        Constraint::Length(49),
                        Constraint::Length(16),
                        Constraint::Length(22)
                    ]
                    .as_ref()
                )
                .split(chunks[0]);

            let addresses: Vec<ListItem> = file.addresses
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from((*i).clone())];
                    ListItem::new(lines)
                })
                .collect();

            let address_block = Block::default()
                .title("Address")
                .borders(Borders::ALL);

            let address_list = List::new(addresses)
                .block(address_block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Red)
                );
            
            frame.render_stateful_widget(address_list, chunks[0], &mut file.addresses.state);


            let hex_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3)

                    ]
                )
                .margin(1)
                .split(chunks[1]);

            
            let mut column_i: usize = 0;
            for column in file.get_hex_view() {
                let addresses: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from((*i).clone())];
                        ListItem::new(lines)
                    })
                    .collect();

                let address_block = Block::default()
                    .borders(Borders::NONE);

                let address_list = List::new(addresses)
                    .block(address_block)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Red)
                    );
                
                frame.render_stateful_widget(address_list, hex_columns[column_i], &mut file.hex_view[column_i].state);
                column_i += 1;
            }

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
                },
                Key::Down => {
                    file.next_address();
                },
                Key::Up => {
                    file.previous_address();
                },
                Key::Right => {
                    file.next_offset();
                },
                Key::Left => {
                    file.previous_offset();
                },
                _ => ()
            }
        }
    }



}
