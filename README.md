# NES Emulator

An emulator for the NTSC Nintendo Entertainment System, written in Rust.
Passes nestest with both official and illegal opcodes.

## Progress 

The project is currently in-progress, so not all the core functionality is implemented yet.

### Core 

- [x] CPU
- [x] CPU bus
- [x] Cartridge loading
- [ ] PPU
- [ ] PPU bus
- [ ] APU
- [ ] Controller I/O

### Planned

- [ ] Save states
- [ ] MOS 6502 CPU library
  - Trait containing all generic instructions
  - Allows for easier development of other emulators later (Apple II, Commodore 64, Atari)
- [ ] Game rewind
- [ ] More mappers
  - Support for more games
