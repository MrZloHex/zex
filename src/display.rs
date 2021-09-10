use crate::file::File;
use crate::stateful_widgets::StatefulList;
use crate::colors::{ColorPallete, ByteColors};

use unicode_width::UnicodeWidthStr;
use tui::style::Style;
use tui::style::Color;


#[derive(Clone)]
pub enum InputMode {
    Normal,
    Editing
}


#[derive(Clone)]
pub struct Display {
    file: File,

    addresses: StatefulList<(String, Style)>,
    bytes: Vec<StatefulList<(String, Style)>>,
    chars: Vec<StatefulList<(String, Style)>>,

    h_offset: usize,
    v_offset: usize,

    max_v_offset: [usize; 16],
    max_h_offset: usize,

    colors: ColorPallete,

    command: String,
    pub input: InputMode
}

impl Display {
    pub fn new(fl: File, colors: ColorPallete) -> Display {
        let addresses = make_addresses(fl.get_length(), colors.text());
        let (bytes, max_v_offset) = make_bytes(fl.get_bytes(), colors.bc(), colors.bg());
        let chars = make_chars(fl.get_chars(), colors.bc(), colors.bg());

        Display {
            file: fl,
            addresses,
            bytes,
            chars,

            h_offset: 0,
            v_offset: 0,
            max_v_offset,
            max_h_offset: 15,

            colors,
            
            command: String::new(),
            input: InputMode::Normal
        }
    }


    pub fn get_adresses(&mut self) -> StatefulList<(String, Style)> {
        self.addresses.clone()
    }

    pub fn get_bytes(&mut self) -> Vec<StatefulList<(String, Style)>> {
        self.bytes.clone()
    }

    pub fn get_chars(&mut self) -> Vec<StatefulList<(String, Style)>> {
        self.chars.clone()
    }




    pub fn next_address(&mut self) {
        if self.v_offset != self.max_v_offset[self.h_offset] { 
            self.v_offset += 1;

        }
    }

    pub fn prev_address(&mut self) {
        if self.v_offset != 0 { 
            self.v_offset -= 1;
        }
    }

    pub fn next_offset(&mut self) {
        if self.h_offset != self.max_h_offset && !(self.max_v_offset[self.h_offset] != self.max_v_offset[self.h_offset+1] && self.v_offset == self.max_v_offset[self.h_offset]) {
            self.h_offset += 1;
        }
    }

    pub fn prev_offset(&mut self) {
        if self.h_offset != 0 {
            self.h_offset -= 1;
        }
    }

    pub fn update_cursor_pos(&mut self) {
        self.addresses.select(self.v_offset.clone());

        for i in 0..=self.max_h_offset {
            self.bytes[i].select(self.v_offset.clone());
            self.chars[i].select(self.v_offset.clone());
        }
    }

    pub fn get_hl_style(&mut self, index: usize) -> Style {
        if index == self.h_offset {
            Style::default().bg(self.colors.slct())
        } else {
            Style::default().bg(self.colors.bg())
        }
    }



    pub fn push_ch_command(&mut self, ch: char) {
        self.command.push(ch);
    }

    pub fn set_command(&mut self, s: String) {
        self.command = s;
    }

    pub fn pop_command(&mut self) {
        self.command.pop();
    }

    pub fn command_width(&mut self) -> usize {
        self.command.width()
    }

    pub fn get_command(&mut self) -> String {
        self.command.clone()
    }
}



fn make_addresses(length: usize, color: Color) -> StatefulList<(String, Style)> {
    let mut vec_addresses: Vec<(String, Style)> = Vec::new();

    for i in 0..length {
        if (i % 16) == 0 {
            vec_addresses.push((
                format!("{:>0width$X}", i, width = 10),
                Style::default().fg(color)
            ))
        }
    }

    StatefulList::new(vec_addresses.clone())
}


fn make_bytes(bytes: Vec<u8>, color: ByteColors, bg: Color) -> (Vec<StatefulList<(String, Style)>>, [usize; 16]) {
    let mut vec_0 = Vec::new();
    let mut vec_1 = Vec::new();
    let mut vec_2 = Vec::new();
    let mut vec_3 = Vec::new();
    let mut vec_4 = Vec::new();
    let mut vec_5 = Vec::new();
    let mut vec_6 = Vec::new();
    let mut vec_7 = Vec::new();
    let mut vec_8 = Vec::new();
    let mut vec_9 = Vec::new();
    let mut vec_10 = Vec::new();
    let mut vec_11 = Vec::new();
    let mut vec_12 = Vec::new();
    let mut vec_13 = Vec::new();
    let mut vec_14 = Vec::new();
    let mut vec_15 = Vec::new();
    let mut offset: u8 = 0;

    for byte in bytes {
        if offset == 16 {
            offset = 0;
        }
        match offset {
            0 => vec_0.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            1 => vec_1.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            2 => vec_2.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            3 => vec_3.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            4 => vec_4.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            5 => vec_5.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            6 => vec_6.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            7 => vec_7.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            8 => vec_8.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            9 => vec_9.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            10 => vec_10.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            11 => vec_11.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            12 => vec_12.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            13 => vec_13.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            14 => vec_14.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            15 => vec_15.push((format!("{:>0w$X}", byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            _ => (),
        }
        offset += 1;
    }
    let length: [usize; 16] = [
        vec_0.len()-1,
        vec_1.len()-1,
        vec_2.len()-1,
        vec_3.len()-1,
        vec_4.len()-1,
        vec_5.len()-1,
        vec_6.len()-1,
        vec_7.len()-1,
        vec_8.len()-1,
        vec_9.len()-1,
        vec_10.len()-1,
        vec_11.len()-1,
        vec_12.len()-1,
        vec_13.len()-1,
        vec_14.len()-1,
        vec_15.len()-1,
    ];

    let max_length: usize = length[0];
    let filler = ("  ".to_string(), Style::default().fg(bg));
    for i in 0..16 {
        if length[i] < max_length {
            match i {
                1 => vec_1.push(filler.clone()),
                2 => vec_2.push(filler.clone()),
                3 => vec_3.push(filler.clone()),
                4 => vec_4.push(filler.clone()),
                5 => vec_5.push(filler.clone()),
                6 => vec_6.push(filler.clone()),
                7 => vec_7.push(filler.clone()),
                8 => vec_8.push(filler.clone()),
                9 => vec_9.push(filler.clone()),
                10 => vec_10.push(filler.clone()),
                11 => vec_11.push(filler.clone()),
                12 => vec_12.push(filler.clone()),
                13 => vec_13.push(filler.clone()),
                14 => vec_14.push(filler.clone()),
                15 => vec_15.push(filler.clone()),
                _ => ()
            }
        }
    }

    let hex = vec![
        StatefulList::new(vec_0),
        StatefulList::new(vec_1),
        StatefulList::new(vec_2),
        StatefulList::new(vec_3),
        StatefulList::new(vec_4),
        StatefulList::new(vec_5),
        StatefulList::new(vec_6),
        StatefulList::new(vec_7),
        StatefulList::new(vec_8),
        StatefulList::new(vec_9),
        StatefulList::new(vec_10),
        StatefulList::new(vec_11),
        StatefulList::new(vec_12),
        StatefulList::new(vec_13),
        StatefulList::new(vec_14),
        StatefulList::new(vec_15),
        ];
    
    (hex, length)
}

fn make_chars(bytes: Vec<char>, color: ByteColors, bg: Color) -> Vec<StatefulList<(String, Style)>> {
    let mut vec_0 = Vec::new();
        let mut vec_1 = Vec::new();
        let mut vec_2 = Vec::new();
        let mut vec_3 = Vec::new();
        let mut vec_4 = Vec::new();
        let mut vec_5 = Vec::new();
        let mut vec_6 = Vec::new();
        let mut vec_7 = Vec::new();
        let mut vec_8 = Vec::new();
        let mut vec_9 = Vec::new();
        let mut vec_10 = Vec::new();
        let mut vec_11 = Vec::new();
        let mut vec_12 = Vec::new();
        let mut vec_13 = Vec::new();
        let mut vec_14 = Vec::new();
        let mut vec_15 = Vec::new();
        let mut offset: u8 = 0;

        for byte in bytes {
            if offset == 16 {
                offset = 0;
            }
            match offset {
                0 => vec_0.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                1 => vec_1.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                2 => vec_2.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                3 => vec_3.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                4 => vec_4.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                5 => vec_5.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                6 => vec_6.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                7 => vec_7.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                8 => vec_8.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                9 => vec_9.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                10 => vec_10.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                11 => vec_11.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                12 => vec_12.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                13 => vec_13.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                14 => vec_14.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                15 => vec_15.push((make_char(&byte), Style::default().fg(color.get_byte_color(byte as u8)))),
                _ => (),
            }
            offset += 1;
        }

        let length: [usize; 16] = [
            vec_0.len()-1,
            vec_1.len()-1,
            vec_2.len()-1,
            vec_3.len()-1,
            vec_4.len()-1,
            vec_5.len()-1,
            vec_6.len()-1,
            vec_7.len()-1,
            vec_8.len()-1,
            vec_9.len()-1,
            vec_10.len()-1,
            vec_11.len()-1,
            vec_12.len()-1,
            vec_13.len()-1,
            vec_14.len()-1,
            vec_15.len()-1,
        ];

        let max_length: usize = length[0];
        let filler = ("  ".to_string(), Style::default().fg(bg));
        for i in 0..16 {
            if length[i] < max_length {
                match i {
                    1 => vec_1.push(filler.clone()),
                    2 => vec_2.push(filler.clone()),
                    3 => vec_3.push(filler.clone()),
                    4 => vec_4.push(filler.clone()),
                    5 => vec_5.push(filler.clone()),
                    6 => vec_6.push(filler.clone()),
                    7 => vec_7.push(filler.clone()),
                    8 => vec_8.push(filler.clone()),
                    9 => vec_9.push(filler.clone()),
                    10 => vec_10.push(filler.clone()),
                    11 => vec_11.push(filler.clone()),
                    12 => vec_12.push(filler.clone()),
                    13 => vec_13.push(filler.clone()),
                    14 => vec_14.push(filler.clone()),
                    15 => vec_15.push(filler.clone()),
                    _ => ()
                }
            }
        }

        let ascii = vec![
            StatefulList::new(vec_0),
            StatefulList::new(vec_1),
            StatefulList::new(vec_2),
            StatefulList::new(vec_3),
            StatefulList::new(vec_4),
            StatefulList::new(vec_5),
            StatefulList::new(vec_6),
            StatefulList::new(vec_7),
            StatefulList::new(vec_8),
            StatefulList::new(vec_9),
            StatefulList::new(vec_10),
            StatefulList::new(vec_11),
            StatefulList::new(vec_12),
            StatefulList::new(vec_13),
            StatefulList::new(vec_14),
            StatefulList::new(vec_15),
        ];
        ascii
}

fn make_char(ch: &char) -> String {
    if ((*ch) as u8) > 32 && ((*ch) as u8) < 127 {
        (*ch).to_string()
    } else if ((*ch) as u8) == 32 {
        "_".to_string()
    } else if ((*ch) as u8) == 0 {
        "0".to_string()
    } else {
        ".".to_string()
    }
}
