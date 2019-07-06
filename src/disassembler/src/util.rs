extern crate architecture;

use architecture::instruction::Instruction;

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
        counter += i.num_of_bytes();
    }
}