pub struct Memory {
    ram: [u8; 0x0800],
    ppu: [u8; 0x0008],
    apu_n_io: [u8; 0x0018],

    cartridge_map: [u8; 0xBFE0]
}

impl Default for Memory {
    fn default() -> Self {
        let ram: [u8; 0x0800] = [0; 0x0800];
        let ppu: [u8; 0x0008] = [0; 0x0008];
        let apu_n_io: [u8; 0x0018] = [0; 0x0018];
        let cartridge_map: [u8; 0xBFE0] = [0; 0xBFE0];

        Memory { ram, ppu, apu_n_io, cartridge_map }
    }
}

impl Memory {
    pub fn load_cartridge_rom(&mut self, data: [u8; 0x8000]) {
        for i in 0..0x8000 {
            self.write(0x8000 + i, data[i as usize]);
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if (0..=0x1FFF).contains(&addr) {
            return self.ram[(addr % 0x0800) as usize];
        } else if (0x2000..=0x3FFF).contains(&addr) {
            return self.ppu[((addr - 0x2000) % 0x0008) as usize]
        } else if (0x4000..=0x4017).contains(&addr) {
            return self.apu_n_io[(addr - 0x4000) as usize]
        } else if (0x4020..=0xFFFF).contains(&addr) {
            return self.cartridge_map[(addr - 0x4020) as usize];
        }

        println!("Attempted to read address {} which is unmapped.", addr);

        return 0;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if (0..=0x1FFF).contains(&addr) {
            self.ram[(addr % 0x0800) as usize] = data;
        } else if (0x2000..=0x3FFF).contains(&addr) {
            self.ppu[((addr - 0x2000) % 0x0008) as usize] = data;
        } else if (0x4000..=0x4017).contains(&addr) {
            self.apu_n_io[(addr - 0x4000) as usize] = data;
        } else if (0x4020..=0xFFFF).contains(&addr) {
            self.cartridge_map[(addr - 0x4020) as usize] = data;
        }

        println!("Attempted to write to address {} data {} which is unmapped.", addr, data);
    }
}