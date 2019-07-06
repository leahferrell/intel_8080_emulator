use crate::state::State;
use crate::instruction::Instruction;

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    state.pc = instruction.get_addr() as usize;
    false
}

fn condition(state: &mut State, instruction: &Instruction, value: u8, expected: u8) -> bool {
    if value == expected {
        state.pc = instruction.get_addr() as usize;
    }else{
        state.pc += 1;
    }
    false
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