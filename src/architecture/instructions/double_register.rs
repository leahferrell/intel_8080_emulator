use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::units::stack;
use crate::architecture::units::memory_unit;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::model::registers::Register;
use crate::architecture::AddressPtr;

/// # Register Pair Instructions:
///
/// Instructions which operate on pairs of registers

pub fn push(state: &mut State, instruction: &Instruction) -> AddressPtr {
    stack::push_reg(state, &instruction.register[0]);
    state.pc + instruction.size()
}

pub fn pop(state: &mut State, instruction: &Instruction) -> AddressPtr {
    stack::pop_to_reg(state, &instruction.register[0]);
    state.pc + instruction.size()
}

pub fn double_add(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg_pair_val = memory_unit::get_double_reg_value(state, &instruction.register[0]);
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    let result = arithmetic_logic_unit::double_add(reg_pair_val, hl_pair_val);

    state.cc.cy = result.1.cy;

    memory_unit::set_double_reg_value(state, &Register::H, result.0);

    state.pc + instruction.size()
}

pub fn increment(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = &instruction.register[0];

    let reg_pair = memory_unit::get_double_reg_value(state, reg);
    let result = arithmetic_logic_unit::double_add(reg_pair, (0,1));
    memory_unit::set_double_reg_value(state, reg, result.0);
    state.pc + instruction.size()
}

pub fn decrement(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn exchange(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    let de_pair_val = memory_unit::get_double_reg_value(state, &Register::D);

    memory_unit::set_double_reg_value(state, &Register::H, de_pair_val);
    memory_unit::set_double_reg_value(state, &Register::D, hl_pair_val);

    state.pc + instruction.size()
}

pub fn exchange_sp(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let sp1_loc = memory_unit::get_double_reg_value(state, &Register::SP);
    let sp2_loc = arithmetic_logic_unit::double_add(sp1_loc, (0,1)).0;

    let sp1_value = memory_unit::get_mem_value(state, sp1_loc);
    let sp2_value = memory_unit::get_mem_value(state, sp2_loc);

    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);

    memory_unit::set_double_reg_value(state, &Register::H, (sp2_value, sp1_value));
    memory_unit::set_mem_value(state, sp1_loc, hl_pair_val.1);
    memory_unit::set_mem_value(state, sp2_loc, hl_pair_val.0);

    state.pc + instruction.size()
}

pub fn load_sp_from_hl(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let hl_pair_val = memory_unit::get_double_reg_value(state, &Register::H);
    memory_unit::set_double_reg_value(state, &Register::SP, hl_pair_val);
    state.pc + instruction.size()
}