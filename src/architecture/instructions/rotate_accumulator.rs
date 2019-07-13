use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::AddressPtr;

/// # Rotate Accumulator Instructions:
///
/// Instructions which rotate the contents of the accumulator.

pub fn rotate_left(state: &mut State, instruction: &Instruction) -> AddressPtr {
    state.cc.cy = state.a > 127;
    state.a = state.a.rotate_left(1);
    state.pc + instruction.size()
}

pub fn rotate_right(state: &mut State, instruction: &Instruction) -> AddressPtr {
    state.cc.cy = state.a % 2 == 1;
    state.a = state.a.rotate_right(1);
    state.pc + instruction.size()
}

pub fn rotate_left_through_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let former_carry = if state.cc.cy {1} else {0};
    state.cc.cy = state.a > 127;

    state.a = (state.a << 1) + former_carry;
    state.pc + instruction.size()
}

pub fn rotate_right_through_carry(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let former_carry = if state.cc.cy {128} else {0};
    state.cc.cy = state.a % 2 == 1;

    state.a = (state.a >> 1) + former_carry;
    state.pc + instruction.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::model::opcodes::OpCode;

    #[test]
    fn test_rotate_left() {
        let mut state = State {a:0x16,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_left(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, false);
        assert_eq!(state.a, 0x2c);
    }

    #[test]
    fn test_rotate_right() {
        let mut state = State {a:0xf2,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_right(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, false);
        assert_eq!(state.a, 0x79);
    }

    #[test]
    fn test_rotate_left_w_carry() {
        let mut state = State {a:0xf2,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_left(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
        assert_eq!(state.a, 0xe5);
    }

    #[test]
    fn test_rotate_right_w_carry() {
        let mut state = State {a:0x79,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_right(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
        assert_eq!(state.a, 0xbc);
    }

    #[test]
    fn test_rotate_left_through_carry() {
        let mut state = State {a:0xb5,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_left_through_carry(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, true);
        assert_eq!(state.a, 0x6a);
    }

    #[test]
    fn test_rotate_right_through_carry() {
        let mut state = State {a:0x6a,..Default::default()};
        state.cc.cy = true;

        let instruction = &Instruction{opcode:OpCode::RLC,..Default::default()};

        let result = rotate_right_through_carry(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.cc.cy, false);
        assert_eq!(state.a, 0xb5);
    }
}