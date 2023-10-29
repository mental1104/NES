/*
 * @Date: 2023-10-29 11:35:19
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 11:58:37
 */


use chip::Byte;
use chip::Address;
use crate::chip;

pub struct MainBus {
    m_ram: [Byte; 0x800],
}

impl MainBus {
    pub fn new() -> MainBus {
        MainBus {
            m_ram: [0; 0x800],
        }
    }

    pub fn read(&self, addr: Address) -> Byte {
        if addr < 0x2000 {
            return self.m_ram[(addr & 0x7FF) as usize];
        }
        0
    }

    pub fn write(&mut self, addr: Address, val: Byte) {
        if addr < 0x2000 {
            self.m_ram[(addr & 0x7FF) as usize] = val;
        }
    }
}
