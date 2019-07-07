use crate::architecture::isa;
use crate::architecture::state::State;
use crate::disassembler::parser;
use crate::architecture::condition_codes::ConditionCodes;

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
                ac: true,
                pad: 3
            },
            ..Default::default()
        };

        println!("Set memory to: {}", state.memory.capacity());

        CpuContext{
            state
        }
    }

    pub fn run(&mut self){
        let mem_end = self.state.memory.len();
        let mut program_exit: bool = false;

        while self.state.pc < mem_end && !program_exit {
            let instr = isa::read_next(&self.state.memory, self.state.pc);
            self.state.debug(instr.to_string());
            program_exit = instr.execute(&mut self.state);
        }
    }
}