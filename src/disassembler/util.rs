use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use crate::architecture::models::instruction::Instruction;

pub fn print_in_hex(buf: Vec<u8>, chunk_size: usize){
    let mut counter = 0;

    for bytes in buf.chunks(chunk_size) {
        print!("{:07x} ", counter);
        for byte in bytes {
            print!("{:02x} ", byte);
            counter += 1;
        }
        println!();
    }
}

pub fn print_instructions(instructions: Vec<Instruction>){
    let mut counter:usize = 0;

    for i in instructions {
        println!("{:04x} {}", counter, i.to_string());
        counter += i.size();
    }
}

pub fn write_instructions_to_file(instructions: Vec<Instruction>, filename: &str, with_address: bool) -> std::io::Result<()>{
    let file = File::create(filename)?;
    let mut file = LineWriter::new(file);

    let mut counter:usize = 0;

    for i in instructions {
        let line = if with_address {
            format!("{:04x} {}\n", counter, i.to_string())
        }else{
            format!("{}\n", i.to_string())
        };
        file.write_all(line.as_bytes())?;
        counter += i.size();
    }

    Ok(())
}