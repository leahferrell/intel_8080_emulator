use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

/// # Interrupt Instructions:
///
/// Instructions which operate directly upon the Interrupt Enable
/// flip-flop INTE.

pub fn enable(state: &mut State, instruction: &Instruction) -> AddressPtr {
    state.int_enable = true;
    state.pc + instruction.size()
}

pub fn disable(state: &mut State, instruction: &Instruction) -> AddressPtr {
    state.int_enable = false;
    state.pc + instruction.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::model::opcodes::OpCode;

    #[test]
    fn test_enable() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::EI,..Default::default()};
        let result = enable(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.int_enable, true);
    }

    #[test]
    fn test_disable() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::DI,..Default::default()};
        let result = disable(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.int_enable, false);
    }
}