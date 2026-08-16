const GREYS: [Rgba; 4] = [
    Rgba {
        R: 0x00,
        G: 0x00,
        B: 0x00,
        A: 0xFF,
    },
    Rgba {
        R: 0x55,
        G: 0x55,
        B: 0x55,
        A: 0xFF,

    },
    Rgba {
        R: 0xAA,
        G: 0xAA,
        B: 0xAA,
        A: 0xFF,

    },
    Rgba {
        R: 0xFF,
        G: 0xFF,
        B: 0xFF,
        A: 0xFF,

    },
];

#[derive(Clone, Copy)]
pub struct Rgba {
    R: u8,
    G: u8,
    B: u8,
    A: u8,
}

pub struct Bitmap {
    pixels: Vec<Rgba>,
    width: u32,
    height: u32,
}

impl Bitmap {
    pub fn new(pixels: Vec<Rgba>, width: u32, height: u32) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }
    pub fn from_pattern_table(chr: &[u8], table: u8) -> Self {
        let mut pixels: Vec<Rgba> = vec![];
        for tile_y in 0..16 {
            for tile_x in 0..16 {
                let base = if table == 0 {0} else {0x1000};
                let offset = base + ((tile_x + tile_y * 16) * 16) as usize;
                let tile = &chr[offset..offset + 16];
                let plane_0 = &tile[0..8];
                let plane_1 = &tile[8..16];

                for row in 0..8 {
                    let mut plane_0_byte = plane_0[row];
                    let mut plane_1_byte = plane_1[row];

                    for col in 0..8 {
                        let colour = ((plane_1_byte & 0x80) >> 6) | ((plane_0_byte & 0x80) >> 7);
                        let pixel = GREYS[colour as usize];

                        let screen_x = tile_x * 8 + col;
                        let screen_y = tile_y * 8 + row;
                        let index = screen_y * 128 + screen_x;
                        pixels[index] = pixel;

                        plane_0_byte <<= 1;
                        plane_1_byte <<= 1;
                    }
                }
            }
        }
        Self::new(pixels, 128, 128)
    }
}
