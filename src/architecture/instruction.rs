extern crate strum;
extern crate strum_macros;

use crate::architecture::opcodes::OpCode;
use crate::architecture::registers::Register;
use crate::architecture::state::State;
use crate::architecture::processor;

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction{
    pub opcode: OpCode,
    pub register: Vec<Register>,
    pub operands: Vec<u8>
}

impl Instruction{
    pub fn to_string(&self) -> String {
        let mut instruction:String = self.opcode.to_string();

        match self.register.len() {
            1 => instruction = format!("{} {}", instruction, self.register[0]),
            2 => instruction = format!("{} {},{}", instruction, self.register[0], self.register[1]),
            _ => ()
        }

        match self.operands.len() {
            1 if self.register.len() == 0 => instruction = format!("{} #${:02x}", instruction, self.operands[0]),
            1 if self.register.len() > 0 => instruction = format!("{},#${:02x}", instruction, self.operands[0]),
            2 if self.register.len() == 0 => instruction = format!("{} ${:02x}{:02x}",instruction, self.operands[0], self.operands[1]),
            2 if self.register.len() > 0 => instruction = format!("{},#${:02x}{:02x}",instruction, self.operands[0], self.operands[1]),
            _ => ()
        }

        instruction
    }

    pub fn get_addr(&self) -> u16 {
        match self.operands.as_slice() {
            [first, second] => ((*first as u16) << 8) | *second as u16,
            [first] => *first as u16,
            _ => 0 as u16
        }
    }

    pub fn num_of_bytes(&self) -> usize {
        self.operands.len() + 1
    }

    pub fn execute(&self, state: &mut State) -> bool {
        processor::determine_processing_unit(&self.opcode)(state, self)
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: OpCode::NOP,
            register: vec![],
            operands: vec![]
        }
    }
}