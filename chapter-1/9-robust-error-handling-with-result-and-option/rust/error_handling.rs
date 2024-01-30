use std::fs::File;
use std::io::{Read, Error};

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename = "content.txt";

    match read_file_contents(filename) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

// File contents:
// line 1
// line 2
// line 3
// 
