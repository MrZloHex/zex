use std::fs;
use std::io::{self, Read};

use super::stateful_widgets::{StatefulList, StatefulTable};

pub struct File {
    filename: String,
    data: Vec<u8>,
    length: usize,

    // TUI
    pub addresses: StatefulList<String>,
    pub hex_view: Vec<StatefulList<String>>
}

impl File {
    pub fn new(filename: &str) -> File {
        let data = File::get_file_data(filename);
        let addresses:Vec<String> = File::get_addresses_fom_length(&data);
        let hex = File::get_hex_data(&data);
        File {
            filename: filename.to_string(),
            length: data.len(),
            data,

            addresses: StatefulList::new(addresses),
            hex_view: hex
        }
    }

    fn get_file_data(filename: &str) -> Vec<u8> {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
    
        buffer
    }

    fn get_addresses_fom_length(data: &Vec<u8>) -> Vec<String> {
        let mut addresses: Vec<String> = Vec::new();

        for i in 0..data.len() {
            if (i % 16) == 0 {
                addresses.push(format!("{:>0width$X}", i, width=10))
            }
        }

        addresses

    }

    fn get_hex_data(data: &Vec<u8>) -> Vec<StatefulList<String>> {
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
        let mut index: usize = 0;

        for byte in data {
            if offset == 16 {
                offset = 0;
            }
            match offset {
                0 => vec_0.push(format!("{:>0width$X}", byte, width=2)),
                1 => vec_1.push(format!("{:>0width$X}", byte, width=2)),
                2 => vec_2.push(format!("{:>0width$X}", byte, width=2)),
                3 => vec_3.push(format!("{:>0width$X}", byte, width=2)),
                4 => vec_4.push(format!("{:>0width$X}", byte, width=2)),
                5 => vec_5.push(format!("{:>0width$X}", byte, width=2)),
                6 => vec_6.push(format!("{:>0width$X}", byte, width=2)),
                7 => vec_7.push(format!("{:>0width$X}", byte, width=2)),
                8 => vec_8.push(format!("{:>0width$X}", byte, width=2)),
                9 => vec_9.push(format!("{:>0width$X}", byte, width=2)),
                10 => vec_10.push(format!("{:>0width$X}", byte, width=2)),
                11 => vec_11.push(format!("{:>0width$X}", byte, width=2)),
                12 => vec_12.push(format!("{:>0width$X}", byte, width=2)),
                13 => vec_13.push(format!("{:>0width$X}", byte, width=2)),
                14 => vec_14.push(format!("{:>0width$X}", byte, width=2)),
                15 => vec_15.push(format!("{:>0width$X}", byte, width=2)),
                _ => ()
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





    pub fn next_address(&mut self) {
        self.addresses.next()
    }

    pub fn previous_address(&mut self) {
        self.addresses.previous()
    }





    // INTERFACE
    pub fn get_adresses(&mut self) -> StatefulList<String> {
        self.addresses.clone()
    }

    pub fn get_hex_view(&mut self) -> Vec<StatefulList<String>> {
        self.hex_view.clone()
    }
}
