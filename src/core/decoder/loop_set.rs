use crate::core::cpu::Cpu;
use crate::core::instruction::*;

pub fn decode_loop_set(cpu: &mut Cpu, addr: &u32) -> Instruction {
    let opcode = cpu.read_byte(*addr);
    let disp = cpu.read_byte(*addr + 1) as i8;
    let index = opcode - 0xE0;
    Instruction::Loop(LoopInstruction {
        loop_condition: LoopCondition::try_from(index).unwrap(),
        disp: disp,
        length: 2,
    })
}
