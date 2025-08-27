use crate::core::cpu::Cpu;
use crate::core::decoder::utils::decode_modrm_byte;
use crate::core::instruction::*;

pub fn decode_mov(cpu: &mut Cpu, addr: &u32) -> Instruction {
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
        0x8E | 0x8C => {
            let modrm_byte = cpu.read_byte(*addr + 1);
            let modrm = decode_modrm_byte(modrm_byte);
            let is_reg = matches!(modrm.rm_mode, RMMode::Reg(_));
            let to_rm = opcode == 0x8C;

            let regs = Registers::Seg(SegmentRegister::try_from(modrm.reg_part).unwrap());

            let decoded_rm = match is_reg {
                true => {
                    let rm_val = match modrm.rm_mode {
                        RMMode::Reg(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Reg(Register::try_from(8 + rm_val).unwrap())
                }
                false => {
                    let addr_mode = match modrm.rm_mode {
                        RMMode::Mem(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Mem(addr_mode)
                }
            };

            // Extract displacement and calculate instruction length
            let (displacement, length) = match modrm.displacement_mode {
                DisplacementMode::BYTE => {
                    let disp8 = cpu.read_byte(*addr + 2);
                    (Displacement::Byte(disp8 as i8), 3)
                }
                DisplacementMode::WORD => {
                    let disp16 = cpu.read_word(*addr + 2);
                    (Displacement::Word(disp16 as i16), 4)
                }
                DisplacementMode::ZERO => (Displacement::Zero(0), 2),
            };

            cpu.regs.ip += length;

            // Create appropriate instruction variant
            let internal_struct = MovSregToFromRM {
                is_rm_a_reg: is_reg,
                decdode_reg: regs,
                to_rm: to_rm,
                decoded_rm,
                displacement,
                length: length as u8,
            };
            let mov_instruction = if to_rm {
                MovInstruction::SregToRM(internal_struct)
            } else {
                MovInstruction::RMToSreg(internal_struct)
            };

            Instruction::Mov(mov_instruction)
        }
        0xC6 | 0xC7 => {
            let is_16bit = opcode == 0xC7;
            let modrm_byte = cpu.read_byte(*addr + 1);
            let modrm = decode_modrm_byte(modrm_byte);
            let is_reg = matches!(modrm.rm_mode, RMMode::Reg(_));

            let decoded_rm = match is_reg {
                true => {
                    let rm_val = match modrm.rm_mode {
                        RMMode::Reg(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Reg(Register::try_from(((is_16bit as u8) * 8) + rm_val).unwrap())
                }
                false => {
                    let addr_mode = match modrm.rm_mode {
                        RMMode::Mem(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Mem(addr_mode)
                }
            };

            // Extract displacement and calculate instruction length
            let (displacement, mut length) = match modrm.displacement_mode {
                DisplacementMode::BYTE => {
                    let disp8 = cpu.read_byte(*addr + 2);
                    (Displacement::Byte(disp8 as i8), 3)
                }
                DisplacementMode::WORD => {
                    let disp16 = cpu.read_word(*addr + 2);
                    (Displacement::Word(disp16 as i16), 4)
                }
                DisplacementMode::ZERO => (Displacement::Zero(0), 2),
            };
            let imm = if is_16bit {
                let val = Immediate::Word(cpu.read_word(*addr + length));
                length += 2;
                val
            } else {
                let val = Immediate::Byte(cpu.read_byte(*addr + length));
                length += 1;
                val
            };
            cpu.regs.ip += length as u16;
            let mov_struct = MovInstruction::ImmToRM(MovImmToRM {
                is_16bit: is_16bit,
                is_rm_a_reg: is_reg,
                decoded_rm: decoded_rm,
                displacement: displacement,
                imm: imm,
                length: (length as u8),
            });
            Instruction::Mov(mov_struct)
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
            let modrm_byte = cpu.read_byte(*addr + 1);
            let modrm = decode_modrm_byte(modrm_byte);
            let is_reg = matches!(modrm.rm_mode, RMMode::Reg(_));
            let is_16bit = opcode == 0x89 || opcode == 0x8B;
            let is_mem_to_reg = opcode == 0x8A || opcode == 0x8B;

            let regs = Registers::Gpr(
                Register::try_from(((is_16bit as u8) * 8) + modrm.reg_part).unwrap(),
            );

            let decoded_rm = match is_reg {
                true => {
                    let rm_val = match modrm.rm_mode {
                        RMMode::Reg(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Reg(Register::try_from(((is_16bit as u8) * 8) + rm_val).unwrap())
                }
                false => {
                    let addr_mode = match modrm.rm_mode {
                        RMMode::Mem(val) => val,
                        _ => unreachable!(),
                    };
                    DecodedRMMode::Mem(addr_mode)
                }
            };

            // Extract displacement and calculate instruction length
            let (displacement, length) = match modrm.displacement_mode {
                DisplacementMode::BYTE => {
                    let disp8 = cpu.read_byte(*addr + 2);
                    (Displacement::Byte(disp8 as i8), 3)
                }
                DisplacementMode::WORD => {
                    let disp16 = cpu.read_word(*addr + 2);
                    (Displacement::Word(disp16 as i16), 4)
                }
                DisplacementMode::ZERO => (Displacement::Zero(0), 2),
            };

            cpu.regs.ip += length;

            // Create appropriate instruction variant
            let mov_instruction = if is_mem_to_reg {
                MovInstruction::MemToReg(MovMemToReg {
                    is_16bit,
                    decdode_reg: regs,
                    decoded_rm,
                    displacement,
                    length: length as u8,
                })
            } else {
                MovInstruction::RegToRM(MovRegToRM {
                    is_rm_a_reg: is_reg,
                    is_16bit,
                    decdode_reg: regs,
                    decoded_rm,
                    displacement,
                    length: length as u8,
                })
            };

            Instruction::Mov(mov_instruction)
        }
        _ => {
            // Default case
            unimplemented!("TODO: Wrong MOV Opcode {:2X}", opcode)
        }
    }
}
