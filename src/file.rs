use std::fs;
use std::io::{self, Read};

use super::list::StatefulList;

pub struct File {
    filename: String,
    data: Vec<u8>,
    length: usize,

    // TUI
    addresses: StatefulList<String>
}

impl File {
    pub fn new(filename: &str) -> File {
        let data = File::get_file_data(filename);
        let addresses:Vec<String> = File::get_addresses_fom_length(data.clone());
        File {
            filename: filename.to_string(),
            length: data.len(),
            data,

            addresses: StatefulList::new(addresses)
        }
    }

    fn get_file_data(filename: &str) -> Vec<u8> {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
    
        buffer
    }

    fn get_addresses_fom_length(data: Vec<u8>) -> Vec<String> {
        let mut addresses: Vec<String> = Vec::new();
        let data_length = data.len();

        for i in 0..data_length {
            if (i % 16) == 0 {
                addresses.push(format!("{:>0width$X}", i, width=10))
            }
        }

        addresses

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
}
