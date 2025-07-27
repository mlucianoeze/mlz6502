use core::marker::PhantomData;

pub trait Bus : Copy {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn read16(&mut self, addr: u16) -> u16 {
        read_little_endian(self.read(addr), self.read(addr.wrapping_add(1)))
    }
}

#[inline]
fn read_little_endian(lo: u8, hi: u8) -> u16 {
    (hi as u16) << 8 | lo as u16
}

const MEM_64KB: usize = 0x10000;

#[derive(Copy, Clone)]
pub struct Memory([u8; MEM_64KB]);

impl Memory {
    pub fn new() -> Self {
        Self([0; MEM_64KB])
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.load_rom_at(rom, 0);
    }

    pub fn load_rom_at(&mut self, rom: &[u8], addr: u16) {
        assert!(addr as usize + rom.len() <= MEM_64KB, "ROM too large to fit in memory");
        self.0[addr as usize..addr as usize + rom.len()].copy_from_slice(rom);
    }
}

impl Bus for Memory {
    fn read(&mut self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value;
    }
}

pub struct Cpu6502<B: Bus> {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub pc: u16,
    pub cycles: u64,
    pub bus: B,
    pub instruction_table: [InstructionEntry<B>; 256],
}

impl<B: Bus> Cpu6502<B> {

    pub fn new(bus: B) -> Self {
        let mut instruction_table = [InstructionVariant::<B, Jam, Implicit, operand::Imp>::entry(); 256];
        instruction_table[0xA9] = InstructionVariant::<B, Lda, Immediate, operand::Val>::entry();
        instruction_table[0x8D] = InstructionVariant::<B, Sta, Absolute, operand::Addr>::entry();

        Self {
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            sp: 0,
            pc: 0,
            cycles: 0,
            bus,
            instruction_table,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            let entry = self.instruction_table[opcode as usize];
            (entry.handler)(self);
            if entry.illegal {
                println!("Warning: illegal instruction '{}' (0x{:02x})", entry.name(), opcode);
            }
            if opcode == 0x92 {
                println!("Debug: magic 'jam' instruction reached. Stopping execution.");
                break;
            }
        }
    }

    #[inline]
    fn fetch(&mut self) -> u8 {
        let v = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }

    #[inline]
    fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo 
    }

    #[inline]
    pub fn push(&mut self, v: u8) {
        let addr = 0x0100u16 | self.sp as u16;
        self.bus.write(addr, v);
        self.sp = self.sp.wrapping_sub(1);
    }

    #[inline]
    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 | self.sp as u16;
        self.bus.read(addr)
    }

    #[inline]
    pub fn set_zero(&mut self, on: bool)  {
        if on {
            self.p |= 0x02;
        } else {
            self.p &= !0x02;
        }
    }

    #[inline]
    pub fn set_carry(&mut self, on: bool) {
        if on {
            self.p |= 0x01;
        } else {
            self.p &= !0x01;
        }
    }

    #[inline]
    pub fn set_negative(&mut self, v: u8) {
        if (v & 0x80) != 0 {
            self.p |= 0x80;
        } else {
            self.p &= !0x80;
        }
    }

}

mod operand {

    #[derive(Clone, Copy)]
    pub struct Val(pub u8);

    #[derive(Clone, Copy)]
    pub struct Addr(pub u16);

    #[derive(Clone, Copy)]
    pub struct Rel(pub i8);

    #[derive(Clone, Copy)]
    pub struct Acc;

    #[derive(Clone, Copy)]
    pub struct Imp;

    pub trait Operand {}
    impl Operand for Val {}
    impl Operand for Addr {}
    impl Operand for Rel {}
    impl Operand for Acc {}
    impl Operand for Imp {}
}

pub trait Instruction<B: Bus, O: operand::Operand> {
    fn exec(cpu: &mut Cpu6502<B>, op: O);
    fn illegal() -> bool { false }

    fn name<I: Instruction<B, O>>() -> &'static str {
        std::any::type_name::<I>().split("::").last().unwrap_or("???")
    }
}

pub trait Addressing<B: Bus, O: operand::Operand> {
    fn resolve(cpu: &mut Cpu6502<B>) -> O;
}

// Instructions

pub struct Jam;
impl<B: Bus> Instruction<B, operand::Imp> for Jam {
    fn exec(_cpu: &mut Cpu6502<B>, _: operand::Imp) {}
    fn illegal() -> bool { true }
}

pub struct Cpx;
impl<B: Bus> Instruction<B, operand::Val> for Cpx {
    fn exec(cpu: &mut Cpu6502<B>, operand::Val(v): operand::Val) {
        let x = cpu.x;
        cpu.set_carry(x >= v);
        cpu.set_zero(x == v);
        cpu.set_negative(x.wrapping_sub(v));
    }
}

pub struct Sta;
impl<B: Bus> Instruction<B, operand::Addr> for Sta {
    fn exec(cpu: &mut Cpu6502<B>, operand::Addr(addr): operand::Addr) {
        cpu.bus.write(addr, cpu.a);
    }
}

pub struct Lda;
impl<B: Bus> Instruction<B, operand::Val> for Lda {
    fn exec(cpu: &mut Cpu6502<B>, operand::Val(v): operand::Val) {
        cpu.a = v;
        cpu.set_zero(cpu.a == 0);
        cpu.set_negative(cpu.a);
    }
}

// Addressing modes

pub struct Implicit;
impl<B: Bus> Addressing<B, operand::Imp> for Implicit {
    fn resolve(_cpu: &mut Cpu6502<B>) -> operand::Imp {
        operand::Imp
    }
}

pub struct Immediate;
impl<B: Bus> Addressing<B, operand::Val> for Immediate {
    fn resolve(cpu: &mut Cpu6502<B>) -> operand::Val {
        operand::Val(cpu.fetch())
    }
}

pub struct Absolute;
impl Absolute {
    #[inline]
    fn eff_addr<B: Bus>(cpu: &mut Cpu6502<B>) -> u16 {
        cpu.fetch16()
    }
}

impl<B: Bus> Addressing<B, operand::Addr> for Absolute {
    fn resolve(cpu: &mut Cpu6502<B>) -> operand::Addr {
        operand::Addr(Self::eff_addr(cpu))
    }
}

impl<B: Bus> Addressing<B, operand::Val> for Absolute {
    fn resolve(cpu: &mut Cpu6502<B>) -> operand::Val {
        let addr = Self::eff_addr(cpu);
        operand::Val(cpu.bus.read(addr))
    }
}

#[derive(Copy, Clone)]
pub struct InstructionEntry<B: Bus> {
    name: &'static str,
    pub illegal: bool,
    pub cycles: u8,
    pub handler: fn(&mut Cpu6502<B>),
}

impl<B: Bus> InstructionEntry<B> {
    pub fn new<I: Instruction<B, O>, O: operand::Operand>(handler: fn(&mut Cpu6502<B>)) -> Self {
        Self {
            name: I::name::<I>(),
            illegal: I::illegal(),
            cycles: 0, // TODO: get cycles from instruction
            handler,
        }
    }

    pub fn name(&self) -> String {
        self.name.to_lowercase()
    }
}

pub struct InstructionVariant<B: Bus, I, AM, O>(PhantomData<(B, I, AM, O)>)
where
    O: operand::Operand,
    I: Instruction<B, O>,
    AM: Addressing<B, O>;

impl<B: Bus, I, AM, O> InstructionVariant<B, I, AM, O>
where
    O: operand::Operand,
    I: Instruction<B, O>,
    AM: Addressing<B, O>,
{

    fn exec(cpu: &mut Cpu6502<B>) {
        let op = AM::resolve(cpu);
        I::exec(cpu, op);
    }

    pub fn entry() -> InstructionEntry<B> {
        InstructionEntry::new::<I, O>(Self::exec)
    }
}

