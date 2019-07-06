use std::io;
use std::result::Result::Ok;

use emulator::cpu::CpuContext;
use disassembler::util;

fn main() -> io::Result<()> {
    let program = "resources/invaders.rom";

    util::disassemble(program)?;

    let mut cpu = CpuContext::load_program(program, 10000);
    cpu.run();

    Ok(())
}