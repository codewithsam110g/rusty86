use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_jcond(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 2;
    let signed_disp = cpu.read_byte(*addr + 1);
    let range = 0x70..=0x7F;
    let is_in_range = range.contains(&opcode) || opcode == 0xE3;
    if !is_in_range {
        cpu.regs.ip -= 2;
        unimplemented!("TODO: Unknown Jcond opcode: 0x{:2X}", opcode)
    } else {
        Instruction::Jcond(JumpInstruction {
            jump_condition: JumpCondition::try_from(opcode - 0x70).unwrap(),
            signed_disp: signed_disp as i8,
            length: 2,
        })
    }
}

pub fn decode_jcxz(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let signed_disp = cpu.read_byte(*addr + 1);
    match opcode {
        0x9B => {
            cpu.regs.ip += 2;
            Instruction::Jcxz(JcxzInstruction {
                signed_disp: signed_disp as i8,
                length: 2,
            })
        }
        _ => {
            unimplemented!("TODO: Unknown Jcxz opcode: 0x{:2X}", opcode)
        }
    }
}