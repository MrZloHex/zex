use std::io::{self, Read};

use clap::{load_yaml, App};

use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::{Terminal, backend::TermionBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Span, Spans}, widgets::{Block, Borders, List, ListItem, Paragraph}};

mod file;
use file::File;

mod stateful_widgets;

mod display;
use display::{Display, InputMode};

mod colors;
use colors::{ColorPallete, ByteColors};




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

    let mut statement = true;
    let mut last_command_succes = true;
    // let mut update = false;

    loop {
        if !statement {
            terminal.clear()?;
            return Ok(());
        }

        // if update {
        //     update = false;
        //     display.update();
        // }

        display.update_cursor_pos();
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
            let command_str = display.get_command();
            let command = Paragraph::new(command_str.as_ref())
                .block(command_block);

            frame.render_widget(command, chunks[1]);
            match display.input {
                InputMode::Normal => {}
                InputMode::Editing => {
                    frame.set_cursor(
                        chunks[1].x + display.command_width() as u16 + 1,
                        chunks[1].y + 1
                    )
                }
            }



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
                .map(|address| {
                    let lines = vec![Spans::from(Span::styled((*address).0.clone(), (*address).1.clone()))];
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

            frame.render_stateful_widget(address_list, chunks[0], &mut display.get_adresses().state);

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
                    .map(|byte| {
                        let lines = vec![Spans::from(Span::styled((*byte).0.clone(), (*byte).1))];
                        ListItem::new(lines)
                    })
                    .collect();

                let hex_block = Block::default().borders(Borders::NONE);

                let hex_list = List::new(hex)
                    .block(hex_block)
                    .highlight_style(display.get_hl_style(column_i.clone()));
                frame.render_stateful_widget(
                    hex_list,
                    hex_columns[hex_i],
                    &mut display.get_bytes()[column_i].state,
                );
                column_i += 1;
                hex_i += 2;
            }

            // ASCII

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
            for column in display.get_chars() {
                let ascii: Vec<ListItem> = column
                    .items
                    .iter()
                    .map(|ch| {
                        let lines = vec![Spans::from(Span::styled((*ch).0.clone(), (*ch).1))];
                        ListItem::new(lines)
                    })
                    .collect();

                let ascii_block = Block::default().borders(Borders::NONE);

                let ascii_list = List::new(ascii)
                    .block(ascii_block)
                    .highlight_style(display.get_hl_style(column_i.clone()));

                frame.render_stateful_widget(
                    ascii_list,
                    ascii_columns[column_i],
                    &mut display.get_chars()[column_i].state,
                );
                column_i += 1;
            }

            let nothing = Block::default().style(Style::default().bg(colors.bg()));
            frame.render_widget(nothing, chunks[3]);
        })?;


        for key in asi.by_ref().keys() {
            match display.input {
                InputMode::Normal => match key.unwrap() {
                    Key::Down => {
                        display.next_address();
                    }
                    Key::Up => {
                        display.prev_address();
                    }
                    Key::Right => {
                        display.next_offset();
                    }
                    Key::Left => {
                        display.prev_offset();
                    }
                    Key::Char(':') => {
                        display.input = InputMode::Editing;
                        if !display.get_command().is_empty() {
                            if display.get_command().chars().nth(0).unwrap() != ':' || last_command_succes {
                                display.set_command(":".to_string());
                            }
                        } else {
                            display.push_ch_command(':');
                        }
                    }
                    _ => ()
                },
                InputMode::Editing => match key.unwrap() {
                    Key::Char('\n') => {
                        execute_command(
                            display.get_command().drain(..).collect(),  
                            &mut statement, 
                            &mut last_command_succes,
                             &mut display
                        );
                        display.input = InputMode::Normal;
                    }
                    Key::Esc => {
                        display.input = InputMode::Normal
                    }
                    Key::Char(ch) => {
                        display.push_ch_command(ch);
                    }
                    Key::Backspace => {
                        display.pop_command();
                    }
                    _ => ()
                }
            }
        }
    }
}


fn execute_command(command: String, state: &mut bool, command_state: &mut bool, display: &mut Display) {
    if command.eq(":q") {
        *state = false;
    } else if command.eq(":w") {
        display.save_file();
    } else if command.starts_with(":c") {
        let new_value = match command.strip_prefix(":c").unwrap().parse::<u8>() {
            Ok(u) => u,
            Err(_) => {
                display.set_command("Incorrect value to write (from 0 to 255)".to_string());
                *command_state = false;
                return;
            }
        };
        display.set_byte(new_value);
        display.update();
        *command_state = true;
        // *update = true;
    } else {
        display.set_command("Incorrect command, check README for commands".to_string());
        *command_state = false;
        return
    }
}
