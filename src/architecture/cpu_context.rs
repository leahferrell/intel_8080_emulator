use crate::architecture::instruction_set;
use crate::architecture::model::condition_codes::ConditionCodes;
use crate::architecture::model::state::State;
use crate::disassembler::parser;
use crate::architecture::units::io_unit;

pub struct CpuContext {
    pub state: State
}

impl CpuContext {

    pub fn load_program(program: &str, mem_size: usize) -> CpuContext {
        let program_in_bytes = parser::parse(program, mem_size).unwrap();

        let state = State{
            memory: program_in_bytes,
            cc: ConditionCodes {
                z: true,
                s: true,
                p: true,
                cy: true,
                ac: true
            },
            ..Default::default()
        };

        println!("Set memory to: {}", state.memory.capacity());

        CpuContext{
            state
        }
    }

    pub fn run(&mut self){
        self.state.nc = self.state.pc;
        self.state.pc = self.state.nc + 1;

        while self.still_running() {
            self.cycle()
        }
    }

    fn still_running(&self) -> bool {
        self.state.pc < self.state.memory.len() && self.state.nc != self.state.pc
    }

    // TODO: Determine how many cycles each step takes & potentially add a "cycles remaining" param to each instruction
    //       Really just need to figure out how to get proper timings....
    fn cycle(&mut self){
        // TODO
        // 1. Check input queue and update accordingly (ex: interrupts, etc.)

        // 2. Execute instructions
        self.state.pc = self.state.nc;
        let instr = instruction_set::read_next(&self.state.memory, self.state.pc);
        //self.state.debug(instr.to_string());
        self.state.nc = instr.execute(&mut self.state);

        // 3. Execute any output queue instructions
        match self.state.output_queue.pop() {
            Some(os) => (io_unit::get_handler(os.device).handle(&mut self.state, os)),
            None => ()
        }
    }
}