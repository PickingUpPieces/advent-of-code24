use std::io::{self, BufRead};
use std::fs::File;

pub fn init() {
    // Initialization code here
    env_logger::init();
}

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    // Check if file exists
    let file = File::open(file_path)?;
    let buffer = io::BufReader::new(file);

    // Read file line by line
    let content: Vec<String> = buffer
    .lines()
    .collect::<Result<_,_>>()?;
    
    return Ok(content) 
}