use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_ret(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xC3 => {
            cpu.regs.ip += 1;
            Instruction::Ret(RetInstruction::Ret(RetIntraInter {
                is_inter: false,
                length: 1,
            }))
        }
        0xCB => {
            cpu.regs.ip += 1;
            Instruction::Ret(RetInstruction::Ret(RetIntraInter {
                is_inter: true,
                length: 1,
            }))
        }
        0xC2 => {
            cpu.regs.ip += 3;
            let data_l = cpu.read_byte(*addr + 1);
            let data_h = cpu.read_byte(*addr + 2);
            let data: u16 = ((data_h as u16) << 8) | (data_l as u16);
            Instruction::Ret(RetInstruction::RetAdd(RetAddIntraInter {
                is_inter: false,
                data: data,
                length: 3,
            }))
        }
        0xCA => {
            cpu.regs.ip += 3;
            let data_l = cpu.read_byte(*addr + 1);
            let data_h = cpu.read_byte(*addr + 2);
            let data: u16 = ((data_h as u16) << 8) | (data_l as u16);
            Instruction::Ret(RetInstruction::RetAdd(RetAddIntraInter {
                is_inter: true,
                data: data,
                length: 3,
            }))
        }
        _ => {
            unimplemented!("TODO: Unknown RET opcode: 0x{:2X}", opcode)
        }
    }
}
