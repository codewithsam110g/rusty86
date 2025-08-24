use crate::core::{cpu::Cpu, instruction::*};

pub fn decode(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xB0..0xBF | 0x8E | 0xC6 | 0xC7 | 0xA0..=0xA3 | 0x88..=0x8C => decodeMov(cpu, addr),
        _ => {
            unimplemented!("TODO: Unknown opcode: 0x{:X}", opcode)
        }
    }
}
fn decodeMov(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xB0..=0xB7 => {
            // MOV reg, imm8 (B0h + reg)
            // Opcode --- Data  === Max 2 bytes
            let imm = cpu.read_byte(*addr + 1);
            let mov_struct = MovImmToReg {
                dest: Register::try_from(opcode - 0xB0).unwrap(),
                imm: Immediate::Byte(imm),
                length: 2,
            };
            let mov_instruction = MovInstruction::ImmToReg(mov_struct);
            cpu.regs.ip += 2;
            Instruction::Mov(mov_instruction)
        }
        0xB8..=0xBF => {
            // MOV reg, imm16 (B0h + reg)
            // Opcode --- Data(L) --- OP1(H) === Max 3 bytes
            let imm_l = cpu.read_byte(*addr + 1);
            let imm_h = cpu.read_byte(*addr + 2);
            let mov_struct = MovImmToReg {
                dest: Register::try_from(opcode - 0xB0).unwrap(),
                imm: Immediate::Word(((imm_h as u16) << 8) | (imm_l as u16)),
                length: 3,
            };
            cpu.regs.ip += 3;
            let mov_instruction = MovInstruction::ImmToReg(mov_struct);
            Instruction::Mov(mov_instruction)
        }
        0x8E => {
            // Handle 8E
            unimplemented!("TODO: Decode MOV opcode 8e")
        }
        0x8C => {
            // Handle 8C
            unimplemented!("TODO: Decode MOV opcode 8c")
        }
        0xC6 => {
            // Handle C6
            unimplemented!("TODO: Decode MOV opcode c6")
        }
        0xC7 => {
            // Handle C7
            unimplemented!("TODO: Decode MOV opcode c7")
        }
        0xA0..=0xA3 => {
            // Opcode --- AddrL --- AddrH === Max 3 bytes
            // MOV AL, [addr] (A0h)
            // MOV AX, [addr] (A1h)
            // MOV [addr], AL (A2h)
            // MOV [addr], AX (A3h)

            let is16_bit = opcode == 0xA1 || opcode == 0xA3;
            let is_mem_to_acc = opcode == 0xA0 || opcode == 0xA1;
            if !is16_bit {
                if is_mem_to_acc {
                    // (A0h)
                    let addr_l = cpu.read_byte(*addr + 1);
                    let addr_h = cpu.read_byte(*addr + 2);
                    let addr = Cpu::get_physical_address(
                        cpu.regs.ds,
                        ((addr_h as u16) << 8) | (addr_l as u16),
                    );
                    println!("Mem value: {:02X}", cpu.read_byte(addr));
                    let mov_struct = MovMemToAcc {
                        dest: Register::AL,
                        mem_addr: MemoryAddress::Byte(addr),
                        to_acc: true,
                        length: 3,
                    };
                    cpu.regs.ip += 3;
                    Instruction::Mov(MovInstruction::MemToAcc(mov_struct))
                } else {
                    // (A2h)
                    let addr_l = cpu.read_byte(*addr + 1);
                    let addr_h = cpu.read_byte(*addr + 2);
                    let addr = Cpu::get_physical_address(
                        cpu.regs.ds,
                        ((addr_h as u16) << 8) | (addr_l as u16),
                    );
                    println!("Mem value: {:02X}", cpu.read_byte(addr));

                    let mov_struct = MovMemToAcc {
                        dest: Register::AL,
                        mem_addr: MemoryAddress::Byte(addr),
                        to_acc: false,
                        length: 3,
                    };
                    cpu.regs.ip += 3;
                    Instruction::Mov(MovInstruction::MemToAcc(mov_struct))
                }
            } else {
                if is_mem_to_acc {
                    // (A1h)
                    let addr_l = cpu.read_byte(*addr + 1);
                    let addr_h = cpu.read_byte(*addr + 2);
                    let addr = Cpu::get_physical_address(
                        cpu.regs.ds,
                        ((addr_h as u16) << 8) | (addr_l as u16),
                    );
                    println!("Mem value: {:04X}", cpu.read_word(addr));

                    let mov_struct = MovMemToAcc {
                        dest: Register::AX,
                        mem_addr: MemoryAddress::Word(addr),

                        to_acc: true,
                        length: 3,
                    };
                    cpu.regs.ip += 3;
                    Instruction::Mov(MovInstruction::MemToAcc(mov_struct))
                } else {
                    // (A3h)
                    let addr_l = cpu.read_byte(*addr + 1);
                    let addr_h = cpu.read_byte(*addr + 2);
                    let addr = Cpu::get_physical_address(
                        cpu.regs.ds,
                        ((addr_h as u16) << 8) | (addr_l as u16),
                    );
                    println!("Mem value: {:04X}", cpu.read_word(addr));
                    let mov_struct = MovMemToAcc {
                        dest: Register::AX,
                        mem_addr: MemoryAddress::Word(addr),
                        to_acc: false,
                        length: 3,
                    };
                    cpu.regs.ip += 3;
                    Instruction::Mov(MovInstruction::MemToAcc(mov_struct))
                }
            }
        }
        0x88..=0x8B => {
            // Handle 88–8B
            unimplemented!("TODO: Decode MOV opcode 88->8B")
        }
        _ => {
            // Default case
            unimplemented!("TODO: Wrong Mov Opcode")
        }
    }
}
