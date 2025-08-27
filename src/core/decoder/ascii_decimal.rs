use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_daa(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x27 => {
            cpu.regs.ip += 1;
            Instruction::Daa(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown DAA opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_das(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x2F => {
            cpu.regs.ip += 1;
            Instruction::Das(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown DAS opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_aaa(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x37 => {
            cpu.regs.ip += 1;
            Instruction::Aaa(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown AAA opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_aas(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x3F => {
            cpu.regs.ip += 1;
            Instruction::Aas(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown AAS opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_aam(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let base = cpu.read_byte(*addr + 1);
    match opcode {
        0xD4 => {
            cpu.regs.ip += 2;
            Instruction::Aam(AAMDBase {
                base: base,
                length: 2,
            })
        }
        _ => {
            unimplemented!("TODO: Unknown AAM opcode: 0x{:2X}:0x{:2X}", opcode, base)
        }
    }
}

pub fn decode_aad(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let base = cpu.read_byte(*addr + 1);
    match opcode {
        0xD5 => {
            cpu.regs.ip += 2;
            Instruction::Aad(AAMDBase {
                base: base,
                length: 2,
            })
        }
        _ => {
            unimplemented!("TODO: Unknown AAD opcode: 0x{:2X}:0x{:2X}", opcode, base)
        }
    }
}