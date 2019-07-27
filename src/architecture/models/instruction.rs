extern crate strum;
extern crate strum_macros;

use crate::architecture::models::opcodes::OpCode;
use crate::architecture::models::registers::Register;
use crate::architecture::models::state::State;
use crate::architecture::units::processing_unit;


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

    pub fn size(&self) -> usize {
        self.operands.len() + 1
    }

    pub fn execute(&self, state: &mut State) -> usize {
        processing_unit::process_instruction(&self.opcode)(state, self)
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