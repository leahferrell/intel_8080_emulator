extern crate disassembler;

use std::io;
use std::result::Result::Ok;

use emulator::cpu::CpuContext;

fn main() -> io::Result<()> {
    let mut cpu = CpuContext::load_program("resources/invaders.rom");
    cpu.run();
    Ok(())
}