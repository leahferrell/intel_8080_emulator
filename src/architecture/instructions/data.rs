use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

/// # Data Instructions:
///
/// Instructions which specify data

pub fn define_bytes(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn define_word(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn define_storage(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}