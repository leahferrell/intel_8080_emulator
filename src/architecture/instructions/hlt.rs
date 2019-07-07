use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;


/// # Halt Instructions:
///
/// Instructions which operate directly upon the Interrupt Enable
/// flip-flop INTE.

pub fn halt(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}