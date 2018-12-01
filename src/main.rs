use std::env::args;
use std::io::{Result, Error, ErrorKind};

mod interpreter;


fn main() {
    match read_file() {
        Ok(program) => interpreter::run(&program),
        Err(e)      => println!("{}", e)
    }
}

fn read_file() -> Result<String> {
    let filename = args()
        .nth(1)
        .ok_or(Error::new(ErrorKind::Other, "No file specified"))?;
    
    let content = std::fs::read_to_string(filename)?;

    Ok(content)
}