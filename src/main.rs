use std::io::{self, Read};

use clap::{load_yaml, App};

use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

mod file;
use file::File;

mod stateful_widgets;

mod display;
use display::Display;

mod colors;
use colors::{ColorPallete, ByteColors};


fn make_char(ch: &char) -> String {
    if ((*ch) as u8) > 32 && ((*ch) as u8) < 127 {
        ch.to_string()
    } else if (*ch) as u8 == 32 {
        "_".to_string()
    } else if (*ch) as u8 == 0 {
        "0".to_string()
    } else {
        ".".to_string()
    }
}

fn main() -> Result<(), io::Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let filename = if let Some(fname) = matches.value_of("file") {
        fname
    } else {
        panic!("You should set filname");
    };

    let file = File::new(filename);
    let colors = ColorPallete::new();
    let mut display = Display::new(file, colors.clone());

    

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
                .constraints([Constraint::Percentage(96), Constraint::Percentage(4)].as_ref())
                .split(frame.size());

            let command_block = Block::default()
                .title(Span::styled("Command", Style::default().fg(colors.tl())))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.bs()))
                .style(Style::default().bg(colors.bg()));

            frame.render_widget(command_block, chunks[1]);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(12),
                        Constraint::Length(53),
                        Constraint::Length(18),
                        Constraint::Length(8),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);

            // ADDRESSES

            let addresses: Vec<ListItem> = display
                .get_adresses()
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(Span::styled((*i).0, (*i).1))];
                    ListItem::new(lines)
                })
                .collect();

            let address_block = Block::default()
                .title(Span::styled("Address", Style::default().fg(colors.tl())))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.bs()))
                .style(Style::default().bg(colors.bg()));

            let address_list = List::new(addresses)
                .block(address_block)
                .highlight_style(Style::default().bg(colors.slct()));

            frame.render_stateful_widget(address_list, chunks[0], &mut file.get_adresses().state);

            // HEX

            let raw_view_block = Block::default()
                .title(Span::styled("Hex", Style::default().fg(colors.tl())))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.bs()))
                .style(Style::default().bg(colors.bg()));
            frame.render_widget(raw_view_block, chunks[1]);

            let hex_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(3), // EMPTY
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(1),
                ])
                .margin(1)
                .split(chunks[1]);

            let mut column_i: usize = 0;
            let mut hex_i: usize = 1;
            for column in display.get_bytes() {
                let hex: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from(Span::styled((*i).0, (*i).1))];
                        ListItem::new(lines)
                    })
                    .collect();

                let hex_block = Block::default().borders(Borders::NONE);

                let hex_list = List::new(hex)
                    .block(hex_block)
                    .highlight_style(Style::default().bg(colors.slct()));
                frame.render_stateful_widget(
                    hex_list,
                    hex_columns[hex_i],
                    &mut file.get_hex_view()[column_i].state,
                );
                column_i += 1;
                hex_i += 2;
                //if hex_i == 15 {
                //    hex_i = 18;
                //}
            }

            // ACII

            let char_view_block = Block::default()
                .title(Span::styled("ASCII", Style::default().fg(colors.tl())))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.bs()))
                .style(Style::default().bg(colors.bg()));
            frame.render_widget(char_view_block, chunks[2]);

            let ascii_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
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
                    Constraint::Length(1),
                ])
                .margin(1)
                .split(chunks[2]);

            let mut column_i: usize = 0;
            for column in file.get_ascii_view() {
                let ascii: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from(Span::styled(
                            make_char(i),
                            Style::default().fg(pick_color(&(*i as u8))),
                        ))];
                        ListItem::new(lines)
                    })
                    .collect();

                let ascii_block = Block::default().borders(Borders::NONE);

                let ascii_list = List::new(ascii)
                    .block(ascii_block)
                    .highlight_style(Style::default().bg(colors.slct()));

                frame.render_stateful_widget(
                    ascii_list,
                    ascii_columns[column_i],
                    &mut file.get_ascii_view()[column_i].state,
                );
                column_i += 1;
            }

            let nothing = Block::default().style(Style::default().bg(colors.bg()));
            frame.render_widget(nothing, chunks[3]);
        })?;

        for key in asi.by_ref().keys() {
            match key.unwrap() {
                // If any of them is q, quit
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Down => {
                    file.next_address();
                }
                Key::Up => {
                    file.previous_address();
                }
                Key::Right => {
                    file.next_offset();
                }
                Key::Left => {
                    file.previous_offset();
                }
                _ => (),
            }
        }
    }
}
