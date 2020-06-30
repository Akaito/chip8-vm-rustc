// This is a translation to Rust from my C++ CHIP-8 emulator at
// https://github.com/Akaito/csaru-chip8
// Which itself is *heavily* based on Laurence Muller's tutorial at
// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/


const MEMORY_BYTES:   usize = 4096;
const REGISTER_COUNT: usize = 16;
const RENDER_WIDTH:   usize = 64;
const RENDER_HEIGHT:  usize = 32;
const STACK_SIZE:     usize = 16;
const KEY_COUNT:      usize = 16;

const INTERPRETER_BEGIN:  u16 = 0x000;
const INTERPRETER_END:    u16 = 0x1FF;
const FONT_BEGIN:         u16 = 0x050;
const FONT_END:           u16 = 0x0A0;
const PROG_ROM_RAM_BEGIN: u16 = 0x200;
const PROG_ROM_RAM_END:   u16 = 0xFFF;


#[derive(Debug)]
struct Chip8 {
    // reminder: 'static' isn't a thing like this
    memory: Vec<u8>,               // should have MEMORY_BYTES elements
    v:      [u8; REGISTER_COUNT],  // registers V1-VF
    i:           u16,              // index register
    pc:          u16,              // program counter
    delay_timer: u8,               // decrement if not 0
    sound_timer: u8,               // decrement if not 0, beep on reaching 0
    key_states:  [u8; KEY_COUNT],  // hex keypad buttonstates

    opcode: u16,
    stack:  [u16; STACK_SIZE],
    sp:     u8,  // stack pointer
    render_out: Vec<u8>,  // should have RENDER_WIDTH * RENDER_HEIGHT elements, 64x32 B&W display

    draw_flag: bool,  // whether or not a GUI application should render
}


impl Chip8 {

    fn new () -> Chip8 {
        // TODO
        // call init
    }

    fn init (&self, rand_seed: u32) {
        // memory MEMORY_BYTES
        for &byte in self.memory.iter() {
            byte = 0u8;
        }
        self.i  = 0x0000;
        self.pc = PROG_ROM_RAM_BEGIN;
        self.delay_timer = 0;
        self.sound_timer = 0;
    }


}


fn main() {
    let c8 = Chip8::new();
}
