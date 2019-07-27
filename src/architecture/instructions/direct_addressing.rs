use crate::architecture::models::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::models::instruction::Instruction;
use crate::architecture::AddressPtr;
use crate::architecture::models::registers::Register;
use crate::architecture::units::arithmetic_logic_unit;

/// # Direct Addressing Instructions:
///
/// Instructions which reference memory by a two-byte address
/// which is part of the instruction itself.

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    memory_unit::set_mem_value(state, (instruction.operands[0], instruction.operands[1]), state.a);
    state.pc + instruction.size()
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let data = memory_unit::get_mem_value(state, (instruction.operands[0], instruction.operands[1]));
    memory_unit::set_reg_value(state, &Register::A, data);
    state.pc + instruction.size()
}

pub fn store_hl(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let addr1 = (instruction.operands[0],instruction.operands[1]);
    let addr2 = arithmetic_logic_unit::double_add(addr1, (0x00, 0x01)).0;

    let reg_value = memory_unit::get_double_reg_value(state, &Register::H);

    memory_unit::set_mem_value(state, addr1, reg_value.1);
    memory_unit::set_mem_value(state, addr2, reg_value.0);

    state.pc + instruction.size()
}

pub fn load_hl(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let addr1 = (instruction.operands[0],instruction.operands[1]);
    let addr2 = arithmetic_logic_unit::double_add(addr1, (0x00, 0x01)).0;

    let addr1_value = memory_unit::get_mem_value(state, addr1);
    let addr2_value = memory_unit::get_mem_value(state, addr2);

    memory_unit::set_double_reg_value(state, &Register::H, (addr2_value, addr1_value));

    state.pc + instruction.size()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::models::opcodes::OpCode;

    #[test]
    fn test_store_accumulator() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        let mut state = State {a: value, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STA,operands:vec![0x00, 0x20],..Default::default()};

        let result = store_accumulator(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
    }

    #[test]
    fn test_load_accumulator() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        memory[index] = value;
        let mut state = State {memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LDA,operands:vec![0x00, 0x20],..Default::default()};

        let result = load_accumulator(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
    }

    #[test]
    fn test_store_hl() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value1:u8 = 0x1c;
        let value2:u8 = 0x11;
        let mut state = State {h: value2, l: value1, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::SHLD,operands:vec![0x00, 0x20],..Default::default()};

        let result = store_hl(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.memory[index], value1);
        assert_eq!(state.memory[index+1], value2);
    }

    #[test]
    fn test_load_hl() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value1:u8 = 0x1c;
        let value2:u8 = 0x11;

        memory[index] = value1;
        memory[index + 1] = value2;

        let mut state = State {memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LHLD,operands:vec![0x00, 0x20],..Default::default()};

        let result = load_hl(&mut state, instruction);
        assert_eq!(result, 3);
        assert_eq!(state.memory[index], value1);
        assert_eq!(state.memory[index+1], value2);
        assert_eq!(state.l, value1);
        assert_eq!(state.h, value2);
    }
}