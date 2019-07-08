use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;
use crate::architecture::units::stack;


/// # Register Pair Instructions:
///
/// Instructions which operate on pairs of registers

pub fn push(state: &mut State, instruction: &Instruction) -> bool {
    stack::push_reg(state, &instruction.register[0]);
    state.pc += instruction.num_of_bytes();
    false
}

pub fn pop(state: &mut State, instruction: &Instruction) -> bool {
    stack::pop_to_reg(state, &instruction.register[0]);
    state.pc += instruction.num_of_bytes();
    false
}

pub fn double_add(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn increment(state: &mut State, instruction: &Instruction) -> bool {
    let reg = &instruction.register[0];

    match reg {
        Register::B => {
            let regs = inc(state.b, state.c);
            state.b = regs.0;
            state.c = regs.1;
        },
        Register::D => {
            let regs = inc(state.d, state.e);
            state.d = regs.0;
            state.e = regs.1;
        },
        Register::H => {
            let regs = inc(state.h, state.l);
            state.h = regs.0;
            state.l = regs.1;
        },
        Register::SP => {
            state.sp += 1;
        },
        _ => ()
    };

    state.pc += instruction.num_of_bytes();

    false
}

pub fn decrement(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn exchange(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn exchange_sp(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn load_sp_from_hl(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

fn inc(left: u8, right: u8) -> (u8,u8) {
    if right < 0xff {
        (left, right+1)
    }else{
        (left+1, right)
    }
}