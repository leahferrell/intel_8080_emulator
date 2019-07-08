use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Reset Instructions:
///
/// The restart instruction is a special purpose subroutine jump.

pub fn restart(state: &mut State, instruction: &Instruction) -> usize {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}