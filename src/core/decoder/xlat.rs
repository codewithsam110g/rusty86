use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_xlat(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xD7 => {
            cpu.regs.ip += 1;
            Instruction::Xlat(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown XLAT opcode: 0x{:2X}", opcode)
        }
    }
}