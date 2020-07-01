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


const FONT_SET: [u8; 5 * 16] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0,  // 0
    0x20, 0x60, 0x20, 0x20, 0x70,  // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
    0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
    0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
    0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
    0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
    0xF0, 0x80, 0xF0, 0x80, 0x80,  // F
];


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
        let mut c8 = Chip8 {
            memory: Vec::with_capacity(MEMORY_BYTES),
            v: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            i: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            key_states: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            opcode: 0,
            stack: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sp: 0,
            render_out: Vec::with_capacity(RENDER_WIDTH * RENDER_HEIGHT),
            draw_flag: false,
        };
        // call init
        c8.init(42);
        c8
    }


    fn init (&mut self, rand_seed: u32) {
        // memory MEMORY_BYTES
        for i in 0..self.memory.len() { self.memory[i] = 0; }
        for i in 0..self.v.len() { self.v[i] = 0 }
        self.i  = 0x0000;
        self.pc = PROG_ROM_RAM_BEGIN;
        self.delay_timer = 0;
        self.sound_timer = 0;
        for i in 0..self.key_states.len() { self.key_states[i] = 0; }
        self.opcode = 0;
        for i in 0..self.stack.len() { self.stack[i] = 0; }
        self.sp = 0;
        for i in 0..self.render_out.len() { self.render_out[i] = 0; }
        self.draw_flag = false;
    }


    fn load_rom_file (&mut self, path: &str) -> std::io::Result<()> {
        self.init(42);
        let mut f = std::fs::File::open(path);
        // read the whole file
        self.memory = std::fs::read(path)?;

        Ok(())
    }

}


fn main() {
    let mut c8 = Chip8::new();
    c8.load_rom_file("Maze (alt) [David Winter, 199x].ch8")
        .expect("Issue loading Chip-8 ROM file into VM memory");

    println!("First few program bytes: {:#04x} {:#04x} {:#04x} {:#04x}",
             c8.memory[0], c8.memory[1], c8.memory[2], c8.memory[3]);
}

