use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_seg_override(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0x2E => Instruction::Seg(SegmentOverride {
            segment: SegmentRegister::CS,
            length: 1,
        }),
        0x3E => Instruction::Seg(SegmentOverride {
            segment: SegmentRegister::DS,
            length: 1,
        }),
        0x26 => Instruction::Seg(SegmentOverride {
            segment: SegmentRegister::ES,
            length: 1,
        }),
        0x36 => Instruction::Seg(SegmentOverride {
            segment: SegmentRegister::SS,
            length: 1,
        }),
        _ => {
            cpu.regs.ip -= 1;
            unimplemented!("TODO: Unknown Segment Override: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_lock(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xF0 => {
            cpu.regs.ip += 1;
            Instruction::Lock(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown LOCK opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_rep(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0xF3 => Instruction::Rep(RepInstruction::Repz),
        0xF2 => Instruction::Rep(RepInstruction::Repnz),
        _ => {
            unimplemented!("TODO: Unknown Rep opcode: 0x{:2X}", opcode)
        }
    }
}
