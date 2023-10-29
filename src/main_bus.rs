/*
 * @Date: 2023-10-29 11:35:19
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 23:25:25
 */


use chip::Byte;
use chip::Address;
use mapper::Mapper;
use crate::mapper;
use crate::cartridge::Cartridge;
use crate::chip;

pub struct MainBus {
    m_ram: [Byte; 0x800],
    m_ext_ram: Vec<u8>,
    cartridge: Cartridge,
    mapper: Mapper
}

impl MainBus {
    pub fn new() -> MainBus {
        MainBus {
            m_ram: [0; 0x800],
            m_ext_ram: Vec::new(),
            cartridge: Cartridge::new(),
            mapper: Mapper::new()
        }
    }

    pub fn load(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge; 
    }

    pub fn set_mapper(&mut self, mapper: Mapper) -> bool {
        self.mapper = mapper;

        if self.mapper.has_extended_ram() {
            self.m_ext_ram.resize(0x2000, 0);
        }

        return true;
    }

    pub fn read(&self, addr: Address) -> Byte {
        if addr < 0x2000 {
            return self.m_ram[(addr & 0x7FF) as usize];
        }

        if addr >= 0x8000 {
            // let val = self.cartridge.get_rom()[(addr - 0x8000) as usize];
            // println!("MainBus Read a Byte: {:02X}", val);
            // return val;
        }

        0
    }

    pub fn write(&mut self, addr: Address, val: Byte) {
        if addr < 0x2000 {
            self.m_ram[(addr & 0x7FF) as usize] = val;
        }
    }
}
