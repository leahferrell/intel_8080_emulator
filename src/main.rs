use std::io;
use std::result::Result::Ok;

use intel_8080_emulator::architecture::cpu_context::CpuContext;
use intel_8080_emulator::disassembler;

fn main() -> io::Result<()> {
    let program = "resources/invaders.rom";

    disassembler::disassemble(program)?;

    let mut cpu = CpuContext::load_program(program, 0xffff);
    cpu.run();

    Ok(())
}
