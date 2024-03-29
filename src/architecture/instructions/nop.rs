use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

pub fn no_operation(state: &mut State, _instruction: &Instruction) -> AddressPtr {
    state.pc + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::model::opcodes::OpCode;

    #[test]
    fn test_nop() {
        let mut state = State {..Default::default()};
        let instruction = &Instruction{opcode:OpCode::NOP,..Default::default()};
        let result = no_operation(&mut state, instruction);
        assert_eq!(result, 1);
    }
}