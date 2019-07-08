use crate::architecture::state::State;
use crate::architecture::instruction::Instruction;
use crate::architecture::registers::Register;
use crate::architecture::units::stack;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::units::memory_unit;


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
    let reg_pair_val = memory_unit::get_double_reg_value(state, &instruction.register[0]);
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    let result = arithmetic_logic_unit::double_add(reg_pair_val, hl_pair_val);

    state.cc.cy = result.1.cy;

    memory_unit::set_double_reg_value(state, &Register::H, result.0);

    state.pc += instruction.num_of_bytes();

    false
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
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    let de_pair_val = memory_unit::get_double_reg_value(state, &Register::D);

    memory_unit::set_double_reg_value(state, &Register::H, de_pair_val);
    memory_unit::set_double_reg_value(state, &Register::D, hl_pair_val);

    state.pc += instruction.num_of_bytes();

    false
}

pub fn exchange_sp(state: &mut State, instruction: &Instruction) -> bool {
    let sp1_loc = memory_unit::get_double_reg_value(state, &Register::SP);
    let sp2_loc = arithmetic_logic_unit::double_add(sp1_loc, (0,1)).0;

    let sp1_value = memory_unit::get_mem_value(state, sp1_loc);
    let sp2_value = memory_unit::get_mem_value(state, sp2_loc);

    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);

    memory_unit::set_double_reg_value(state, &Register::H, (sp2_value, sp1_value));
    memory_unit::set_mem_value(state, sp1_loc, hl_pair_val.1);
    memory_unit::set_mem_value(state, sp2_loc, hl_pair_val.0);

    state.pc += instruction.num_of_bytes();

    false
}

pub fn load_sp_from_hl(state: &mut State, instruction: &Instruction) -> bool {
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    let sp_pair_val = memory_unit::get_double_reg_value(state, &Register::SP);

    memory_unit::set_double_reg_value(state, &Register::SP, hl_pair_val);

    state.pc += instruction.num_of_bytes();

    false
}

fn inc(left: u8, right: u8) -> (u8,u8) {
    if right < 0xff {
        (left, right+1)
    }else{
        (left+1, right)
    }
}