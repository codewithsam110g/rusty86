use crate::core::decoder::decode;
use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u16 {
        const CARRY           = 0x0001; // Bit 0
        const PARITY          = 0x0004; // Bit 2
        const AUXILIARY_CARRY = 0x0010; // Bit 4
        const ZERO            = 0x0040; // Bit 6
        const SIGN            = 0x0080; // Bit 7
        const TRAP            = 0x0100; // Bit 8
        const INTERRUPT       = 0x0200; // Bit 9
        const DIRECTION       = 0x0400; // Bit 10
        const OVERFLOW        = 0x0800; // Bit 11
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    // General purpose registers
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,

    // Segment Registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub ss: u16,

    // Stack Pointer, Base Pointer
    pub sp: u16,
    pub bp: u16,

    // Source/Destination Index
    pub si: u16,
    pub di: u16,

    // Instruction Pointer (Program Counter)
    pub ip: u16,
}

impl Registers {
    pub fn al(&self) -> u8 {
        self.ax as u8
    }
    pub fn ah(&self) -> u8 {
        (self.ax >> 8) as u8
    }

    pub fn bl(&self) -> u8 {
        self.bx as u8
    }
    pub fn bh(&self) -> u8 {
        (self.bx >> 8) as u8
    }

    pub fn cl(&self) -> u8 {
        self.cx as u8
    }
    pub fn ch(&self) -> u8 {
        (self.cx >> 8) as u8
    }

    pub fn dl(&self) -> u8 {
        self.dx as u8
    }
    pub fn dh(&self) -> u8 {
        (self.dx >> 8) as u8
    }

    pub fn set_al(&mut self, value: u8) {
        // To set the low byte, we clear the existing low byte with a bitmask
        // and then OR in the new value.
        self.ax = (self.ax & 0xFF00) | (value as u16);
    }
    pub fn set_bl(&mut self, value: u8) {
        self.bx = (self.bx & 0xFF00) | (value as u16);
    }
    pub fn set_cl(&mut self, value: u8) {
        self.cx = (self.cx & 0xFF00) | (value as u16);
    }
    pub fn set_dl(&mut self, value: u8) {
        self.dx = (self.dx & 0xFF00) | (value as u16);
    }

    pub fn set_ah(&mut self, value: u8) {
        // To set the high byte, we clear the existing high byte,
        // then OR in the new value after it has been shifted left.
        self.ax = (self.ax & 0x00FF) | ((value as u16) << 8);
    }
    pub fn set_bh(&mut self, value: u8) {
        self.bx = (self.bx & 0x00FF) | ((value as u16) << 8);
    }
    pub fn set_ch(&mut self, value: u8) {
        self.cx = (self.cx & 0x00FF) | ((value as u16) << 8);
    }
    pub fn set_dh(&mut self, value: u8) {
        self.dx = (self.dx & 0x00FF) | ((value as u16) << 8);
    }
}

pub struct Cpu {
    pub regs: Registers,
    pub memory: Box<[u8; 1024 * 1024]>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers {
                sp: 0xFFFe,
                ..Default::default()
            },
            memory: Box::new([0; 1024 * 1024]),
        }
    }

    pub fn load_com(&mut self, program: &[u8], segment: Option<u16>, offset: Option<u16>) {
        let seg = segment.unwrap_or(0x1000);
        let oft = offset.unwrap_or(0x100);
        let load_address = Cpu::get_physical_address(seg, oft);
        for (i, &byte) in program.iter().enumerate() {
            self.memory[(load_address as usize) + i] = byte;
        }
        self.regs.ip = load_address as u16;
        self.regs.cs = seg;
        self.regs.ds = seg;
        self.regs.es = seg;
        self.regs.ss = seg;
    }

    pub fn get_physical_address(segment: u16, offset: u16) -> u32 {
        ((segment as u32) << 4) + (offset as u32)
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_word(&self, addr: u32) -> u16 {
        let low = self.read_byte(addr);
        let high = self.read_byte(addr + 1);
        u16::from_le_bytes([low, high])
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn write_word(&mut self, addr: u32, val: u16) {
        self.memory[addr as usize] = val as u8;
        self.memory[(addr + 1) as usize] = (val >> 8) as u8;
    }

    // The Corrected Stack Logic
    pub fn push(&mut self, val: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        let addr = Cpu::get_physical_address(self.regs.ss, self.regs.sp);
        self.write_word(addr, val);
    }

    pub fn pop(&mut self) -> u16 {
        let addr = Cpu::get_physical_address(self.regs.ss, self.regs.sp);
        let val = self.read_word(addr);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        val
    }

    pub fn step(&mut self) {
        let ins = decode(self, &Cpu::get_physical_address(self.regs.cs, self.regs.ip));
        println!("Decoded Instruction: {:#?}", ins)
    }
}
