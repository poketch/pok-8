use pok_8_core::emu::*;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

const DEFAULT_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = SCREEN_WIDTH as u32;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT as u32;
const TICKS_PER_FRAME: usize = 15;

pub struct POK8;

impl POK8 {
    // TODO: try to refacto this into multiple functions? SDL doesn't seem to like that
    pub fn init(path_to_rom: impl Into<PathBuf>, scale: Option<u32>) -> () {
        
        let scale = scale.unwrap_or(DEFAULT_SCALE);
        
        // Setup SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut window = video_subsystem
            .window("POK8 Emulator", WINDOW_WIDTH * scale, WINDOW_HEIGHT * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
            
        window.set_icon(sdl2::surface::Surface::from_file("./frontend-desktop/POK8_logo.png").unwrap());


        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut pok8 = Emu::init();

        let mut rom = File::open(&path_to_rom.into()).expect("Unable to open file");
        let mut buffer = Vec::new();

        rom.read_to_end(&mut buffer).unwrap();
        pok8.load(&buffer);

        'gameloop: loop {
            for evt in event_pump.poll_iter() {
                match evt {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'gameloop;
                    }
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if let Some(k) = Self::key2btn(key) {
                            pok8.key_down(k);
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        if let Some(k) = Self::key2btn(key) {
                            pok8.key_up(k);
                        }
                    }
                    _ => (),
                }
            }

            for _ in 0..TICKS_PER_FRAME {
                pok8.cycle();
            }
            pok8.tick_timers();
            Self::draw_screen(&pok8, &mut canvas, scale);
        }
    }
}

impl POK8 {
    fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>, scale: u32) {
        // Clear canvas as black
        canvas.set_draw_color(BLACK);
        canvas.clear();

        let screen_buf = emu.get_display();
        // Now set draw color to white, iterate through each point and see if it should be drawn
        canvas.set_draw_color(WHITE);
        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                // Convert our 1D array's index into a 2D (x,y) position
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;

                // Draw a rectangle at (x,y), scaled up by our SCALE value
                let rect = Rect::new((x * scale) as i32, (y * scale) as i32, scale, scale);
                canvas.fill_rect(rect).unwrap();
            }
        }
        canvas.present();
    }

    /*
       Keyboard                    Chip-8
       +---+---+---+---+           +---+---+---+---+
       | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
       +---+---+---+---+           +---+---+---+---+
       | Q | W | E | R |           | 4 | 5 | 6 | D |
       +---+---+---+---+     =>    +---+---+---+---+
       | A | S | D | F |           | 7 | 8 | 9 | E |
       +---+---+---+---+           +---+---+---+---+
       | Z | X | C | V |           | A | 0 | B | F |
       +---+---+---+---+           +---+---+---+---+
    */

    fn key2btn(key: Keycode) -> Option<usize> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
