use intel_8080_emulator::emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new("resources/invaders.rom");
    emulator.run();
}
