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
pub struct FillerInstruction {
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedOut {
    pub is_ax: bool,
    pub port_number: u8,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableOut {
    pub is_ax: bool,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutInstruction {
    Fixed(FixedOut),
    Variable(VariableOut),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedIn {
    pub is_ax: bool,
    pub port_number: u8,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableIn {
    pub is_ax: bool,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InInstruction {
    Fixed(FixedIn),
    Variable(VariableIn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepInstruction {
    Repz,
    Repnz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AAMDBase {
    pub base: u8,
    pub length: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntImm8Instruction {
    pub int_vector: u8,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntInstruction {
    Int3(FillerInstruction),
    IntImm8(IntImm8Instruction),
    Into(FillerInstruction),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetIntraInter {
    pub is_inter: bool,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetAddIntraInter {
    pub is_inter: bool,
    pub data: u16,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetInstruction {
    Ret(RetIntraInter),
    RetAdd(RetAddIntraInter),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Mov(MovInstruction),
    Nop(FillerInstruction),
    Hlt(FillerInstruction),
    Lock(FillerInstruction),
    Cbw(FillerInstruction),
    Cwd(FillerInstruction),
    Aaa(FillerInstruction),
    Aad(AAMDBase),
    Aam(AAMDBase),
    Aas(FillerInstruction),
    Daa(FillerInstruction),
    Das(FillerInstruction),
    Clc(FillerInstruction),
    Cld(FillerInstruction),
    Cli(FillerInstruction),
    Stc(FillerInstruction),
    Std(FillerInstruction),
    Sti(FillerInstruction),
    Cmc(FillerInstruction),
    Sahf(FillerInstruction),
    Lahf(FillerInstruction),
    Pushf(FillerInstruction),
    Popf(FillerInstruction),
    Iret(FillerInstruction),
    Wait(FillerInstruction),
    Xlat(FillerInstruction),
    Ret(RetInstruction),
    Int(IntInstruction),
    Out(OutInstruction),
    In(InInstruction),
    Rep(RepInstruction),
}
