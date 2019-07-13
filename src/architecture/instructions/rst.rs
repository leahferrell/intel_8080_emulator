use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;
use crate::architecture::units::stack;

/// # Reset Instructions:
///
/// The restart instruction is a special purpose subroutine jump.

//TODO: unit tests
pub fn restart(state: &mut State, instruction: &Instruction) -> AddressPtr {
    stack::push_return_addr(state, state.pc as u16);
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    0x18
}