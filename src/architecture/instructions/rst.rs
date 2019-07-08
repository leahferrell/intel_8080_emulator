use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Reset Instructions:
///
/// The restart instruction is a special purpose subroutine jump.

pub fn restart(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}