use crate::bus::Bus;

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
        assert!(
            addr as usize + rom.len() <= MEM_64KB,
            "ROM too large to fit in memory"
        );
        self.0[addr as usize..addr as usize + rom.len()].copy_from_slice(rom);
    }
}

impl Bus for Memory {
    fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value;
    }
}
