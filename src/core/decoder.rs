use crate::core::{cpu::Cpu, instruction::*};

pub fn decode(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xB0..0xBF | 0x8E | 0xC6 | 0xC7 | 0xA0..=0xA3 | 0x88..=0x8C => decode_mov(cpu, addr),
        0xE4 | 0xE5 | 0xEC | 0xED => decode_in(cpu, addr),
        0xE6 | 0xE7 | 0xEE | 0xEF => decode_out(cpu, addr),
        0xF3 | 0xF2 => decode_rep(cpu, addr),
        0xF4 => decode_hlt(cpu, addr),
        0x90 => decode_nop(cpu, addr),
        0x98 => decode_cbw(cpu, addr),
        0x99 => decode_cwd(cpu, addr),
        _ => {
            unimplemented!("TODO: Unknown opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_rep(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_in(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_out(cpu: &mut Cpu, addr: &u32) -> Instruction {
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
        0xE8 => {
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

fn decode_cbw(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x98 {
        cpu.regs.ip += 1;
        Instruction::Cbw(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong CBW opcode: 0x{:2X}", opcode)
    }
}

fn decode_cwd(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x99 {
        cpu.regs.ip += 1;
        Instruction::Cwd(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong CWD opcode: 0x{:2X}", opcode)
    }
}

fn decode_hlt(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0xF4 {
        cpu.regs.ip += 1;
        Instruction::Hlt(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong HLT opcode: 0x{:2X}", opcode)
    }
}

fn decode_nop(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    if opcode == 0x90 {
        cpu.regs.ip += 1;
        Instruction::Nop(FillerInstruction { length: 1 })
    } else {
        unimplemented!("Wrong NOP opcode: 0x{:2X}", opcode)
    }
}

fn decode_mov(cpu: &mut Cpu, addr: &u32) -> Instruction {
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
            unimplemented!("TODO: Wrong MOV Opcode {:2X}", opcode)
        }
    }
}
