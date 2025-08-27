use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_wait(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9B => {
            cpu.regs.ip += 1;
            Instruction::Wait(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown WAIT opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_hlt(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0xF4 {
        cpu.regs.ip += 1;
        Instruction::Hlt(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong HLT opcode: 0x{:2X}", opcode)
    }
}

pub fn decode_nop(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x90 {
        cpu.regs.ip += 1;
        Instruction::Nop(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong NOP opcode: 0x{:2X}", opcode)
    }
}