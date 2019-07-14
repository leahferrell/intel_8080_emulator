use crate::architecture::models::condition_codes::ConditionCodes;
use crate::architecture::ExecutionStep;
use crate::architecture::AddressPtr;
use crate::architecture::IoSignal;

#[derive(Default)]
pub struct State {
    pub a: u8,                                 // accumulator
    pub b: u8,                                 // reg b
    pub c: u8,                                 // reg c
    pub d: u8,                                 // reg d
    pub e: u8,                                 // reg e
    pub h: u8,                                 // reg h
    pub l: u8,                                 // reg l
    pub pc: AddressPtr,                        // program counter
    pub sp: AddressPtr,                        // stack pointer
    pub nc: AddressPtr,                        // next counter
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: bool,                      // interrupt enabled / disabled
    pub stopped: bool,                         // if true, no further activity occurs until an interrupt
    pub input_queue: Vec<u8>,                  // signals sent from input devices
    pub output_queue: Vec<IoSignal>,       // signals sent to output devices
    pub instruction_queue: Vec<ExecutionStep>
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