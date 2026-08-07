const GREYS: [u32; 4] = [
    0x000000FF,
    0x555555FF,
    0xAAAAAAFF,
    0xFFFFFFFF,
];

pub fn draw_pattern_framebuffer(chr: &[u8]) -> Vec<u32> {
    let mut framebuffer = vec![0; 128 * 128];
    for tile_y in 0..16 {
        for tile_x in 0..16 {
            let offset = ((tile_x + tile_y * 16) * 16) as usize;
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
                    framebuffer[index] = pixel;

                    plane_0_byte <<= 1;
                    plane_1_byte <<= 1;
                }
            }
        }
    }
    framebuffer
}
