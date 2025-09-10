use crate::core::cpu::Cpu;
use crate::core::decoder::utils::decode_modrm_byte;
use crate::core::instruction::*;

pub fn decode_load_pointer(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let to_ds = opcode == 0xC5;
    let is_lea = opcode == 0x8D;
    let modrm_byte = cpu.read_byte(*addr + 1);
    let modrm = decode_modrm_byte(modrm_byte);
    let is_reg = matches!(modrm.rm_mode, RMMode::Reg(_));

    let regs = Register::try_from(8 + modrm.reg_part).unwrap();

    let decoded_rm = match is_reg {
        true => {
            unreachable!()
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

    let internal_struct = LoadInstructionData {
        decoded_mem_mode: decoded_rm,
        displacement: displacement,
        length: length as u8,
        register: regs,
    };
    let load_instr = if is_lea {
        LoadInstruction::LEA(internal_struct)
    } else if to_ds {
        LoadInstruction::LDS(internal_struct)
    } else {
        LoadInstruction::LES(internal_struct)
    };

    Instruction::LoadPointer(load_instr)
}
