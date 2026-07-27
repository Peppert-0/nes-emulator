use std::{fs::File, io::Read};

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr: ChrMemory,
    mapper: Box<dyn Mapper>,
}

pub enum ChrMemory {
    Rom(Vec<u8>),
    Ram(Vec<u8>),
}

pub trait Mapper {
    fn cpu_read(&self, address: u16) -> Option<usize>;
    fn cpu_write(&self, address: u16) -> Option<usize>;
}

pub struct Nrom {}

impl Mapper for Nrom {
    fn cpu_read(&self, address: u16) -> Option<usize> {
        match address {
            0x8000..=0xBFFF => {
                Some(0)
            }
            0xC000..=0xFFFF => {
                Some(0)
            }
            _ => {
                None
            }
        }
    }
    fn cpu_write(&self, address: u16) -> Option<usize> {
        match address {
            0x8000..=0xBFFF => {
                Some(0)
            }
            0xC000..=0xFFFF => {
                Some(0)
            }
            _ => {
                None
            }
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
                Box::new(Nrom{})
            }
            _ => {
                panic!("Unsupported or invalid mapper");
            }
        };

        Self { prg_rom, chr: ChrMemory::Rom(chr_rom), mapper: mapper }
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
