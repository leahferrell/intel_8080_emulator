use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

type AddressPtr = usize;
type ExecutionStep = fn(&mut State, &Instruction) -> AddressPtr;

pub mod instruction_set;
pub mod instructions;
pub mod units;
pub mod cpu_context;
pub mod devices;
pub mod model;