/*
 * @Date: 2023-10-29 11:01:05
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 15:03:48
 */
use chip::Byte;
use chip::Address;
use crate::chip;

use cpu_opcodes::RESET_VECTOR;
use cpu_opcodes::BranchOnFlag;
use cpu_opcodes::BRANCH_INSTRUCTION_MASK;
use cpu_opcodes::BRANCH_INSTRUCTION_MASK_RESULT;
use cpu_opcodes::BRANCH_ON_FLAG_SHIFT;
use cpu_opcodes::INSTRUCTION_MODE_MASK;
use cpu_opcodes::OPERATION_MASK;
use cpu_opcodes::OPERATION_SHIFT;
use cpu_opcodes::ADDR_MODE_MASK;
use cpu_opcodes::ADDR_MODE_SHIFT;
use cpu_opcodes::OperationImplied;
use cpu_opcodes::Operation1;
use cpu_opcodes::Operation2;
use cpu_opcodes::AddressingMode1;
use cpu_opcodes::AddressingMode2;
use crate::cpu_opcodes;

use main_bus::MainBus;
use crate::main_bus;
 
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
        self.m_cycles += 1;
    
        if self.m_skip_cycles > 1 {
            self.m_skip_cycles -= 1;
            return;
        }
    
        self.m_skip_cycles = 0;
        /* 生成程序状态字 */
        /*
        let psw = (self.f_N as u8) << 7 |
                  (self.f_V as u8) << 6 |
                  1 << 5 |
                  (self.f_D as u8) << 3 |
                  (self.f_I as u8) << 2 |
                  (self.f_Z as u8) << 1 |
                  (self.f_C as u8);
        */
        let opcode = self.bus.read(self.r_pc);
        self.r_pc += 1;
        let cycle_length = cpu_opcodes::OPERATION_CYCLES[opcode as usize];
        if cycle_length != 0 && (self.execute_implied(opcode) || self.execute_branch(opcode)
            || self.execute_type1(opcode) || self.execute_type2(opcode))
        {
            self.m_skip_cycles += cycle_length;
        } else {
            eprintln!("Unrecognized opcode: 0x{:02X}", opcode);
        }
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

    pub fn set_zn(&mut self, value: u8) {
        self.f_z = value == 0;
        self.f_n = (value & 0x80) != 0;
    }
    
    pub fn set_page_crossed(&mut self, a: Address, b: Address, inc: u32) {
        // Page is determined by the high byte
        if (a & 0xff00) != (b & 0xff00) {
            self.m_skip_cycles += inc;
        }
    }

    pub fn execute_implied(&mut self, opcode: Byte) -> bool {
        match OperationImplied::from(opcode) {
            OperationImplied::NOP => (),
            OperationImplied::JSR => {
                // Jump to new location, saving Return Address
                // Push address of next instruction - 1, thus r_PC + 1 instead of r_PC + 2
                // since r_PC and r_PC + 1 are the address of the subroutine
                self.push_stack(((self.r_pc + 1) >> 8) as u8);
                self.push_stack((self.r_pc + 1) as u8);
                self.r_pc = self.read_address(self.r_pc);
            },
            OperationImplied::RTS => {
                // Return from Subroutine
                self.r_pc = self.pull_stack() as Address;
                self.r_pc |= (self.pull_stack() as Address) << 8;
                self.r_pc += 1;
            },
            OperationImplied::RTI => {
                let flags = self.pull_stack() as u8;
                self.f_n = (flags & 0x80) != 0;
                self.f_v = (flags & 0x40) != 0;
                self.f_d = (flags & 0x8) != 0;
                self.f_i = (flags & 0x4) != 0;
                self.f_z = (flags & 0x2) != 0;
                self.f_c = (flags & 0x1) != 0;
                self.r_pc = self.pull_stack() as Address;
                self.r_pc |= (self.pull_stack() as Address) << 8;
            },
            OperationImplied::JMP => {
                self.r_pc = self.read_address(self.r_pc);
            },
            OperationImplied::JMPI => {
                let location = self.read_address(self.r_pc);
                // 6502 has a bug such that when the vector of an indirect address begins at the last byte of a page,
                // the second byte is fetched from the beginning of that page rather than the beginning of the next
                // Recreating here:
                let page = location & 0xff00;
                self.r_pc = self.bus.read(location) as Address | 
                            (self.bus.read(page | ((location + 1) & 0xff)) as Address) << 8;
            },
            OperationImplied::PHP => {
                let flags = (self.f_n as u8) << 7 |
                            (self.f_v as u8) << 6 |
                            1 << 5 | // supposed to always be 1
                            1 << 4 | // PHP pushes with the B flag as 1, no matter what
                            (self.f_d as u8) << 3 |
                            (self.f_i as u8) << 2 |
                            (self.f_z as u8) << 1 |
                            (self.f_c as u8);
                self.push_stack(flags);
            },
            OperationImplied::PLP => {
                let flags = self.pull_stack() as u8;
                self.f_n = (flags & 0x80) != 0;
                self.f_v = (flags & 0x40) != 0;
                self.f_d = (flags & 0x8) != 0;
                self.f_i = (flags & 0x4) != 0;
                self.f_z = (flags & 0x2) != 0;
                self.f_c = (flags & 0x1) != 0;
            },
            OperationImplied::PHA => {
                self.push_stack(self.r_a);
            },
            OperationImplied::PLA => {
                self.r_a = self.pull_stack() as Byte;
                self.set_zn(self.r_a);
            },
            OperationImplied::DEY => {
                self.r_y = self.r_y.wrapping_sub(1);
                self.set_zn(self.r_y);
            },
            OperationImplied::DEX => {
                self.r_x = self.r_x.wrapping_sub(1);
                self.set_zn(self.r_x);
            },
            OperationImplied::TAY => {
                self.r_y = self.r_a;
                self.set_zn(self.r_y);
            },
            OperationImplied::INY => {
                self.r_y = self.r_y.wrapping_add(1);
                self.set_zn(self.r_y);
            },
            OperationImplied::INX => {
                self.r_x = self.r_x.wrapping_add(1);
                self.set_zn(self.r_x);
            },
            OperationImplied::CLC => {
                self.f_c = false;
            },
            OperationImplied::SEC => {
                self.f_c = true;
            },
            OperationImplied::CLI => {
                self.f_i = false;
            },
            OperationImplied::SEI => {
                self.f_i = true;
            },
            OperationImplied::CLD => {
                self.f_d = false;
            },
            OperationImplied::SED => {
                self.f_d = true;
            },
            OperationImplied::TYA => {
                self.r_a = self.r_y;
                self.set_zn(self.r_a);
            },
            OperationImplied::CLV => {
                self.f_v = false;
            },
            OperationImplied::TXA => {
                self.r_a = self.r_x;
                self.set_zn(self.r_a);
            },
            OperationImplied::TXS => {
                self.r_sp = self.r_x;
            },
            OperationImplied::TAX => {
                self.r_x = self.r_a;
                self.set_zn(self.r_x);
            },
            OperationImplied::TSX => {
                self.r_x = self.r_sp;
                self.set_zn(self.r_x);
            },
            OperationImplied::Unknown => {
                return false;
            }
        }
        true
    }
    
    pub(crate) fn execute_branch(&mut self, opcode: u8) -> bool {
        if (opcode & BRANCH_INSTRUCTION_MASK) == BRANCH_INSTRUCTION_MASK_RESULT {
            // branch is initialized to the condition required (for the flag specified later)
            let branch = opcode & BRANCH_INSTRUCTION_MASK;
            let branch_flag: bool;
            // set branch to true if the given condition is met by the given flag
            // We use xnor here, it is true if either both operands are true or false
            match BranchOnFlag::from(opcode >> BRANCH_ON_FLAG_SHIFT) {
                //BranchOnFlag::Negative => branch = !(branch ^ self.f_n)
                BranchOnFlag::Negative => {
                    let flag_n = self.f_n as u8;
                    branch_flag = !(branch ^ flag_n != 0);
                }
                BranchOnFlag::Overflow => {
                    let flag_v = self.f_v as u8;
                    branch_flag = !(branch ^ flag_v != 0);
                }
                BranchOnFlag::Carry => {
                    let flag_c = self.f_c as u8;
                    branch_flag = !(branch ^ flag_c != 0);
                }
                BranchOnFlag::Zero => {
                    let flag_z = self.f_z as u8;
                    branch_flag = !(branch ^ flag_z != 0);
                }
            }
    
            if branch_flag {
                let offset = self.bus.read(self.r_pc as Address) as i8;
                self.m_skip_cycles += 1;
                let new_pc: u16 = (self.r_pc as i32 + offset as i32) as Address;
                self.set_page_crossed(self.r_pc, new_pc, 2);
                self.r_pc = new_pc;
            } else {
                self.r_pc += 1;
            }
            return true;
        }
        false
    }

    fn execute_type1(&mut self, opcode: u8) -> bool {
        if (opcode & INSTRUCTION_MODE_MASK) == 0x1 {
            let mut location: Address;
            let op = Operation1::from(((opcode & OPERATION_MASK) >> OPERATION_SHIFT) as u8);
            let addressing_mode = AddressingMode1::from(((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT) as u8);
    
            match addressing_mode {
                AddressingMode1::IndexedIndirectX => {
                    let zero_addr = self.r_x.wrapping_add(self.bus.read(self.r_pc));
                    let read_addr1: Address = (zero_addr & 0xFF) as Address;
                    let read_addr2: Address = ((zero_addr + 1) & 0xFF) as Address;
                    location = (self.bus.read(read_addr1) as Address)
                        | ((self.bus.read(read_addr2) as Address) << 8);
                    self.r_pc += 1;
                }
                AddressingMode1::ZeroPage => {
                    location = self.bus.read(self.r_pc) as Address;
                    self.r_pc += 1;
                }
                AddressingMode1::Immediate => {
                    location = self.r_pc;
                    self.r_pc += 1;
                }
                AddressingMode1::Absolute => {
                    location = self.read_address(self.r_pc);
                    self.r_pc += 2;
                }
                AddressingMode1::IndirectY => {
                    let zero_addr = self.bus.read(self.r_pc);
                    let read_addr1: Address = (zero_addr & 0xFF) as Address;
                    let mut read_addr2: Address = ((zero_addr + 1) & 0xFF) as Address;
                    read_addr2 = read_addr2 << 8;
                    location = (self.bus.read(read_addr1) as Address)
                        | ((self.bus.read(read_addr2) as Address) << 8);
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location.wrapping_add(self.r_y.into()), 1);
                    }
                    location = location.wrapping_add(self.r_y.into());
                    self.r_pc += 1;
                }
                AddressingMode1::IndexedX => {
                    let zero_addr = (self.bus.read(self.r_pc) as u16)
                        .wrapping_add(self.r_x as u16);
                    location = zero_addr as Address & 0xFF;
                    self.r_pc += 1;
                }
                AddressingMode1::AbsoluteY => {
                    location = self.read_address(self.r_pc);
                    self.r_pc += 2;
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location.wrapping_add(self.r_y.into()), 1);
                    }
                    location = location.wrapping_add(self.r_y.into());
                }
                AddressingMode1::AbsoluteX => {
                    location = self.read_address(self.r_pc);
                    self.r_pc += 2;
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location.wrapping_add(self.r_x.into()), 1);
                    }
                    location = location.wrapping_add(self.r_x.into());
                }
            }
    
            match op {
                Operation1::ORA => {
                    self.r_a |= self.bus.read(location);
                    self.set_zn(self.r_a);
                }
                Operation1::AND => {
                    self.r_a &= self.bus.read(location);
                    self.set_zn(self.r_a);
                }
                Operation1::EOR => {
                    self.r_a ^= self.bus.read(location);
                    self.set_zn(self.r_a);
                }
                Operation1::ADC => {
                    let operand = self.bus.read(location);
                    let sum = self.r_a as u16 + operand as u16 + (self.f_c as u16);
                    self.f_c = (sum & 0x100) != 0;
                    self.f_v = (self.r_a ^ sum as u8) & (operand ^ sum as u8) & 0x80 != 0;
                    self.r_a = sum as u8;
                    self.set_zn(self.r_a);
                }
                Operation1::STA => {
                    self.bus.write(location, self.r_a);
                }
                Operation1::LDA => {
                    self.r_a = self.bus.read(location);
                    self.set_zn(self.r_a);
                }
                Operation1::SBC => {
                    let subtrahend = self.bus.read(location);
                    let diff = self.r_a as i16 - subtrahend as i16 - !(self.f_c as i16);
                    self.f_c = !(diff & 0x100 != 0);
                    self.f_v = (self.r_a ^ diff as u8) & (!subtrahend ^ diff as u8) & 0x80 != 0;
                    self.r_a = diff as u8;
                    self.set_zn(diff as u8);
                }
                Operation1::CMP => {
                    let diff = self.r_a as i16 - self.bus.read(location) as i16;
                    self.f_c = !(diff & 0x100 != 0);
                    self.set_zn(diff as u8);
                }
            }
            return true;
        }
        false
    }

    pub fn execute_type2(&mut self, opcode: u8) -> bool {
        if (opcode & INSTRUCTION_MODE_MASK) == 2 {
            let mut location: Address = 0;
            let op = Operation2::from((opcode & OPERATION_MASK) >> OPERATION_SHIFT);
            let addr_mode =
                AddressingMode2::from((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT);
            match addr_mode {
                AddressingMode2::Immediate_ => location = self.r_pc,
                AddressingMode2::ZeroPage_ => location = self.bus.read(self.r_pc) as Address,
                AddressingMode2::Accumulator => {}
                AddressingMode2::Absolute_ => {
                    location = self.read_address(self.r_pc);
                }
                AddressingMode2::Indexed => {
                    location = self.bus.read(self.r_pc) as Address;
                    let index: Byte = if op == Operation2::LDX || op == Operation2::STX {
                        self.r_y
                    } else {
                        self.r_x
                    };
                    location = (location.wrapping_add(index.into()) & 0xFF) as Address;
                }
                AddressingMode2::AbsoluteIndexed => {
                    location = self.read_address(self.r_pc);
                    let index: Byte = if op == Operation2::LDX || op == Operation2::STX {
                        self.r_y
                    } else {
                        self.r_x
                    };
                    self.set_page_crossed(location, location.wrapping_add(index.into()), 1);
                    location = location.wrapping_add(index.into());
                }
            }
    
            let mut operand: u16;
            match op {
                Operation2::ASL | Operation2::ROL => {
                    if addr_mode == AddressingMode2::Accumulator {
                        let prev_c = self.f_c;
                        self.f_c = (self.r_a & 0x80) != 0;
                        self.r_a <<= 1;
                        self.r_a |= (prev_c && (op == Operation2::ROL)) as Byte;
                        self.set_zn(self.r_a);
                    } else {
                        let prev_c = self.f_c;
                        operand = self.bus.read(location) as u16;
                        self.f_c = (operand & 0x80) != 0;
                        operand = (operand << 1 | (prev_c && (op == Operation2::ROL)) as u16) & 0xFF;
                        self.set_zn(self.r_a);
                        self.bus.write(location, operand as Byte);
                    }
                }
                Operation2::LSR | Operation2::ROR => {
                    if addr_mode == AddressingMode2::Accumulator {
                        let prev_c = self.f_c;
                        self.f_c = (self.r_a & 1) != 0;
                        self.r_a >>= 1;
                        self.r_a |= ((prev_c && (op == Operation2::ROR)) as Byte) << 7;
                        self.set_zn(self.r_a);
                    } else {
                        let prev_c = self.f_c;
                        operand = self.bus.read(location) as u16;
                        self.f_c = (operand & 1) != 0;
                        operand = (operand >> 1 | ((prev_c && (op == Operation2::ROR)) as u16) << 7) & 0xFF;
                        self.set_zn(self.r_a);
                        self.bus.write(location, operand as Byte);
                    }
                }
                Operation2::STX => {
                    self.bus.write(location, self.r_x);
                }
                Operation2::LDX => {
                    self.r_x = self.bus.read(location);
                    self.set_zn(self.r_x);
                }
                Operation2::DEC => {
                    let mut tmp = self.bus.read(location);
                    tmp = tmp.wrapping_sub(1);
                    self.set_zn(tmp);
                    self.bus.write(location, tmp as Byte);
                }
                Operation2::INC => {
                    let mut tmp = self.bus.read(location);
                    tmp = tmp.wrapping_add(1);
                    self.set_zn(tmp);
                    self.bus.write(location, tmp as Byte);
                }
            }
            return true;
        }
        false
    }
}