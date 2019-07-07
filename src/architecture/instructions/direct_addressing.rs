use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;
use crate::architecture::units::memory_unit;


/// # Direct Addressing Instructions:
///
/// Instructions which reference memory by a two-byte address
/// which is part of the instruction itself.

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    memory_unit::set_mem_value(state, (instruction.operands[0], instruction.operands[1]), state.a);
    state.pc += instruction.num_of_bytes();
    false
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn store_hl(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn load_hl(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}