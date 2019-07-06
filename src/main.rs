extern crate disassembler;

use std::io;
use std::result::Result::Ok;

use disassembler::{parser,converter, util};
use emulator::cpu;
use architecture::state::State;

fn main() -> io::Result<()> {
    let program_in_bytes = parser::parse("resources/invaders.rom").unwrap();

    //util::print_in_hex(program_in_bytes.clone(), 16);

    let instructions = converter::convert_to_assembly(program_in_bytes.clone());

    //util::print_instructions(instructions);

    let state = &mut State{
        memory: program_in_bytes,
        ..Default::default()
    };

    cpu::emulate(state);

    Ok(())
}