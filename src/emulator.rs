use std::string::String;

use cpu::CPU;
use crate::cpu;

use cartridge::Cartridge;
use crate::cartridge;

use main_bus::MainBus;
use crate::main_bus;

use mapper::Mapper;
use crate::mapper;

// Assume that the CPU, Cartridge, MainBus, and Mapper types are defined in other modules.

pub struct Emulator<'a> {
    pub m_cpu: CPU<'a>,
    pub m_bus: MainBus
}

impl<'a> Emulator<'a> {

    pub fn run(&mut self, rom_path: String) {
        let mut cartridge: Cartridge = Cartridge::new();
        if let Err(_error) = cartridge.load_from_file(&rom_path) {
            eprintln!("Unable to load ROM from file: {}", rom_path);
            return;
        } 

        let mut mapper = Mapper::new();
        mapper.load(cartridge);
        // Add code for PPU bus mapper setup here if necessary.

        self.m_bus.set_mapper(mapper);

        self.m_cpu.reset();
    }
}

