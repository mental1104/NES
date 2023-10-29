/*
 * @Date: 2023-10-29 12:19:06
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 14:55:50
 */


pub(crate) const INSTRUCTION_MODE_MASK: u8 = 0x3;

pub(crate) const OPERATION_MASK: u8 = 0xe0;
pub(crate) const OPERATION_SHIFT: u8 = 5;

pub(crate) const ADDR_MODE_MASK: u8 = 0x1c;
pub(crate) const ADDR_MODE_SHIFT: u8 = 2;

pub(crate) const BRANCH_INSTRUCTION_MASK: u8 = 0x1f;
pub(crate) const BRANCH_INSTRUCTION_MASK_RESULT: u8 = 0x10;
//pub(crate) const BRANCH_CONDITION_MASK: u8 = 0x20;
pub(crate) const BRANCH_ON_FLAG_SHIFT: u8 = 6;

pub(crate) const RESET_VECTOR: u16 = 0xfffc;

pub(crate) enum BranchOnFlag {
    Negative,
    Overflow,
    Carry,
    Zero,
}

impl From<u8> for BranchOnFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => BranchOnFlag::Negative,
            1 => BranchOnFlag::Overflow,
            2 => BranchOnFlag::Carry,
            3 => BranchOnFlag::Zero,
            _ => panic!("Unknown integer value for BranchOnFlag enum"),
        }
    }
}

pub(crate) enum Operation1 {
    ORA, // 'OR' memory with ACC
    AND,
    EOR, // 'Exclusive OR' with ACC
    ADC,
    STA,
    LDA, // Load ACC with Mem
    CMP,
    SBC,
}

impl PartialEq for Operation1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operation1::ORA, Operation1::ORA) => true,
            (Operation1::AND, Operation1::AND) => true,
            (Operation1::EOR, Operation1::EOR) => true,
            (Operation1::ADC, Operation1::ADC) => true,
            (Operation1::STA, Operation1::STA) => true,
            (Operation1::LDA, Operation1::LDA) => true,
            (Operation1::CMP, Operation1::CMP) => true,
            (Operation1::SBC, Operation1::SBC) => true,
            _ => false,
        }
    }
}


impl From<u8> for Operation1 {
    fn from(value: u8) -> Self {
        match value {
            0 => Operation1::ORA,
            1 => Operation1::AND,
            2 => Operation1::EOR,
            3 => Operation1::ADC,
            4 => Operation1::STA,
            5 => Operation1::LDA,
            6 => Operation1::CMP,
            7 => Operation1::SBC,
            _ => panic!("Unknown opcode for Operation1"),
        }
    }
}

pub(crate) enum OperationImplied {
    NOP = 0xea,
    JSR = 0x20,
    RTI = 0x40,
    RTS = 0x60,
    JMP = 0x4c,
    JMPI = 0x6c, // JMP Indirect
    PHP = 0x08,
    PLP = 0x28,
    PHA = 0x48,
    PLA = 0x68,
    DEY = 0x88,
    DEX = 0xca,
    TAY = 0xa8,
    INY = 0xc8,
    INX = 0xe8,
    CLC = 0x18,
    SEC = 0x38,
    CLI = 0x58,
    SEI = 0x78,
    TYA = 0x98,
    CLV = 0xb8,
    CLD = 0xd8,
    SED = 0xf8,
    TXA = 0x8a,
    TXS = 0x9a,
    TAX = 0xaa,
    TSX = 0xba,
    Unknown
}

impl From<u8> for OperationImplied {
    fn from(value: u8) -> Self {
        match value {
            0xea => OperationImplied::NOP,
            0x20 => OperationImplied::JSR,
            0x40 => OperationImplied::RTI,
            0x60 => OperationImplied::RTS,
            0x4c => OperationImplied::JMP,
            0x6c => OperationImplied::JMPI,
            0x08 => OperationImplied::PHP,
            0x28 => OperationImplied::PLP,
            0x48 => OperationImplied::PHA,
            0x68 => OperationImplied::PLA,
            0x88 => OperationImplied::DEY,
            0xca => OperationImplied::DEX,
            0xa8 => OperationImplied::TAY,
            0xc8 => OperationImplied::INY,
            0xe8 => OperationImplied::INX,
            0x18 => OperationImplied::CLC,
            0x38 => OperationImplied::SEC,
            0x58 => OperationImplied::CLI,
            0x78 => OperationImplied::SEI,
            0x98 => OperationImplied::TYA,
            0xb8 => OperationImplied::CLV,
            0xd8 => OperationImplied::CLD,
            0xf8 => OperationImplied::SED,
            0x8a => OperationImplied::TXA,
            0x9a => OperationImplied::TXS,
            0xaa => OperationImplied::TAX,
            0xba => OperationImplied::TSX,
            _ => OperationImplied::Unknown
        }
    }
}


pub(crate) enum AddressingMode1 {
    IndexedIndirectX,
    ZeroPage,
    Immediate,
    Absolute,
    IndirectY,
    IndexedX,
    AbsoluteY,
    AbsoluteX,
}

impl From<u8> for AddressingMode1 {
    fn from(value: u8) -> Self {
        match value {
            0 => AddressingMode1::IndexedIndirectX,
            1 => AddressingMode1::ZeroPage,
            2 => AddressingMode1::Immediate,
            3 => AddressingMode1::Absolute,
            4 => AddressingMode1::IndirectY,
            5 => AddressingMode1::IndexedX,
            6 => AddressingMode1::AbsoluteY,
            7 => AddressingMode1::AbsoluteX,
            _ => unimplemented!(), // Handle the case for other u8 values if needed
        }
    }
}



pub(crate) enum Operation2 {
    ASL,
    ROL,
    LSR,
    ROR,
    STX,
    LDX,
    DEC,
    INC,
}

impl From<u8> for Operation2 {
    fn from(value: u8) -> Self {
        match value {
            0 => Operation2::ASL,
            1 => Operation2::ROL,
            2 => Operation2::LSR,
            3 => Operation2::ROR,
            4 => Operation2::STX,
            5 => Operation2::LDX,
            6 => Operation2::DEC,
            7 => Operation2::INC,
            _ => panic!("Unknown opcode for Operation2"),
        }
    }
}

impl PartialEq for Operation2 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operation2::ASL, Operation2::ASL) => true,
            (Operation2::ROL, Operation2::ROL) => true,
            (Operation2::LSR, Operation2::LSR) => true,
            (Operation2::ROR, Operation2::ROR) => true,
            (Operation2::STX, Operation2::STX) => true,
            (Operation2::LDX, Operation2::LDX) => true,
            (Operation2::DEC, Operation2::DEC) => true,
            (Operation2::INC, Operation2::INC) => true,
            _ => false,
        }
    }
}

pub(crate) enum AddressingMode2 {
    Immediate_,
    ZeroPage_,
    Accumulator,
    Absolute_,
    Indexed = 5,
    AbsoluteIndexed = 7,
}

impl From<u8> for AddressingMode2 {
    fn from(value: u8) -> Self {
        match value {
            0 => AddressingMode2::Immediate_,
            1 => AddressingMode2::ZeroPage_,
            2 => AddressingMode2::Accumulator,
            3 => AddressingMode2::Absolute_,
            5 => AddressingMode2::Indexed,
            7 => AddressingMode2::AbsoluteIndexed,
            _ => unimplemented!(), // Handle the case for other u8 values if needed
        }
    }
}

impl PartialEq for AddressingMode2 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AddressingMode2::Immediate_, AddressingMode2::Immediate_) => true,
            (AddressingMode2::ZeroPage_, AddressingMode2::ZeroPage_) => true,
            (AddressingMode2::Accumulator, AddressingMode2::Accumulator) => true,
            (AddressingMode2::Absolute_, AddressingMode2::Absolute_) => true,
            (AddressingMode2::Indexed, AddressingMode2::Indexed) => true,
            (AddressingMode2::AbsoluteIndexed, AddressingMode2::AbsoluteIndexed) => true,
            _ => false,
        }
    }
}

// pub(crate) enum Operation0 {
//     BIT = 1,
//     STY = 4,
//     LDY,
//     CPY,
//     CPX,
// }

pub(crate) const OPERATION_CYCLES: [u32; 0x100] = [
    7, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 0, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 3, 3, 5, 0, 4, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 3, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 4, 2, 2, 0, 5, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    0, 6, 0, 0, 3, 3, 3, 0, 2, 0, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 4, 4, 4, 0, 2, 5, 2, 0, 0, 5, 0, 0,
    2, 6, 2, 0, 3, 3, 3, 0, 2, 2, 2, 0, 4, 4, 4, 0,
    2, 5, 0, 0, 4, 4, 4, 0, 2, 4, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 2, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
];
