use crate::architecture::models::state::State;
use crate::architecture::models::instruction::Instruction;
use crate::architecture::units::memory_unit;
use crate::architecture::models::registers::Register;
use crate::architecture::AddressPtr;
use crate::architecture::IoSignal;
use crate::architecture::units::io_unit;

/// # Input/Output Instructions:
///
/// Instructions which cause data to be input to or output from the 8080.

pub fn input(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let handler = io_unit::get_input_handler(instruction.operands[0]);

    let signal = handler.next_signal(state).signal;

    memory_unit::set_reg_value(state, &Register::A, signal);

    state.pc + instruction.size()
}

pub fn output(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let signal = memory_unit::get_reg_value(state, &Register::A);
    let device = instruction.operands[0];

    state.output_queue.push(IoSignal {signal, device});
    state.pc + instruction.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::models::opcodes::OpCode;

    #[test]
    fn test_input() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::IN,operands: vec![0x10],..Default::default()};
        let result = input(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.a, 0x1c);
    }

    #[test]
    fn test_output() {
        let mut state = State {a: 0x1c,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::OUT,operands: vec![0x10],..Default::default()};
        let result = output(&mut state, instruction);
        assert_eq!(result, 2);
        assert_eq!(state.output_queue.pop().unwrap().signal, 0x1c)
    }
}