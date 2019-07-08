use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Register or Memory to Accumulator Instructions:
///
/// Instructions which operate on the accumulator using a byte
/// fetched from another register or memory.

pub fn add(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn add_with_carry(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn sub(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn sub_with_borrow(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn logical_and(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn logical_xor(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn logical_or(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn compare(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}