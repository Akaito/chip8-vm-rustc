// This is a translation to Rust from my C++ CHIP-8 emulator at
// https://github.com/Akaito/csaru-chip8
// Which itself is *heavily* based on Laurence Muller's tutorial at
// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

struct Chip8 {
    static const MEMORY_BYTES:   u32 = 4096;
    static const REGISTER_COUNT: u32 = 16;
    static const RENDER_WIDTH:   u32 = 64;
    static const RENDER_HEIGHT:  u32 = 32;
    static const STACK_SIZE:     u32 = 16;
    static const KEY_COUNT:      u32 = 16;

    static const INTERPRETER_BEGIN:  u16 = 0x000;
    static const INTERPRETER_END:    u16 = 0x1FF;
    static const FONT_BEGIN:         u16 = 0x050;
    static const FONT_END:           u16 = 0x0A0;
    static const PROG_ROM_RAM_BEGIN: u16 = 0x200;
    static const PROG_ROM_RAM_END:   u16 = 0xFFF;

    let memory: [u8; MEMORY_BYTES];
    let v:      [u8; REGISTER_COUNT];  // registers V1-VF
    let i:           u16;              // index register
    let pc:          u16;              // program counter
    let delay_timer: u8;               // decrement if not 0
    let sound_timer: u8;               // decrement if not 0, beep on reaching 0
    let key_states:  [u8; KEY_COUNT];  // hex keypad buttonstates

    let opcode: u16;
    let stack:  [u16; STACK_SIZE];
    let sp:     u8;  // stack pointer
    let render_out: [u8; RENDER_WIDTH * RENDER_HEIGHT];  // 64x32 B&W display

    let draw_flag: bool;  // whether or not a GUI application should render
};

fn main() {
    println!("Hello, world!");
}
