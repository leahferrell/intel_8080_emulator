use crate::architecture::model::state::State;
use crate::architecture::units::memory_unit;
use crate::architecture::model::instruction::Instruction;
use crate::architecture::model::registers::Register;
use crate::architecture::AddressPtr;

/// # Data Instructions:
///
/// Instructions which transfer data between registers or between
/// memory and registers.

pub fn move_reg(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let value = memory_unit::get_reg_value(state, &instruction.register[1]);

    memory_unit::set_reg_value(state, &instruction.register[0], value);

    state.pc + instruction.size()
}

pub fn load_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let data = memory_unit::get_reg_mem_value(state, &instruction.register[0]);

    memory_unit::set_reg_value(state, &Register::A, data);

    state.pc + instruction.size()
}

pub fn store_accumulator(state: &mut State, instruction: &Instruction) -> AddressPtr {
    let data = memory_unit::get_reg_value(state, &Register::A);

    memory_unit::set_reg_mem_value(state, &instruction.register[0], data);

    state.pc + instruction.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::model::condition_codes::ConditionCodes;
    use crate::architecture::model::opcodes::OpCode;

    #[test]
    fn test_move_reg_to_reg() {
        let mut state = State {a:10,b:7,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::MOV,register:vec![Register::A, Register::B],..Default::default()};

        assert_eq!(state.a, 10);
        assert_eq!(state.b, 7);

        let result = move_reg(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.a, 7);
        assert_eq!(state.b, 7);
    }

    #[test]
    fn test_move_reg_to_mem() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        let mut state = State {b:value, h:0x00, l:0x20,memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::MOV,register:vec![Register::M, Register::B],..Default::default()};

        let result = move_reg(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.b, value);
    }

    #[test]
    fn test_move_mem_to_reg() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        memory[index] = value;
        let mut state = State {b:0x00, h:0x00, l:0x20,memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::MOV,register:vec![Register::B, Register::M],..Default::default()};

        let result = move_reg(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.b, value);
    }

    #[test]
    fn test_load_acc_from_bc() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        memory[index] = value;
        let mut state = State {b:0x00, c:0x20, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LDAX,register:vec![Register::B],..Default::default()};

        let result = load_accumulator(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
        assert_eq!(state.b, 0x00);
        assert_eq!(state.c, 0x20);
    }

    #[test]
    fn test_load_acc_from_de() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        memory[index] = value;
        let mut state = State {d:0x00, e:0x20, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::LDAX,register:vec![Register::D],..Default::default()};

        let result = load_accumulator(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0x20);
    }

    #[test]
    fn test_store_acc_to_bc() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        let mut state = State {a: value, b:0x00, c:0x20, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STAX,register:vec![Register::B],..Default::default()};

        let result = store_accumulator(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
        assert_eq!(state.b, 0x00);
        assert_eq!(state.c, 0x20);
    }

    #[test]
    fn test_store_acc_to_de() {
        let index:usize = 0x0020;
        let mut memory: Vec<u8> = vec![0;100];
        let value:u8 = 0x1c;
        let mut state = State {a: value, d:0x00, e:0x20, memory,..Default::default()};
        let instruction = &Instruction{opcode:OpCode::STAX,register:vec![Register::D],..Default::default()};

        let result = store_accumulator(&mut state, instruction);
        assert_eq!(result, 1);
        assert_eq!(state.memory[index], value);
        assert_eq!(state.a, value);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0x20);
    }
}