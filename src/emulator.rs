use crate::disassembler;
use crate::architecture::cpu_context::CpuContext;
use crate::ui::window;

pub struct Emulator {
    rom: String,
    cpu: CpuContext
}

impl Emulator {
    pub fn new(rom: &str) -> Emulator {
        disassembler::disassemble(rom, false).expect(format!("failed to load file {}", rom).as_str());
        let mut cpu = CpuContext::load_program(rom, 65536);

        Emulator {
            rom: rom.to_owned(),
            cpu
        }
    }

    pub fn run(&mut self){
        //self.cpu.run();

        window::load_window();
    }
}