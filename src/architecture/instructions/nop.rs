use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

pub fn no_operation(state: &mut State, _instruction: &Instruction) -> AddressPtr {
    state.pc + 1
}