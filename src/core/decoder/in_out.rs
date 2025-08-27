use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_in(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xE4 => {
            let port = cpu.read_byte(*addr + 1);
            cpu.regs.ip += 2;
            Instruction::In(InInstruction::Fixed(FixedIn {
                is_ax: false,
                port_number: port,
                length: 2,
            }))
        }
        0xE5 => {
            let port = cpu.read_byte(*addr + 1);
            cpu.regs.ip += 2;
            Instruction::In(InInstruction::Fixed(FixedIn {
                is_ax: true,
                port_number: port,
                length: 2,
            }))
        }
        0xEC => {
            cpu.regs.ip += 1;
            Instruction::In(InInstruction::Variable(VariableIn {
                is_ax: false,
                length: 1,
            }))
        }
        0xED => {
            cpu.regs.ip += 1;
            Instruction::In(InInstruction::Variable(VariableIn {
                is_ax: true,
                length: 1,
            }))
        }
        _ => {
            unimplemented!("TODO: In opcode: 0x{:2X}", opcode)
        }
    }
}

pub fn decode_out(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xE6 => {
            let port = cpu.read_byte(*addr + 1);
            cpu.regs.ip += 2;
            Instruction::Out(OutInstruction::Fixed(FixedOut {
                is_ax: false,
                port_number: port,
                length: 2,
            }))
        }
        0xE7 => {
            let port = cpu.read_byte(*addr + 1);
            cpu.regs.ip += 2;
            Instruction::Out(OutInstruction::Fixed(FixedOut {
                is_ax: true,
                port_number: port,
                length: 2,
            }))
        }
        0xEE => {
            cpu.regs.ip += 1;
            Instruction::Out(OutInstruction::Variable(VariableOut {
                is_ax: false,
                length: 1,
            }))
        }
        0xEF => {
            cpu.regs.ip += 1;
            Instruction::Out(OutInstruction::Variable(VariableOut {
                is_ax: true,
                length: 1,
            }))
        }
        _ => {
            unimplemented!("TODO: OUT opcode: 0x{:2X}", opcode)
        }
    }
}
