extern crate strum;
extern crate strum_macros;

use crate::opcodes::OpCode;
use crate::registers::Register;
use crate::state::State;
use crate::processor;

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

    pub fn num_of_bytes(&self) -> usize {
        self.operands.len() + 1
    }

    pub fn execute(&self, state: &State) -> bool {
        processor::determine_processing_unit(&self.opcode)(state, self)
    }
}