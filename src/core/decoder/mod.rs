mod xlat;
mod flags;
mod in_out;
mod interrupt;
mod jump;
mod mov;
mod prefix;
mod stack;
mod utils;
mod nop;
mod convert;
mod ascii_decimal;
mod subroutine;
mod loop_set;

use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    match opcode {
        0xB0..=0xBF | 0x8E | 0xC6 | 0xC7 | 0xA0..=0xA3 | 0x88..=0x8C => mov::decode_mov(cpu, addr),
        0x2E | 0x3E | 0x26 | 0x36 => prefix::decode_seg_override(cpu, addr),
        0xE4 | 0xE5 | 0xEC | 0xED => in_out::decode_in(cpu, addr),
        0xE6 | 0xE7 | 0xEE | 0xEF => in_out::decode_out(cpu, addr),
        0xC3 | 0xCB | 0xC2 | 0xCA => subroutine::decode_ret(cpu, addr),
        0xF8 | 0xFC | 0xFA => flags::decode_clear_flags(cpu, addr),
        0xF9 | 0xFD | 0xFB => flags::decode_store_flags(cpu, addr),
        0xE0 | 0xE1 | 0xE2 => loop_set::decode_loop_set(cpu, addr),
        0xCC | 0xCD | 0xCE => interrupt::decode_int(cpu, addr),
        0x70..=0x7F => jump::decode_jcond(cpu, addr),
        0xF3 | 0xF2 => prefix::decode_rep(cpu, addr),
        0xE3 => jump::decode_jcxz(cpu, addr),
        0xF4 => nop::decode_hlt(cpu, addr),
        0x90 => nop::decode_nop(cpu, addr),
        0x98 => convert::decode_cbw(cpu, addr),
        0x99 => convert::decode_cwd(cpu, addr),
        0x37 => ascii_decimal::decode_aaa(cpu, addr),
        0xD5 => ascii_decimal::decode_aad(cpu, addr),
        0xD4 => ascii_decimal::decode_aam(cpu, addr),
        0x3F => ascii_decimal::decode_aas(cpu, addr),
        0x27 => ascii_decimal::decode_daa(cpu, addr),
        0x2F => ascii_decimal::decode_das(cpu, addr),
        0xF5 => flags::decode_cmc(cpu, addr),
        0xCF => interrupt::decode_iret(cpu, addr),
        0x9E => flags::decode_sahf(cpu, addr),
        0x9F => flags::decode_lahf(cpu, addr),
        0xF0 => prefix::decode_lock(cpu, addr),
        0x9B => nop::decode_wait(cpu, addr),
        0x9C => stack::decode_pushf(cpu, addr),
        0x9D => stack::decode_popf(cpu, addr),
        0xD7 => xlat::decode_xlat(cpu, addr),
        _ => {
            unimplemented!("TODO: Unknown opcode: 0x{:2X}", opcode)
        }
    }
}
