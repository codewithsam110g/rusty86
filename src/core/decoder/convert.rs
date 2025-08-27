use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_cbw(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x98 {
        cpu.regs.ip += 1;
        Instruction::Cbw(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong CBW opcode: 0x{:2X}", opcode)
    }
}

pub fn decode_cwd(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x99 {
        cpu.regs.ip += 1;
        Instruction::Cwd(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong CWD opcode: 0x{:2X}", opcode)
    }
}
