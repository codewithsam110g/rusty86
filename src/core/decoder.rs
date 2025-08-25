use crate::core::{cpu::Cpu, instruction::*};

pub fn decode(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xB0..=0xBF | 0x8E | 0xC6 | 0xC7 | 0xA0..=0xA3 | 0x88..=0x8C => decode_mov(cpu, addr),
        0x2E | 0x3E | 0x26 | 0x36 => decode_seg_override(cpu, addr),
        0xE4 | 0xE5 | 0xEC | 0xED => decode_in(cpu, addr),
        0xE6 | 0xE7 | 0xEE | 0xEF => decode_out(cpu, addr),
        0xC3 | 0xCB | 0xC2 | 0xCA => decode_ret(cpu, addr),
        0xF8 | 0xFC | 0xFA => decode_clear_flags(cpu, addr),
        0xF9 | 0xFD | 0xFB => decode_store_flags(cpu, addr),
        0xCC | 0xCD | 0xCE => decode_int(cpu, addr),
        0x70..=0x7F => decode_jump(cpu, addr),
        0xF3 | 0xF2 => decode_rep(cpu, addr),
        0xE3 => decode_jcxz(cpu, addr),
        0xF4 => decode_hlt(cpu, addr),
        0x90 => decode_nop(cpu, addr),
        0x98 => decode_cbw(cpu, addr),
        0x99 => decode_cwd(cpu, addr),
        0x37 => decode_aaa(cpu, addr),
        0xD5 => decode_aad(cpu, addr),
        0xD4 => decode_aam(cpu, addr),
        0x3F => decode_aas(cpu, addr),
        0x27 => decode_daa(cpu, addr),
        0x2F => decode_das(cpu, addr),
        0xF5 => decode_cmc(cpu, addr),
        0xCF => decode_iret(cpu, addr),
        0x9E => decode_sahf(cpu, addr),
        0x9F => decode_lahf(cpu, addr),
        0xF0 => decode_lock(cpu, addr),
        0x9B => decode_wait(cpu, addr),
        0x9C => decode_pushf(cpu, addr),
        0x9D => decode_popf(cpu, addr),
        0xD7 => decode_xlat(cpu, addr),
        _ => {
            unimplemented!("TODO: Unknown opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_seg_override(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_jcxz(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_jump(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_ret(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_xlat(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_wait(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_popf(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_pushf(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_lock(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_sahf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9E => {
            cpu.regs.ip += 1;
            Instruction::Sahf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown SAHF opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_lahf(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x9F => {
            cpu.regs.ip += 1;
            Instruction::Lahf(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown LAHF opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_iret(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_int(cpu: &mut Cpu, addr: &u32) -> Instruction {
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

fn decode_store_flags(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0xF9 => Instruction::Stc(FillerInstruction { length: 1 }),
        0xFD => Instruction::Std(FillerInstruction { length: 1 }),
        0xFB => Instruction::Sti(FillerInstruction { length: 1 }),
        _ => {
            cpu.regs.ip -= 1;
            unimplemented!("TODO: Unknown Store Flag opcode: 0x{:2X}", opcode)
        }
    }
}
fn decode_clear_flags(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    cpu.regs.ip += 1;
    match opcode {
        0xF8 => Instruction::Clc(FillerInstruction { length: 1 }),
        0xFC => Instruction::Cld(FillerInstruction { length: 1 }),
        0xFA => Instruction::Cli(FillerInstruction { length: 1 }),
        _ => {
            cpu.regs.ip -= 1;
            unimplemented!("TODO: Unknown Clear Flag opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_cmc(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xF5 => {
            cpu.regs.ip += 1;
            Instruction::Cmc(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown CMC opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_daa(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x27 => {
            cpu.regs.ip += 1;
            Instruction::Daa(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown DAA opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_das(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x2F => {
            cpu.regs.ip += 1;
            Instruction::Das(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown DAS opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_aaa(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x37 => {
            cpu.regs.ip += 1;
            Instruction::Aaa(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown AAA opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_aas(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0x3F => {
            cpu.regs.ip += 1;
            Instruction::Aas(FillerInstruction { length: 1 })
        }
        _ => {
            unimplemented!("TODO: Unknown AAS opcode: 0x{:2X}", opcode)
        }
    }
}

fn decode_aam(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let base = cpu.read_byte(*addr + 1);
    match opcode {
        0xD4 => {
            cpu.regs.ip += 2;
            Instruction::Aam(AAMDBase {
                base: base,
                length: 2,
            })
        }
        _ => {
            unimplemented!("TODO: Unknown AAM opcode: 0x{:2X}:0x{:2X}", opcode, base)
        }
    }
}

fn decode_aad(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let base = cpu.read_byte(*addr + 1);
    match opcode {
        0xD5 => {
            cpu.regs.ip += 2;
            Instruction::Aad(AAMDBase {
                base: base,
                length: 2,
            })
        }
        _ => {
            unimplemented!("TODO: Unknown AAD opcode: 0x{:2X}:0x{:2X}", opcode, base)
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
