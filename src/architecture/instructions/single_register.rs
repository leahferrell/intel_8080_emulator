use crate::architecture::models::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::models::instruction::Instruction;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::AddressPtr;

/// # Single Register Instructions:
///
/// Instructions which operate on a single register or memory location.

//TODO: unit tests
pub fn increment(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::add(value, 1);

    memory_unit::set_reg_value(state, &instruction.register[0], result.0);

    state.cc.z = result.1.z;
    state.cc.s = result.1.s;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn decrement(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &instruction.register[0]);

    let result = arithmetic_logic_unit::sub(value, 1);

    memory_unit::set_reg_value(state, &instruction.register[0], result.0);

    state.cc.z = result.1.z;
    state.cc.s = result.1.s;
    state.cc.p = result.1.p;
    state.cc.ac = result.1.ac;

    state.pc + instruction.size()
}

//TODO: unit tests
pub fn complement_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &instruction.register[0]);
    let result = arithmetic_logic_unit::ones_complement(value);
    memory_unit::set_reg_value(state, &instruction.register[0], result);
    state.pc + instruction.size()
}

//TODO: unit tests & handle edge cases for carry bits
pub fn decimal_adjust_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let inc_amount = if state.a & 0xf0 > 9 || state.cc.ac {
        0x06
    }else{
        0x00
    } +
    if state.a >> 4 > 9 || state.cc.cy {
       0x60
    }else {
        0x00
    };

    let result = arithmetic_logic_unit::add(state.a, inc_amount);
    state.a = result.0;
    state.cc = result.1;

    state.pc + instruction.size()
}