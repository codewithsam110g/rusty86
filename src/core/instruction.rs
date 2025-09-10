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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Registers {
    Gpr(Register),
    Seg(SegmentRegister),
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
pub enum Displacement {
    Zero(u8),
    Byte(i8),
    Word(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovMemToReg {
    // Always to REG part in modrm
    // R/M Part will always encode a Mem addr
    pub is_16bit: bool,
    pub decoded_rm: DecodedRMMode,
    pub decdode_reg: Registers,
    pub displacement: Displacement,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodedRMMode {
    Mem(MemoryMode),
    Reg(Register),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovRegToRM {
    // Always to RM part in modrm
    pub is_16bit: bool,
    pub is_rm_a_reg: bool,
    pub decoded_rm: DecodedRMMode,
    pub decdode_reg: Registers,
    pub displacement: Displacement,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovSregToFromRM {
    pub to_rm: bool,
    pub is_rm_a_reg: bool,
    pub decoded_rm: DecodedRMMode,
    pub decdode_reg: Registers,
    pub displacement: Displacement,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovImmToRM {
    pub is_16bit: bool,
    pub is_rm_a_reg: bool,
    pub decoded_rm: DecodedRMMode,
    pub displacement: Displacement,
    pub imm: Immediate,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovInstruction {
    ImmToReg(MovImmToReg),     // Type 3
    MemToAcc(MovMemToAcc),     // Type 2
    MemToReg(MovMemToReg),     // Type 1 8Ah, 8Bh
    RegToRM(MovRegToRM),       // Type 1 88h, 89h
    SregToRM(MovSregToFromRM), // Type 5 8Ch
    RMToSreg(MovSregToFromRM), // Type 5 8Eh
    ImmToRM(MovImmToRM),       // Type 1 C6h, C7h
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum JumpCondition {
    JO,
    JNO,
    JB_JC_JNAE,
    JAE_JNB_JNC,
    JE_JZ,
    JNE_JNZ,
    JBE_JNA,
    JA_JNBE,
    JS,
    JNS,
    JP_JPE,
    JNP_JPO,
    JL_JNGE,
    JGE_JNL,
    JLE_JNG,
    JG_JNLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpInstruction {
    pub jump_condition: JumpCondition,
    pub signed_disp: i8,
    pub length: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JcxzInstruction {
    pub signed_disp: i8,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentOverride {
    pub segment: SegmentRegister,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MemoryMode {
    BX_SI,
    BX_DI,
    BP_SI,
    BP_DI,
    SI,
    DI,
    DISP16,
    BX,

    BX_SI_DISP8,
    BX_DI_DISP8,
    BP_SI_DISP8,
    BP_DI_DISP8,
    SI_DISP8,
    DI_DISP8,
    BP_DISP8,
    BX_DISP8,

    BX_SI_DISP16,
    BX_DI_DISP16,
    BP_SI_DISP16,
    BP_DI_DISP16,
    SI_DISP16,
    DI_DISP16,
    BP_DISP16,
    BX_DIS168,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RMMode {
    Mem(MemoryMode),
    Reg(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplacementMode {
    ZERO,
    BYTE,
    WORD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModRM {
    pub displacement_mode: DisplacementMode,
    pub reg_part: u8,
    pub rm_mode: RMMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoopCondition {
    NZERO_NEQUAL,
    ZERO_EQUAL,
    DIRECT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopInstruction {
    pub loop_condition: LoopCondition,
    pub disp: i8,
    pub length: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadInstructionData {
    pub register: Register,
    pub displacement: Displacement,
    pub decoded_mem_mode: DecodedRMMode,
    pub length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadInstruction {
    LDS(LoadInstructionData),
    LES(LoadInstructionData),
    LEA(LoadInstructionData)
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
    Jcond(JumpInstruction),
    Jcxz(JcxzInstruction),
    Seg(SegmentOverride),
    Loop(LoopInstruction),
    LoadPointer(LoadInstruction),
}
