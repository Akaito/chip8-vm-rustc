// This is a translation to Rust from my C++ CHIP-8 emulator at
// https://github.com/Akaito/csaru-chip8
// Which itself is *heavily* based on Laurence Muller's tutorial at
// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

mod chip8_vm;

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// used by sdl2 / rendering
const WINDOW_WIDTH:  u16 = 640;
const WINDOW_HEIGHT: u16 = 320;
const PIXEL_SIZE:    u32 = WINDOW_WIDTH as u32 / chip8_vm::RENDER_WIDTH as u32;


fn main() {
    let mut c8 = chip8_vm::Chip8::new();
    c8.load_rom_file("Maze (alt) [David Winter, 199x].ch8")
        .expect("Issue loading Chip-8 ROM file into VM memory");

    let first_few_bytes = c8.get_memory_range(chip8_vm::PROG_ROM_RAM_BEGIN as usize, 4);
    println!("First few program bytes: {:#04X} {:#04X} {:#04X} {:#04X}",
             first_few_bytes[0],
             first_few_bytes[1],
             first_few_bytes[2],
             first_few_bytes[3]);

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
            for y in 0..chip8_vm::RENDER_HEIGHT {
                for x in 0..chip8_vm::RENDER_WIDTH {
                    let color: Color;
                    if c8.render_out[y * chip8_vm::RENDER_WIDTH + x] != 0 {
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

