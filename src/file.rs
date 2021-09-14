use std::fs;
use std::io::{Read, Write};

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
        for byte in bytes {
            chars.push((*byte).clone() as char)
        }
        chars
    }

    // INTERFACE
    pub fn get_filename(&mut self) -> String {
        self.filename.clone()
    }

    pub fn get_bytes(&mut self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn get_chars(&mut self) -> Vec<char> {
        self.chars.clone()
    }

    pub fn get_length(&mut self) -> usize {
        self.length.clone()
    }


    pub fn set_byte(&mut self, value: u8, address: usize) {
        self.bytes[address] = value;
    }

    pub fn set_char(&mut self, value: char, address: usize) {
        self.chars[address] = value;
    }


    pub fn write(&mut self) {
        let mut f = fs::File::open(&self.filename).expect("no file found");
        f.write(&(self.bytes.clone())[..]).unwrap();
    }
}
