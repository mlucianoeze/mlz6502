use crate::instruction::isa::isa6502::INSTRUCTIONS_6502;
use crate::bus::Bus;

pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub pc: u16,
    pub cycles: u64,
    pub bus: Box<dyn Bus>,
}

impl Cpu6502 {
    pub fn new(bus: Box<dyn Bus>) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            p: 0x24,
            sp: 0xFD,
            pc: bus.read16(Self::reset_vector_addr()),
            cycles: 0,
            bus,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            let entry = INSTRUCTIONS_6502[opcode as usize];
            (entry.handler)(self);
            if entry.illegal {
                println!(
                    "Warning: illegal instruction '{}' (0x{:02x})",
                    entry.name, opcode
                );
            }
            if opcode == 0x92 {
                println!("Debug: magic 'jam' instruction reached. Stopping execution.");
                break;
            }
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let v = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }

    pub fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }

    pub fn push(&mut self, v: u8) {
        let addr = self.stack_addr();
        self.bus.write(addr, v);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = self.stack_addr();
        self.bus.read(addr)
    }

    #[inline]
    pub fn stack_addr(&self) -> u16 {
        Self::stack_base_addr() + self.sp as u16
    }

    #[inline]
    pub const fn stack_base_addr() -> u16 {
        0x0100
    }

    #[inline]
    pub const fn reset_vector_addr() -> u16 {
        0xFFFC
    }

    #[inline]
    pub fn carry(&self) -> bool {
        self.p & 0x01 != 0
    }

    #[inline]
    pub fn zero(&self) -> bool {
        self.p & 0x02 != 0
    }

    #[inline]
    pub fn interrupt_disable(&self) -> bool {
        self.p & 0x04 != 0
    }

    #[inline]
    pub fn decimal(&self) -> bool {
        self.p & 0x08 != 0
    }

    #[inline]
    pub fn break_flag(&self) -> bool {
        self.p & 0x10 != 0
    }

    #[inline]
    pub fn overflow(&self) -> bool {
        self.p & 0x40 != 0
    }

    #[inline]
    pub fn negative(&self) -> bool {
        self.p & 0x80 != 0
    }

    #[inline]
    pub fn set_carry(&mut self, on: bool) {
        if on {
            self.p |= 0x01;
        } else {
            self.p &= !0x01;
        }
    }

    pub fn set_zero(&mut self, on: bool) {
        if on {
            self.p |= 0x02;
        } else {
            self.p &= !0x02;
        }
    }

    pub fn set_negative(&mut self, v: u8) {
        if (v & 0x80) != 0 {
            self.p |= 0x80;
        } else {
            self.p &= !0x80;
        }
    }
}
