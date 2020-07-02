// This is a translation to Rust from my C++ CHIP-8 emulator at
// https://github.com/Akaito/csaru-chip8
// Which itself is *heavily* based on Laurence Muller's tutorial at
// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

use rand::Rng;

extern crate sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
//use sdl2::time::Duration;


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


// XXXX    ..X.
// X..X    .XX.
// X..X    ..X.
// X..X    ..X.
// XXXX    .XXX
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


// used by sdl2 / rendering
const WINDOW_WIDTH:  u16 = 640;
const WINDOW_HEIGHT: u16 = 320;
const PIXEL_SIZE:    u32 = WINDOW_WIDTH as u32 / RENDER_WIDTH as u32;


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
        for i in 0..MEMORY_BYTES { c8.memory.push(0); }
        for i in 0..(RENDER_WIDTH * RENDER_HEIGHT) { c8.render_out.push(0); }
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
        // read the whole file into a temp buffer
        let rom = std::fs::read(path)?;
        println!("Read {} bytes of rom data from file.", rom.len());

        // place temp buffer's rom data into VM memory at the right location
        for i in 0..rom.len() {
            self.memory[PROG_ROM_RAM_BEGIN as usize + i] = rom[i];
        }

        Ok(())
    }


    fn emulate_cycle (&mut self) -> Result<(), &'static str> {
        // fetch opcode
        self.opcode =
            (self.memory[self.pc as usize    ] as u16) << 8 |
            (self.memory[self.pc as usize + 1] as u16);
            println!("Chip8: Opcode {:#06X} at address {:#06X} (ROM offset {:#06X}).",
                self.opcode,
                self.pc,
                self.pc - PROG_ROM_RAM_BEGIN);

        // prepare common portions of opcode
        let     x:   usize = ((self.opcode & 0x0F00) >> 8) as usize;
        //let mut vx:  u8  = self.v[x as usize];  // Can't do refs to vec elements like this.
        let     y:   usize = ((self.opcode & 0x00F0) >> 4) as usize;
        //let mut vy:  u8  = &self.v[y as usize];  // Can't do refs to vec elements like this.
        let     n:   u8    = (self.opcode & 0x000F) as u8;
        let     nn:  u8    = (self.opcode & 0x00FF) as u8;
        let     nnn: u16   = self.opcode & 0x0FFF;

        // decode opcode
        // https://wikipedia.org/wiki/CHIP-8#Opcode_table
        if 0x1000 == self.opcode & 0xF000 {  // 0x1NNN: jump to NNN
            self.pc = nnn;
        }
        else if 0x3000 == self.opcode & 0xF000 {  // 0x3XNN
            // skip next instruction if VX == NN
            self.pc += if self.v[x] == nn {4} else {2};
        }
        else if 0x6000 == self.opcode & 0xF000 {  // 0x6XNN: set VX to NN
            self.v[x] = nn;
            self.pc += 2;
        }
        else if 0x7000 == self.opcode & 0xF000 {  // 0x7XNN: add NN to VX
            self.v[x] += nn;
            self.pc += 2;
        }
        else if 0xA000 == self.opcode & 0xF000 {  // 0xANNN: I = NNN
            self.i = nnn;
            self.pc += 2;
        }
        else if 0xC000 == self.opcode & 0xF000 {  // 0xCXNN: VX = (rand & NN)
            // TODO: Replace with a per-Chip8 random number generator.
            let secret_number = rand::thread_rng().gen_range(0, 255);
            self.v[x] = (secret_number as u8) & nn;
            self.pc += 2;
        }
        else if 0xD000 == self.opcode & 0xF000 {  // 0xDXYN
            // XOR-draw N rows of 8-bit-wide sprites from I
            // at (VX, VY), (VX, VY+1), etc.
            // VF set to 1 if a pixel is toggled off, otherwise 0.

            self.v[0xF] = 0;  // clear collision flag

            for sprite_tex_y in 0..n {
                let sprite_byte = self.memory[(self.i + sprite_tex_y as u16) as usize];
                for sprite_tex_x in 0..8 {
                    // shift b1000'0000 right to current column
                    if sprite_byte & (0x80 >> sprite_tex_x) != 0 {
                        // rendering wraps on all edges
                        // FIXME: 'attempt to multiply with overflow'
                        let pixel_x: u16 =  ((self.v[x] + sprite_tex_x) % (RENDER_WIDTH as u8)) as u16;
                        //let pixel_y = ((self.v[y] + sprite_tex_y) % (RENDER_HEIGHT as u8)) * (RENDER_WIDTH as u8);
                        let pixel_y: u16 = 
                            (((self.v[y] + sprite_tex_y) % (RENDER_HEIGHT as u8)) as u16)
                            .wrapping_mul(RENDER_WIDTH as u16);
                        let pixel_index = (pixel_y + pixel_x) as usize;

                        if self.render_out[pixel_index] != 0 {
                            self.render_out[pixel_index] = 0;
                            self.v[0xF] = 1;  // Collision!  Set flag.
                        } else {
                            self.render_out[pixel_index] = 1;
                        }
                    }
                }
            }

            self.pc += 2;
            self.draw_flag = true;
        }
        else {
            println!("Chip8: Bad opcode {:#06X} at address {:#06X} (ROM offset {:#06X}).",
                self.opcode,
                self.pc,
                self.pc - PROG_ROM_RAM_BEGIN);
            return Err("Unsupported opcode.");
        }

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            // (beep)
            self.sound_timer -= 1;
            println!("  -- beep! --  ");  // temporary
        }

        Ok(())
    }

}


fn main() {
    let mut c8 = Chip8::new();
    c8.load_rom_file("Maze (alt) [David Winter, 199x].ch8")
        .expect("Issue loading Chip-8 ROM file into VM memory");

    println!("First few program bytes: {:#04X} {:#04X} {:#04X} {:#04X}",
             c8.memory[PROG_ROM_RAM_BEGIN as usize + 0],
             c8.memory[PROG_ROM_RAM_BEGIN as usize + 1],
             c8.memory[PROG_ROM_RAM_BEGIN as usize + 2],
             c8.memory[PROG_ROM_RAM_BEGIN as usize + 3]);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        //.present_vsync()
        .build().unwrap();

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas.clear();
    canvas.present();

    let texture_creator: sdl2::render::TextureCreator<_> = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    //let mut i = 0;
    'running: loop {
        //i = (i + 1) % 255;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // CHIP-8 stuff
        match c8.emulate_cycle() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
                break;
            },
        }

        if c8.draw_flag {
            canvas.clear();
            c8.draw_flag = false;
            //draw_pixel(&mut canvas, 1, 2);
            for y in 0..RENDER_HEIGHT {
                for x in 0..RENDER_WIDTH {
                    let color: Color;
                    if c8.render_out[y * RENDER_WIDTH + x] != 0 {
                        color = Color::RGB(0xEF, 0xEF, 0xEF);
                    } else {
                        color = Color::RGB(0x40, 0x40, 0x40);
                    }

                    canvas.set_draw_color(color);
                    canvas.fill_rect(Rect::new(
                            (x as u32 * PIXEL_SIZE) as i32,
                            (y as u32 * PIXEL_SIZE) as i32,
                            PIXEL_SIZE,
                            PIXEL_SIZE))
                        .expect("Failed to fill_rect when drawing a CHIP-8 pixel.");
                }
            }
            canvas.present();
        }
        // end CHIP-8 stuff

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ::std::thread::sleep(std::time::Duration::from_millis(27));
    }  // 'running loop

    /*/
    loop {
        match c8.emulate_cycle() {
            Ok(_) => continue,
            Err(e) => {
                println!("{}", e);
                break;
            },
        }
    }
    // */
}

