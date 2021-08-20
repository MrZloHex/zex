use std::fs;
use std::io::{self, Read};



pub struct File {
    filename: String,
    data: Vec<u8>,
    length: usize,

    // TUI
    // addresses: StatefulList<usize>
}

impl File {
    pub fn new(filename: &str) -> Self {
        let data = File::get_file_data(filename);
        let addresses = File::get_addresses_fom_length(&data);
        File {
            filename: filename.to_string(),
            length: data.len(),
            data,

            // addresses: StatefulList::new()
        }
    }

    fn get_file_data(filename: &str) -> Vec<u8> {
        let mut f = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
    
        buffer
    }

    fn get_addresses_fom_length(data: &Vec<u8>) /*-> Vec<usize>*/ {
        let mut addresses: Vec<usize> = Vec::new();
        let data_length = (*data).len();

        for i in 0..data_length {
            if (i % 16) == 0 {
                addresses.push(i);
            }
        }

        println!("{:X}", data_length);
        for address in addresses {
            print!("{:X}  ", address);
        }
    }
}






pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    // pub fn next(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i >= self.items.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    // pub fn previous(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.items.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}



#[derive(Debug, Clone)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl Default for ListState {
    fn default() -> ListState {
        ListState {
            offset: 0,
            selected: None,
        }
    }
}

impl ListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}