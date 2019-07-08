use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Halt Instructions:
///
/// Instructions which operate directly upon the Interrupt Enable
/// flip-flop INTE.

pub fn halt(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}