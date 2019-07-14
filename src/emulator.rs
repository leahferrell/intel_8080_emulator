use crate::disassembler;
use crate::architecture::cpu_context::CpuContext;
use crate::ui::window::Window;
use std::thread;
use std::sync::mpsc::channel;

use crate::actor::cpu_actor::CpuActor;
use crate::actor::ui_actor::UiActor;

pub struct Emulator {
    rom: String,
}

impl Emulator {
    pub fn new(rom: &str) -> Emulator {
        disassembler::disassemble(&rom, false).expect(format!("failed to load file {}", rom).as_str());

        Emulator {
            rom: rom.to_owned(),
        }
    }

    pub fn run(&mut self){
        let (exit_sender, exit_receiver) = channel();
        let (input_sender, input_receiver) = channel();
        let (display_sender, display_receiver) = channel();
        let (audio_sender, audio_receiver) = channel();

        let cpu_actor = CpuActor::new(exit_receiver, input_receiver, display_sender, audio_sender);
        let ui_actor = UiActor::new(exit_sender, input_sender, display_receiver, audio_receiver);

        let mut cpu = CpuContext::load_program(&self.rom, 65536, cpu_actor);

        let handle = thread::spawn(move || {
            cpu.run();
        });

        let mut window = Window::new(ui_actor);
        window.render();

        handle.join().unwrap();
    }
}