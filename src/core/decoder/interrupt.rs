use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_iret(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xCF => {
            cpu.regs.ip += 1;
            Instruction::Iret(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown IRET opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_int(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xCC => {
            cpu.regs.ip += 1;
            Instruction::Int(IntInstruction::Int3(FillerInstruction { length: 1 }))
        }
        0xCD => {
            cpu.regs.ip += 2;
            let int_vector = cpu.read_byte(*addr + 1);
            Instruction::Int(IntInstruction::IntImm8(IntImm8Instruction {
                int_vector: int_vector,
                length: 2,
            }))
        }
        0xCE => {
            cpu.regs.ip += 1;
            Instruction::Int(IntInstruction::Into(FillerInstruction { length: 1 }))
        }
        _ => {
            unimplemented!("TODO: Unknown INT opcode: 0x{:2X}", opcode)
        }
    }
}