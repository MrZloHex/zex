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

    // INTERFACE
    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn get_chars(&self) -> Vec<char> {
        self.chars.clone()
    }

    pub fn get_length(&self) -> usize {
        self.length.clone()
    }
}


// pub fn next_address(&mut self) {
    //     self.addresses.next();
    //     self.hex_view[self.horizontal_offset].next();
    //     self.ascii_view[self.horizontal_offset].next()
    // }

    // pub fn previous_address(&mut self) {
    //     self.addresses.previous();
    //     self.hex_view[self.horizontal_offset].previous();
    //     self.ascii_view[self.horizontal_offset].previous()
    // }

    // pub fn next_offset(&mut self) {
    //     // UNSELECT PREVIOUS
    //     self.hex_view[self.horizontal_offset].unselect();
    //     self.ascii_view[self.horizontal_offset].unselect();
    //     // GET VERTICAL OFFSET
    //     let v_offset = self.hex_view[self.horizontal_offset].selected_row;
    //     // INCREMENT OFFSET
    //     self.horizontal_offset += 1;
    //     if self.horizontal_offset == 16 {
    //         self.horizontal_offset = 0;
    //     }

    //     // SELECT RIGHT COLUMN
    //     self.hex_view[self.horizontal_offset].select(v_offset.clone());
    //     self.ascii_view[self.horizontal_offset].select(v_offset)
    // }

    // pub fn previous_offset(&mut self) {
    //     // UNSELECT PREVIOUS
    //     self.hex_view[self.horizontal_offset].unselect();
    //     self.ascii_view[self.horizontal_offset].unselect();
    //     // GET VERTICAL OFFSET
    //     let v_offset = self.hex_view[self.horizontal_offset].selected_row;
    //     // DECREMENT OFFSET
    //     if self.horizontal_offset == 0 {
    //         self.horizontal_offset = 16;
    //     }
    //     self.horizontal_offset -= 1;

    //     // SELECT RIGHT COLUMN
    //     self.hex_view[self.horizontal_offset].select(v_offset.clone());
    //     self.ascii_view[self.horizontal_offset].select(v_offset)
    // }