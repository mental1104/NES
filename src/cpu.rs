/*
 * @Date: 2023-10-29 11:01:05
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 12:08:56
 */
use chip::Byte;
use chip::Address;
use crate::chip;
use main_bus::MainBus;
use crate::main_bus;

pub const RESET_VECTOR: u16 = 0xfffc;
 
pub struct CPU<'a> {
    bus: &'a mut MainBus,
    pub r_a: u8,
    pub r_x: u8,
    pub r_y: u8,
    pub r_sp: u8,
    pub r_pc: Address,
    pub m_skip_cycles: u32,
    pub m_cycles: u32,
    pub f_i: bool,
    pub f_c: bool,
    pub f_d: bool,
    pub f_n: bool,
    pub f_v: bool,
    pub f_z: bool,
}

impl<'a> CPU<'a> {
    pub fn new(mem: &'a mut MainBus) -> Self {
        CPU {
            bus: mem,
            r_a: 0,
            r_x: 0,
            r_y: 0,
            r_sp: 0xfd,
            r_pc: 0,
            m_skip_cycles: 0,
            m_cycles: 0,
            f_i: true,
            f_c: false,
            f_d: false,
            f_n: false,
            f_v: false,
            f_z: false,
        }
    }

    pub fn reset(&mut self) {
        let addr = self.read_address(RESET_VECTOR);
        self.reset_with_start_addr(addr);
    }

    pub fn step(&mut self) {
        // Byte opcode = self.bus.read(self.r_pc);
        // 待实现解码执行
        // self.r_pc += 1;
    }

    pub fn read_address(&mut self, addr: Address) -> Address {
        let low_byte = self.bus.read(addr) as u16;
        let high_byte = (self.bus.read(addr + 1) as u16) << 8;
        low_byte | high_byte
    }

    pub fn reset_with_start_addr(&mut self, start_addr: Address) {
        self.m_skip_cycles = 0;
        self.m_cycles = 0;
        self.r_a = 0;
        self.r_x = 0;
        self.r_y = 0;
        self.f_i = true;
        self.f_c = false;
        self.f_d = false;
        self.f_n = false;
        self.f_v = false;
        self.f_z = false;
        self.r_pc = start_addr;
        self.r_sp = 0xfd; // documented startup state
    }

    pub fn push_stack(&mut self, val: Byte) {
        self.bus.write(0x100 | self.r_sp as Address, val);
        self.r_sp = self.r_sp.wrapping_sub(1);
    }

    pub fn pull_stack(&mut self) -> Byte {
        self.r_sp = self.r_sp.wrapping_add(1);
        self.bus.read(0x100 | self.r_sp as Address)
    }
}