use std::fmt;

use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Register {
    // 8-bit
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,

    // 16-bit
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SegmentRegister {
    ES,
    CS,
    SS,
    DS,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Immediate {
    Byte(u8),
    Word(u16),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemoryAddress {
    Byte(u32),
    Word(u32),
}

impl fmt::Debug for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Immediate::Byte(val) => write!(f, "Byte(0x{:02X})", val),

            Immediate::Word(val) => write!(f, "Word(0x{:02X})", val),
        }
    }
}

impl fmt::Debug for MemoryAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryAddress::Byte(val) => write!(f, "Byte(0x{:04X})", val),

            MemoryAddress::Word(val) => write!(f, "Word(0x{:04X})", val),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovImmToReg {
    pub dest: Register,
    pub imm: Immediate,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovMemToAcc {
    pub dest: Register,
    pub mem_addr: MemoryAddress,
    pub to_acc: bool,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovInstruction {
    ImmToReg(MovImmToReg),
    MemToAcc(MovMemToAcc),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Mov(MovInstruction),
}
