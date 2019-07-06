use crate::state::State;
use crate::instruction::Instruction;


/// # Return Instructions:
///
/// Instructions used to return from subroutines.

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

fn condition(state: &mut State, instruction: &Instruction, value: u8, expected: u8) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.cy, 1)
}

pub fn no_carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.cy, 0)
}

pub fn zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.z, 1)
}

pub fn no_zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.z, 0)
}

pub fn minus(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.s, 1)
}

pub fn positive(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.s, 0)
}

pub fn parity_even(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.p, 1)
}

pub fn parity_odd(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.p, 0)
}