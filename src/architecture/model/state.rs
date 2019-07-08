use crate::architecture::model::condition_codes::ConditionCodes;

#[derive(Default)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: usize,
    pub sp: usize,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8
}

impl State {
    pub fn debug(&self, instruction: String) {
        println!("*********************");
        println!("Executing: {} @ {:04x}",instruction, self.pc);
        println!("Registers: [a: {:02x}, b, {:02x}, c: {:02x}, d: {:02x}, e: {:02x}, h: {:02x}, l: {:02x}]", self.a, self.b, self.c, self.d, self.e, self.h, self.l);
        println!("Stack Pointer: {:04x}", self.sp);
        println!("Flags: [ac: {}, cy: {}, s: {}, p: {}, z: {}]", self.cc.ac, self.cc.cy, self.cc.s, self.cc.p, self.cc.z);
        println!("*********************");
    }
}