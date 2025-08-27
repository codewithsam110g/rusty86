use crate::core::instruction::*;

pub fn decode_modrm_byte(modrm: u8) -> ModRM {
    let mod_bits: u8 = (modrm >> 6) & 0b11;
    let reg_bits: u8 = (modrm >> 3) & 0b111;
    let rm_bits: u8 = modrm & 0b111;

    // This is the new, more direct and correct way to determine displacement.
    let disp_mode = if mod_bits == 0b01 {
        DisplacementMode::BYTE
    } else if mod_bits == 0b10 {
        DisplacementMode::WORD
    } else if mod_bits == 0b00 && rm_bits == 0b110 {
        DisplacementMode::WORD
    } else {
        DisplacementMode::ZERO
    };

    if mod_bits != 0b11 {
        // --- Memory Mode ---
        ModRM {
            reg_part: reg_bits,
            displacement_mode: disp_mode,
            rm_mode: RMMode::Mem(if mod_bits == 0b00 && rm_bits == 0b110 {
                MemoryMode::DISP16
            } else {
                MemoryMode::try_from((mod_bits * 8) + rm_bits).unwrap()
            }),
        }
    } else {
        // --- Register Mode ---
        ModRM {
            reg_part: reg_bits,
            displacement_mode: DisplacementMode::ZERO, // No displacement in register mode
            rm_mode: RMMode::Reg(rm_bits),
        }
    }
}