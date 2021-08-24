use crate::file::File;
use crate::stateful_widgets::StatefulList;
use crate::ColorPallete;

use tui::style::Style;
use tui::style::Color;

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
        let bytes = make_bytes(file.get_bytes());
        let chars = make_chars(file.get_chars());

        Display {
            addresses,
            bytes,
            chars,

            h_offset: 0,
            v_offset: 0
        }
    }
}

fn make_addresses(length: usize, color: Color) -> StatefulList<(String, Style)> {
    let mut vec_addresses: Vec<(String, Style)> = Vec::new();

    for i in 0..length {
        if (i % 16) == 0 {
            vec_addresses.push(format!("{:>0width$X}", i, width = 10))
        }
    }

    StatefulList::new(vec_addresses)
}