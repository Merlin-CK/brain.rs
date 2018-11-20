use std::fs::File;
use std::env::args;
use std::io::*;

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
    
    let mut file = File::open(filename)?;

    let mut program = String::new();
    file.read_to_string(&mut program)?;
    
    Ok(program)
}