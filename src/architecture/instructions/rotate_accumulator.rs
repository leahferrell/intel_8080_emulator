use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

/// # Rotate Accumulator Instructions:
///
/// Instructions which rotate the contents of the accumulator.

pub fn rotate_left(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn rotate_right(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn rotate_left_through_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn rotate_right_through_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}