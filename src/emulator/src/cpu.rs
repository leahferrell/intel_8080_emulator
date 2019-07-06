use architecture::isa;
use architecture::state::State;
use disassembler::parser;

pub struct CpuContext {
    pub state: State
}

impl CpuContext {

    pub fn load_program(program: &str) -> CpuContext {
        let program_in_bytes = parser::parse(program).unwrap();
        let state = State{
            memory: program_in_bytes,
            ..Default::default()
        };

        CpuContext{
            state
        }
    }

    pub fn run(&mut self){
        let mem_end = self.state.memory.len();
        let mut program_exit: bool = false;

        while self.state.pc < mem_end && !program_exit {
            let instr = isa::read_next(&self.state.memory, self.state.pc);
            self.state.pc += instr.num_of_bytes();
            program_exit = instr.execute(&self.state);
        }
    }
}