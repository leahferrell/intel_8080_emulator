use crate::architecture::model::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::model::instruction::Instruction;

/// # Direct Addressing Instructions:
///
/// Instructions which reference memory by a two-byte address
/// which is part of the instruction itself.

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> usize {
    memory_unit::set_mem_value(state, (instruction.operands[0], instruction.operands[1]), state.a);
    state.pc + instruction.size()
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn store_hl(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn load_hl(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}