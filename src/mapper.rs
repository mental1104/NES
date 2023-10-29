/*
 * @Date: 2023-10-29 22:07:46
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 23:01:54
 */

 use cartridge::Cartridge;
use crate::cartridge;

pub struct Mapper {
    cartridge: Cartridge,
    one_bank: bool,
    uses_character_ram: bool,
    character_ram: Vec<u8>
}

impl Mapper {
    pub fn new() -> Self {
        Mapper {
            cartridge: Cartridge::new(),
            one_bank: false,
            uses_character_ram: false,
            character_ram: Vec::new()
        }
    }

    pub fn load(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge; 

        if self.cartridge.get_rom().len() == 0x4000 {
            self.one_bank = true;
        } else {
            self.one_bank = false;
        }

        if self.cartridge.get_vrom().len() == 0 {
            self.uses_character_ram = true;
            self.character_ram.resize(0x2000, 0);
            println!("Uses character ram"); 
        } else {
            println!("Using CHR-ROM");
            self.uses_character_ram = false;
        }
    }

    pub fn write_prg(&mut self, addr: u16, value: u8) {
        println!("ROM memory write attempt at {}  to set {}", addr, value);
    }

    pub fn read_prg(&mut self, addr: u16) -> u8 {
        if !self.one_bank {
            let index = (addr - 0x8000) as usize;
            return self.cartridge.get_rom()[index];
        } else {
            let index = ((addr - 0x8000) & 0x3fff) as usize;
            return self.cartridge.get_rom()[index];
        }
    }

    pub fn write_chr(&mut self, addr: u16, value: u8){
        if self.uses_character_ram {
            self.character_ram[addr as usize] = value
        } else {
            println!("Read-only CHR memory write attempt at {}", addr);
        }
    }

    pub fn read_chr(&mut self, addr: u16) -> u8 {
        if self.uses_character_ram {
            return self.character_ram[addr as usize];
        } else {
            return self.cartridge.get_vrom()[addr as usize];
        }
    }

    pub fn has_extended_ram(&mut self) -> bool {
        self.cartridge.has_extended_ram()
    }

}