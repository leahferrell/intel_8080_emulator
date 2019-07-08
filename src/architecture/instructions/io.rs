use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::units::memory_unit;
use crate::architecture::model::registers::Register;
use crate::architecture::AddressPtr;

/// # Input/Output Instructions:
///
/// Instructions which cause data to be input to or output from the 8080.

pub fn input(state: &mut State, instruction: &Instruction) -> AddressPtr {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    state.pc
}

pub fn output(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &Register::A);

    println!("outputting the following data: {:02x}", value);

    state.pc + instruction.size()
}