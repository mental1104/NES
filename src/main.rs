/*
 * @Date: 2023-10-29 10:59:12
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 23:21:47
 */
mod main_bus;
mod chip;
mod cpu;
mod cpu_opcodes;
mod cartridge;
mod emulator;
mod mapper;
use emulator::Emulator;
use main_bus::MainBus;
use cpu::CPU;
use cartridge::Cartridge;

use std::env;

fn main() {
    let mut main_bus = MainBus::new();
    let mut tmp_bus = MainBus::new();
    let mut emulator = Emulator {
        m_cpu: CPU::new(&mut main_bus),
        m_bus: tmp_bus
    };
    let args: Vec<String> = env::args().collect();

    // 第一个参数是程序的名称
    let program_name = &args[0];
    println!("Program name: {}", program_name);

    let argv = args[1].to_string();
    println!("rom name: {}", argv);
    emulator.run(argv);
}
