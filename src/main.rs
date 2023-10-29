/*
 * @Date: 2023-10-29 10:59:12
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 15:51:34
 */
mod main_bus;
mod chip;
mod cpu;
mod cpu_opcodes;
mod cartridge;
use main_bus::MainBus;
use cpu::CPU;
use cartridge::Cartridge;

fn main() {
    let mut cartridge = Cartridge::new();
    if let Err(error) = cartridge.load_from_file("./rom/example.nes") {
        println!("Error: {}", error);
        return;
    }
    let mut bus: MainBus = MainBus::new();
    MainBus::load(&mut bus, cartridge);
    let mut test_cpu = CPU::new(&mut bus);

    test_cpu.reset();

    println!("[+]After reset, the PC is: {:X}", test_cpu.r_pc);

    // CPU取指解码执行
    let mut cycle = 4;
    while cycle > 0 {
        test_cpu.step();
        cycle -= 1;
    }
    
    let acc_val = test_cpu.r_a;
    println!("[+]执行4+2的操作后，ACC寄存器的值为: {}", acc_val);
}
