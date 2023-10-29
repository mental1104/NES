/*
 * @Date: 2023-10-29 10:59:12
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 11:57:54
 */
use std::io;
mod main_bus;
mod chip;
mod cpu;

use main_bus::MainBus;
use cpu::CPU;

fn main() {
    let mut bus = MainBus::new();
    let mut test_cpu = CPU::new(&mut bus);

    test_cpu.reset();
    println!("After reset the PC is: {}", test_cpu.r_pc);

    // CPU取指解码执行，目前仅实现取指操作
    test_cpu.step();
    println!("After Step the PC is: {}", test_cpu.r_pc);

    // 等待用户输入以保持控制台窗口打开
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
