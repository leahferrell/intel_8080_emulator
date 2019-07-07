use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;


/// # Call Instructions:
///
/// Instructions which call subroutines.
/// These instructions operate like the jump instructions, causing
/// a transfer of program control. In addition, a return address
/// is pushed onto the stack for use by the RETURN instructions.

pub fn default(state: &mut State, instruction: &Instruction) -> bool {
    let ret:u16 = (state.pc + instruction.num_of_bytes()) as u16;

    println!("Setting return address: {:04x}", ret);

    state.push_return_addr(ret);
    state.pc = instruction.get_addr() as usize;
    false
}

fn condition(state: &mut State, instruction: &Instruction, expected_condition: bool) -> bool {
    if expected_condition {
        default(state, instruction);
    }else{
        state.pc += 1;
    }
    false
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