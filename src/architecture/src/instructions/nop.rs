use crate::state::State;
use crate::instruction::Instruction;

pub fn no_operation(state: &mut State, _instruction: &Instruction) -> bool {
    state.pc += 1;
    false
}