use std::fs;
use std::io::Read;

use super::stateful_widgets::StatefulList;

#[derive(Clone)]
pub struct File {
    filename: String,
    bytes: Vec<u8>,
    chars: Vec<char>,
    length: usize
}

impl File {
    pub fn new(filename: &str) -> File {
        let bytes = File::get_file_data(filename);
        let chars = File::chars_from_u8(&bytes);

        File {
            filename: filename.to_string(),
            length: bytes.len(),
            bytes,
            chars
        }
    }

    fn get_file_data(filename: &str) -> Vec<u8> {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }

    fn chars_from_u8(bytes: &Vec<u8>) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();
        for byte in *bytes {
            chars.push(byte.clone() as char)
        }
        chars
    }

    fn get_addresses_fom_length(data: &Vec<u8>) -> Vec<String> {
        let mut addresses: Vec<String> = Vec::new();

        for i in 0..data.len() {
            if (i % 16) == 0 {
                addresses.push(format!("{:>0width$X}", i, width = 10))
            }
        }

        addresses
    }

    fn get_hex_data(data: &Vec<u8>) -> Vec<StatefulList<u8>> {
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
                0 => vec_0.push(*byte),
                1 => vec_1.push(*byte),
                2 => vec_2.push(*byte),
                3 => vec_3.push(*byte),
                4 => vec_4.push(*byte),
                5 => vec_5.push(*byte),
                6 => vec_6.push(*byte),
                7 => vec_7.push(*byte),
                8 => vec_8.push(*byte),
                9 => vec_9.push(*byte),
                10 => vec_10.push(*byte),
                11 => vec_11.push(*byte),
                12 => vec_12.push(*byte),
                13 => vec_13.push(*byte),
                14 => vec_14.push(*byte),
                15 => vec_15.push(*byte),
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

    fn get_ascii_data(data: &Vec<u8>) -> Vec<StatefulList<char>> {
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
                0 => vec_0.push(*byte as char),
                1 => vec_1.push(*byte as char),
                2 => vec_2.push(*byte as char),
                3 => vec_3.push(*byte as char),
                4 => vec_4.push(*byte as char),
                5 => vec_5.push(*byte as char),
                6 => vec_6.push(*byte as char),
                7 => vec_7.push(*byte as char),
                8 => vec_8.push(*byte as char),
                9 => vec_9.push(*byte as char),
                10 => vec_10.push(*byte as char),
                11 => vec_11.push(*byte as char),
                12 => vec_12.push(*byte as char),
                13 => vec_13.push(*byte as char),
                14 => vec_14.push(*byte as char),
                15 => vec_15.push(*byte as char),
                _ => (),
            }
            offset += 1;
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

    pub fn next_address(&mut self) {
        self.addresses.next();
        self.hex_view[self.horizontal_offset].next();
        self.ascii_view[self.horizontal_offset].next()
    }

    pub fn previous_address(&mut self) {
        self.addresses.previous();
        self.hex_view[self.horizontal_offset].previous();
        self.ascii_view[self.horizontal_offset].previous()
    }

    pub fn next_offset(&mut self) {
        // UNSELECT PREVIOUS
        self.hex_view[self.horizontal_offset].unselect();
        self.ascii_view[self.horizontal_offset].unselect();
        // GET VERTICAL OFFSET
        let v_offset = self.hex_view[self.horizontal_offset].selected_row;
        // INCREMENT OFFSET
        self.horizontal_offset += 1;
        if self.horizontal_offset == 16 {
            self.horizontal_offset = 0;
        }

        // SELECT RIGHT COLUMN
        self.hex_view[self.horizontal_offset].select(v_offset.clone());
        self.ascii_view[self.horizontal_offset].select(v_offset)
    }

    pub fn previous_offset(&mut self) {
        // UNSELECT PREVIOUS
        self.hex_view[self.horizontal_offset].unselect();
        self.ascii_view[self.horizontal_offset].unselect();
        // GET VERTICAL OFFSET
        let v_offset = self.hex_view[self.horizontal_offset].selected_row;
        // DECREMENT OFFSET
        if self.horizontal_offset == 0 {
            self.horizontal_offset = 16;
        }
        self.horizontal_offset -= 1;

        // SELECT RIGHT COLUMN
        self.hex_view[self.horizontal_offset].select(v_offset.clone());
        self.ascii_view[self.horizontal_offset].select(v_offset)
    }

    // INTERFACE
    pub fn get_adresses(&mut self) -> StatefulList<String> {
        self.addresses.clone()
    }

    pub fn get_hex_view(&mut self) -> Vec<StatefulList<u8>> {
        self.hex_view.clone()
    }

    pub fn get_ascii_view(&mut self) -> Vec<StatefulList<char>> {
        self.ascii_view.clone()
    }
}
