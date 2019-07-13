use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;
use crate::architecture::units::memory_unit;
use crate::architecture::units::arithmetic_logic_unit;

/// # Register or Memory to Accumulator Instructions:
///
/// Instructions which operate on the accumulator using a byte
/// fetched from another register or memory.

//TODO: unit tests
pub fn add(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = memory_unit::get_reg_value(state, &instruction.register[0]);
    let acc = state.a;
    let result = arithmetic_logic_unit::add(acc, reg);
    state.a = result.0;
    state.cc = result.1;
    state.pc + instruction.size()
}

//TODO: unit tests
pub fn add_with_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = memory_unit::get_reg_value(state, &instruction.register[0]);
    let acc = state.a;
    let result = arithmetic_logic_unit::add(acc, reg);
    let result = if state.cc.cy {arithmetic_logic_unit::add(result.0, 1)} else {result};

    state.a = result.0;
    state.cc = result.1;
    state.pc + instruction.size()
}

//TODO: unit tests
pub fn sub(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = memory_unit::get_reg_value(state, &instruction.register[0]);
    let acc = state.a;
    let result = arithmetic_logic_unit::sub(acc, reg);
    state.a = result.0;
    state.cc = result.1;
    state.pc + instruction.size()
}

//TODO: unit tests
pub fn sub_with_borrow(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let reg = memory_unit::get_reg_value(state, &instruction.register[0]);
    let acc = state.a;
    let result = arithmetic_logic_unit::sub(acc, reg);
    let result = if state.cc.cy {arithmetic_logic_unit::sub(result.0, 1)} else {result};

    state.a = result.0;
    state.cc = result.1;
    state.pc + instruction.size()
}

//TODO: unit tests
pub fn logical_and(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let reg_value = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::and(acc_value, reg_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn logical_xor(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let reg_value = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::xor(acc_value, reg_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn logical_or(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc_value = state.a;
    let reg_value = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::or(acc_value, reg_value);

    state.cc.cy = result.1.cy;
    state.cc.s = result.1.s;
    state.cc.z = result.1.z;
    state.cc.p = result.1.p;

    state.a = result.0;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn compare(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let acc = state.a;
    let reg = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::compare(acc, reg);

    state.cc = result.1;

    state.pc + instruction.size()
}