use crate::architecture::model::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::units::arithmetic_logic_unit;
use crate::architecture::AddressPtr;

/// # Single Register Instructions:
///
/// Instructions which operate on a single register or memory location.

pub fn increment(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

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

pub fn complement_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn decimal_adjust_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}