extern crate byteorder;
use byteorder::{ReadBytesExt, BigEndian};

use std::fs::File;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args[1].clone();
    println!("Attempting to read: {}", file_name);

    let file = File::open(file_name).unwrap();
    let length = file.metadata().unwrap().len() as usize;
    println!("Filesize: {} bytes", length);
    // Number of u16 values is length/2
    let mut buffer = vec![0u8; length/2];
    let mut buf_reader = BufReader::new(file);
    // CHIP-8 file is BigEndian u16 values
    buf_reader.read_u16_into::<BigEndian>(&mut buffer[..]).expect("Failed to read file!");
    println!("buffer: {:?}", buffer);
    for word in buffer {
        print!("{:x}, ", word);
    }

    Ok(())
}
