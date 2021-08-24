use crate::file::File;
use crate::stateful_widgets::StatefulList;
use crate::colors::{ColorPallete, ByteColors};

use tui::style::Style;
use tui::style::Color;


#[derive(Clone)]
pub struct Display {
    addresses: StatefulList<(String, Style)>,
    bytes: Vec<StatefulList<(String, Style)>>,
    chars: Vec<StatefulList<(String, Style)>>,

    h_offset: usize,
    v_offset: usize
}

impl Display {
    pub fn new(file: File, colors: ColorPallete) -> Display {
        let addresses = make_addresses(file.get_length(), colors.text());
        let bytes = make_bytes(file.get_bytes(), colors.bc());
        let chars = make_chars(file.get_chars());

        Display {
            addresses,
            bytes,
            chars,

            h_offset: 0,
            v_offset: 0
        }
    }


    pub fn get_adresses(&mut self) -> StatefulList<(String, Style)> {
        self.addresses.clone()
    }

    pub fn get_bytes(&mut self) -> Vec<StatefulList<(String, Style)>> {
        self.bytes.clone()
    }
}

fn make_addresses(length: usize, color: Color) -> StatefulList<(String, Style)> {
    let mut vec_addresses: Vec<(String, Style)> = Vec::new();

    for i in 0..length {
        if (i % 16) == 0 {
            vec_addresses.push(
                format!("{:>0width$X}", i, width = 10),
                Style::default().fg(color)
            )
        }
    }

    StatefulList::new(vec_addresses)
}


fn make_bytes(bytes: Vec<u8>, color: ByteColors) -> Vec<StatefulList<(String, Style)>> {
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

    for byte in data {
        if offset == 16 {
            offset = 0;
        }
        match offset {
            0 => vec_0.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            1 => vec_1.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            2 => vec_2.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            3 => vec_3.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            4 => vec_4.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            5 => vec_5.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            6 => vec_6.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            7 => vec_7.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            8 => vec_8.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            9 => vec_9.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            10 => vec_10.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            11 => vec_11.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            12 => vec_12.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            13 => vec_13.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            14 => vec_14.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            15 => vec_15.push((format!("{:>0w$X}", *byte, w = 2), Style::default().fg(color.get_byte_color(byte)))),
            _ => (),
        }
        offset += 1;
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
    hex
}