/*
 * @Date: 2023-10-29 15:10:56
 * @Author: mental1104 mental1104@gmail.com
 * @LastEditors: mental1104 mental1104@gmail.com
 * @LastEditTime: 2023-10-29 23:37:29
 */
use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    name_table_mirroring: u8,
    mapper_number: u8,
    extended_ram: bool
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            name_table_mirroring: 0,
            mapper_number: 0,
            extended_ram: false
        }
    }

    pub fn get_rom(&self) -> &Vec<u8> {
        &self.prg_rom
    }

    pub fn get_vrom(&self) -> &Vec<u8> {
        &self.chr_rom
    }

    pub fn has_extended_ram(&self) -> bool {
        self.extended_ram
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let mut rom_file = File::open(path).map_err(|e| format!("Could not open ROM file: {}", e))?;
        let mut header = [0; 0x10];

        rom_file.read_exact(&mut header).map_err(|e| format!("Reading iNES header failed: {}", e))?;

        if &header[0..4] != b"NES\x1A" {
            return Err("Not a valid iNES image.".to_string());
        }

        let prg_rom_banks = header[4] as u16;
        println!("16KB PRG-ROM Banks: {}", prg_rom_banks);
        if prg_rom_banks == 0 {
            return Err("ROM has no PRG-ROM banks. Loading ROM failed.".to_string());
        }

        let chr_rom_banks = header[5] as u16;
        println!("8KB PRG-ROM Banks: {}", chr_rom_banks);

        self.name_table_mirroring = header[6] & 0xB;
        self.mapper_number = ((header[6] >> 4) & 0xf) | (header[7] & 0xf0);
        println!("Mapper number {}", self.mapper_number);
        self.extended_ram = header[6] & 0x2 != 0;
        println!("Extended (CPU) RAM: {}", self.extended_ram);
        
        if header[6] & 0x4 != 0 {
            return Err("Trainer is not supported.".to_string());
        }

        if (header[0xA] & 0x3) == 0x2 || (header[0xA] & 0x1) != 0 {
            return Err("PAL ROM not supported.".to_string());
        }

        println!("ROM is NTSC compatible.");

        self.prg_rom.resize((0x4000 * prg_rom_banks) as usize, 0);
        rom_file
            .read_exact(&mut self.prg_rom)
            .map_err(|e| format!("Reading PRG-ROM from image file failed: {}", e))?;


        if chr_rom_banks > 0 {
            self.chr_rom.resize((0x2000 * chr_rom_banks) as usize, 0);
            rom_file
                .read_exact(&mut self.chr_rom)
                .map_err(|e| format!("Reading CHR-ROM from image file failed: {}", e))?;
        } else {
            println!("Cartridge with CHR-RAM.");
        }

        Ok(())
    }
}