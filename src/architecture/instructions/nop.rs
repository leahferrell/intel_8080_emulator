use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

pub fn no_operation(state: &mut State, _instruction: &Instruction) -> usize {
    state.pc + 1
}