use crate::opcodes::OpCode;
use crate::state::State;
use crate::instruction::Instruction;

pub fn no_operation(state: &State, instruction: &Instruction) -> bool {
    println!("no operation....");
    false
}