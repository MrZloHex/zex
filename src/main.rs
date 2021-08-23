use std::io;

use clap::{App, load_yaml};

use std::io::Read;

use tui::{
    backend::TermionBackend,
    Terminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    text::{Span, Spans},
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
                .title(Span::styled("Command", Style::default().fg(Color::Rgb(200, 200, 200))))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(150, 150, 150)))
                .style(
                    Style::default().bg(Color::Rgb(30, 30, 30))
                );

            frame.render_widget(command_block, chunks[1]);


            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(12),
                        Constraint::Length(52),
                        Constraint::Length(18),
                        Constraint::Length(8)
                    ]
                    .as_ref()
                )
                .split(chunks[0]);

            let addresses: Vec<ListItem> = file.get_adresses()
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from((*i).clone())];
                    ListItem::new(lines)
                })
                .collect();

            let address_block = Block::default()
                .title(Span::styled("Address", Style::default().fg(Color::Rgb(200, 200, 200))))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(150, 150, 150)))
                .style(
                    Style::default().bg(Color::Rgb(30, 30, 30))
                );

            let address_list = List::new(addresses)
                .block(address_block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Red)
                );
            
            frame.render_stateful_widget(address_list, chunks[0], &mut file.get_adresses().state);

            // HEX


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
                        Constraint::Length(2),  // EMPTY
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
            let mut hex_i: usize = 0;
            for column in file.get_hex_view() {
                let hex: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from((*i).clone())];
                        ListItem::new(lines)
                    })
                    .collect();

                let hex_block = Block::default()
                    .borders(Borders::NONE);

                let hex_list = List::new(hex)
                    .block(hex_block)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Red)
                    );
                frame.render_stateful_widget(hex_list, hex_columns[hex_i], &mut file.get_hex_view()[column_i].state);
                column_i += 1;
                hex_i += 1;
                if hex_i == 8 {
                    hex_i = 9;
                }
            }

            let raw_view_block = Block::default()
                .title(Span::styled("Hex", Style::default().fg(Color::Rgb(200, 200, 200))))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(150, 150, 150)))
                .style(
                    Style::default().bg(Color::Rgb(30, 30, 30))
                );

            
            frame.render_widget(raw_view_block, chunks[1]);

            // ACII

            let ascii_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1)

                    ]
                )
                .margin(1)
                .split(chunks[2]);

            
            let mut column_i: usize = 0;
            for column in file.get_ascii_view() {
                let ascii: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from((*i).clone())];
                        ListItem::new(lines)
                    })
                    .collect();

                let ascii_block = Block::default()
                    .borders(Borders::NONE);

                let ascii_list = List::new(ascii)
                    .block(ascii_block)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Red)
                    );
                
                frame.render_stateful_widget(ascii_list, ascii_columns[column_i], &mut file.get_ascii_view()[column_i].state);
                column_i += 1;
            }

            let char_view_block = Block::default()
                .title(Span::styled("ASCII", Style::default().fg(Color::Rgb(200, 200, 200))))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(150, 150, 150)))
                .style(
                    Style::default().bg(Color::Rgb(30, 30, 30))
                );
            frame.render_widget(char_view_block, chunks[2]);


            let nothing = Block::default()
                .border_style(Style::default().fg(Color::Rgb(150, 150, 150)))
                .style(
                    Style::default().bg(Color::Rgb(30, 30, 30))
                );
            frame.render_widget(nothing, chunks[3]);


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
