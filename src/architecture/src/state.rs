use crate::condition_codes::ConditionCodes;
use crate::registers::Register;

#[derive(Default)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub m: u8,
    pub pc: usize,
    pub sp: usize,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8
}

impl State {
    pub fn set_8bit_reg(&mut self, reg: &Register, value: u8){
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::M => self.m = value,
            _ => ()
        };
    }

    pub fn debug(&self, instruction: String) {
        println!("*********************");
        println!("Executed: {}",instruction);
        println!("Registers: [a: {:02x}, b, {:02x}, c: {:02x}, d: {:02x}, e: {:02x}, h: {:02x}, l: {:02x}, m: {:02x}]", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.m);
        println!("Program Counter: {:04x}", self.pc);
        println!("Stack Pointer: {:04x}", self.sp);
        println!("*********************");
    }
}