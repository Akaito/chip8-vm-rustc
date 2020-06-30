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

    let memory: Vec<u8>;
};

fn main() {
    println!("Hello, world!");
}
