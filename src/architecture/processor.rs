use crate::architecture::opcodes::OpCode;
use crate::architecture::opcodes::OpCode::*;
use crate::architecture::state::State;
use crate::architecture::instructions::*;
use crate::architecture::instruction::Instruction;
use crate::architecture::units::control_unit;

pub fn determine_processing_unit(opcode: &OpCode) -> fn(&mut State, &Instruction) -> bool {
    match opcode {
        NOP => nop::no_operation,
        LXI => immediate::load_register_pair,
        STAX => data_transfer::store_accumulator,
        INX => double_register::increment,
        INR => single_register::increment,
        DCR => single_register::decrement,
        MVI => immediate::move_data,
        RLC => rotate_accumulator::rotate_left,
        DAD => double_register::double_add,
        LDAX => data_transfer::load_accumulator,
        DCX => double_register::decrement,
        RRC => rotate_accumulator::rotate_right,
        RAL => rotate_accumulator::rotate_left_through_carry,
        RAR => rotate_accumulator::rotate_right_through_carry,
        SHLD => direct_addressing::store_hl,
        DAA => single_register::decimal_adjust_accumulator,
        LHLD => direct_addressing::load_hl,
        CMA => single_register::complement_accumulator,
        STA => direct_addressing::store_accumulator,
        STC => carry_bit::set,
        LDA => direct_addressing::load_accumulator,
        CMC => carry_bit::complement,
        MOV => data_transfer::move_reg,
        HLT => hlt::halt,
        ADD => to_accumulator::add,
        ADC => to_accumulator::add_with_carry,
        SUB => to_accumulator::sub,
        SBB => to_accumulator::sub_with_borrow,
        ANA => to_accumulator::logical_and,
        XRA => to_accumulator::logical_xor,
        ORA => to_accumulator::logical_or,
        CMP => to_accumulator::compare,
        RNZ => control_unit::no_zero,
        POP => double_register::pop,
        JNZ => control_unit::no_zero,
        JMP => control_unit::default,
        CNZ => control_unit::no_zero,
        PUSH => double_register::push,
        ADI => immediate::add,
        RST => rst::restart,
        RZ => control_unit::zero,
        RET => control_unit::default,
        JZ => control_unit::zero,
        CZ => control_unit::zero,
        CALL => control_unit::default,
        ACI => immediate::add_with_carry,
        RNC => control_unit::no_carry,
        JNC => control_unit::no_carry,
        OUT => io::output,
        CNC => control_unit::no_carry,
        SUI => immediate::sub,
        RC => control_unit::carry,
        JC => control_unit::carry,
        IN => io::input,
        CC => control_unit::carry,
        SBI => immediate::sub_with_borrow,
        RPO => control_unit::parity_odd,
        JPO => control_unit::parity_odd,
        XTHL => double_register::exchange_sp,
        CPO => control_unit::parity_odd,
        ANI => immediate::and,
        RPE => control_unit::parity_even,
        PCHL => control_unit::load_pc,
        JPE => control_unit::parity_even,
        XCHG => double_register::exchange,
        CPE => control_unit::parity_even,
        XRI => immediate::xor,
        RP => control_unit::positive,
        JP => control_unit::positive,
        DI => interrupt::disable,
        CP => control_unit::positive,
        ORI => immediate::or,
        RM => control_unit::minus,
        SPHL => double_register::load_sp_from_hl,
        JM => control_unit::minus,
        EI => interrupt::enable,
        CM => control_unit::minus,
        CPI => immediate::compare,
    }
}