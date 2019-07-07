use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;


/// # Return Instructions:
///
/// Instructions used to return from subroutines.

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

fn condition(state: &mut State, instruction: &Instruction, expected_condition: bool) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.cy)
}

pub fn no_carry(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.cy)
}

pub fn zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.z)
}

pub fn no_zero(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.z)
}

pub fn minus(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.s)
}

pub fn positive(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.s)
}

pub fn parity_even(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, state.cc.p)
}

pub fn parity_odd(state: &mut State, instruction: &Instruction) -> bool {
    condition(state, instruction, !state.cc.p)
}