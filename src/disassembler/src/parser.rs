use std::fs::File;
use std::io::Read;
use std::io;

pub fn parse(filename: &str, mem_size: usize) -> io::Result<(Vec<u8>)> {
    let mut file=File::open(filename).unwrap();
    let mut memory = vec![0; mem_size];
    let mut program: Vec<u8> = vec![];

    file.read_to_end(&mut program)?;

    //TODO: find a cleaner way to do this
    for i in 0..program.len() {
        memory[i] = program[i];
    }

    Ok(memory)
}