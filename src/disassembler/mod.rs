pub mod util;
pub mod parser;
pub mod converter;

use crate::disassembler::util::write_instructions_to_file;

pub fn disassemble(program: &str, with_address: bool) -> std::io::Result<()>{
    let program_in_bytes = parser::parse(program, 10000).unwrap();
    let instructions = converter::convert_to_assembly(program_in_bytes.clone());
    write_instructions_to_file(instructions, format!("{}.asm", program).as_str(), with_address)
}