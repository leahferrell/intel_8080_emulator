use crate::architecture::models::instruction::Instruction;
use crate::architecture::instruction_set;

pub fn convert_to_assembly(buf: Vec<u8>) -> Vec<Instruction> {
    let mut ptr:usize = 0;
    buf.len();

    let mut instructions: Vec<Instruction> = Vec::new();

    while ptr < buf.len() {
        let instr = instruction_set::read_next(&buf, ptr);
        ptr += instr.size();
        instructions.push(instr);
    }
    return instructions;
}