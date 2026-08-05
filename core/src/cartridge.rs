use std::{fs::File, io::Read};

pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr: ChrMemory,
    pub mapper: Box<dyn Mapper>,
}

pub enum ChrMemory {
    Rom(Vec<u8>),
    Ram(Vec<u8>),
}

pub trait Mapper {
    fn cpu_read(&self, prg_rom: &Vec<u8>, address: u16) -> u8;
    fn ppu_read(&self, chr: &ChrMemory, vram: &[u8; 2048], address: u16) -> u8;
}

pub struct Nrom {
    header: InesHeader,
}

impl Mapper for Nrom {
    fn cpu_read(&self, prg_rom: &Vec<u8>, address: u16) -> u8 {
        match address {
            0x8000..=0xFFFF => {
                let offset = address - 0x8000;
                if self.header.prg_rom_mult == 1 {
                    prg_rom[(offset & 0x3FFF) as usize]
                } else {
                    prg_rom[offset as usize]
                }
            }
            _ => {
                panic!("Invalid address range")
            }
        }
    }
    fn ppu_read(&self, chr: &ChrMemory, vram: &[u8; 2048], address: u16) -> u8 {
        if let ChrMemory::Rom(chr_rom) = chr {
            match address {
                0x0000..=0x1FFF => {
                    chr_rom[address as usize]
                }
                0x2000..=0x2FFF => {
                    let offset = address - 0x2000;
                    if self.header.vertical_mirroring {
                        match offset {
                            0x0000..=0x07FF => vram[offset as usize],
                            0x0800..=0x0FFF => vram[(offset as usize) & 0x07FF],
                            _ => 0,
                        }
                    } else {
                        match offset {
                            0x0000..=0x03FF => vram[offset as usize],
                            0x0400..=0x07FF => vram[(offset as usize) & 0x3FF],
                            0x0800..=0x0BFF => vram[offset as usize],
                            0x0C00..=0x0FFF => vram[(offset as usize) & 0x3FF],
                            _ => 0,
                        }
                    }
                }
                0x3000..=0x3F1F => {
                    let offset = address - 0x3000;
                    self.ppu_read(chr, vram, offset)
                }
                _ => 0
            }
        } else {
            0
        }
    } 
}

pub struct InesHeader {
    prg_rom_mult: u8,
    chr_rom_mult: u8,
    mapper: u8,
    vertical_mirroring: bool,
    trainer: bool,
}

impl Cartridge {
    pub fn load_from_file(rom: &mut File) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        rom.read_to_end(&mut bytes);
        let header = InesHeader::parse(&mut bytes);

        let mut prg_start = 0x0010u16;
        if header.trainer {prg_start += 512};
        let prg_end = prg_start + (0x4000 * u16::from(header.prg_rom_mult));
        let prg_rom: Vec<u8> = bytes[prg_start as usize..prg_end as usize].to_vec();
        let chr_end = prg_end + (0x2000 * u16::from(header.chr_rom_mult));
        let chr_rom: Vec<u8> = bytes[prg_end as usize..chr_end as usize].to_vec();

        let mapper = match header.mapper {
            0 => {
                Box::new(Nrom{header})
            }
            _ => {
                panic!("Unsupported or invalid mapper");
            }
        };

        Self { prg_rom, chr: ChrMemory::Rom(chr_rom), mapper: mapper }
    }
    pub fn cpu_read(&self, address: u16) -> u8 {
        self.mapper.cpu_read(&self.prg_rom, address)
    }
    pub fn cpu_write(&mut self, address: u16, value: u8) {
    }
}

impl InesHeader {
    pub fn parse(bytes: &[u8]) -> Self {
        Self {
            prg_rom_mult: bytes[4],
            chr_rom_mult: bytes[5],
            mapper: (bytes[6] >> 4) | (bytes[7] & 0xF0),
            vertical_mirroring: (bytes[6] & 1) != 0,
            trainer: (bytes[6] & (1 << 2)) != 0,
        }
    }
}
