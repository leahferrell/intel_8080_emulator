use crate::state::State;
use crate::instruction::Instruction;

pub fn load_register_pair(state: &mut State, instruction: &Instruction) -> bool {
    println!("loading register pair");
    state.pc += 1;
    false
}