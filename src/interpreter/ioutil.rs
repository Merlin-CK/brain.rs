use std::io::{stdout, stdin, Write, Read};


/// Reads a single byte from stdin.
/// 
/// # Panics
/// Panics if the byte could not be read.
pub fn read_byte() -> u8 {
    stdin()
        .bytes()
        .next()
        .expect("Stdin reached end of file")
        .expect("Can't read byte from stdin")
}

/// Writes a single byte to stdout and flushes the output stream.
/// 
/// # Panics
/// Panics if writing a byte to stdout or flushing the stream failed.
pub fn write_byte(b: u8) {
    let buffer = [b];
    
    stdout()
        .write(&buffer)
        .expect("Can't write to stdout");

    stdout()
        .flush()
        .expect("Can't flush stdout");
}