use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_popf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9D => {
            cpu.regs.ip += 1;
            Instruction::Popf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown POPF opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_pushf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9C => {
            cpu.regs.ip += 1;
            Instruction::Pushf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown PUSHF opcode: 0x{:2X}", opcode)
        }
    }
}
