use std::io::{stdout, stdin, Write, Read};


pub fn read_byte() -> u8 {
    let mut buffer = [0u8];

    stdin()
        .read(&mut buffer)
        .expect("Can't read from stdin");

    buffer[0]
}

pub fn write_byte(b: u8) {
    let buffer = [b];
    
    stdout()
        .write(&buffer)
        .expect("Can't write to stdout");

    stdout()
        .flush()
        .expect("Can't flush stdout");
}