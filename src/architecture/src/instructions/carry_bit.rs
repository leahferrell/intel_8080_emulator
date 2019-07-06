use crate::state::State;
use crate::instruction::Instruction;
use crate::registers::Register;


/// # Carry Bit Instructions:
///
/// Instructions which operate directly on the carry bit.

pub fn complement(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn set(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}