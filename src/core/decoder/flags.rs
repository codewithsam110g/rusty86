use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_store_flags(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0xF9 => Instruction::Stc(FillerInstruction { length: 1 }),
        0xFD => Instruction::Std(FillerInstruction { length: 1 }),
        0xFB => Instruction::Sti(FillerInstruction { length: 1 }),
        _ => {
            cpu.regs.ip -= 1;
            unimplemented!("TODO: Unknown Store Flag opcode: 0x{:2X}", opcode)
        }
    }
}
pub fn decode_clear_flags(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0xF8 => Instruction::Clc(FillerInstruction { length: 1 }),
        0xFC => Instruction::Cld(FillerInstruction { length: 1 }),
        0xFA => Instruction::Cli(FillerInstruction { length: 1 }),
        _ => {
            cpu.regs.ip -= 1;
            unimplemented!("TODO: Unknown Clear Flag opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_cmc(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xF5 => {
            cpu.regs.ip += 1;
            Instruction::Cmc(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown CMC opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_sahf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9E => {
            cpu.regs.ip += 1;
            Instruction::Sahf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown SAHF opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_lahf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9F => {
            cpu.regs.ip += 1;
            Instruction::Lahf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown LAHF opcode: 0x{:2X}", opcode)
        }
    }
}

