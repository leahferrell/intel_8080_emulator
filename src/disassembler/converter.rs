
use crate::architecture::isa;
use crate::architecture::instruction::Instruction;

pub fn convert_to_assembly(buf: Vec<u8>) -> Vec<Instruction> {
    let mut ptr:usize = 0;
    buf.len();

    let mut instructions: Vec<Instruction> = Vec::new();

    while ptr < buf.len() {
        let instr = isa::read_next(&buf, ptr);
        ptr += instr.num_of_bytes();
        instructions.push(instr);
    }
    return instructions;
}