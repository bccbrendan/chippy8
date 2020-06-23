use sdl2;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::cpu::DISPLAY_WIDTH;
use crate::cpu::DISPLAY_HEIGHT;

pub struct Display {
    canvas: Canvas<Window>,
    scale: u32,
    pixel_lit: pixels::Color,
    pixel_unlit: pixels::Color,
}

impl Display {
    pub fn new(sdl: &sdl2::Sdl, scale: u32) -> Self {
        println!("getting video subsystem");
        let video = sdl.video().unwrap();
        println!("opening window");
        let window = video.window(
            "CHIPPY8",
            DISPLAY_WIDTH as u32 * scale,
            DISPLAY_HEIGHT as u32 * scale)
            .build()
            .unwrap();
        println!("getting canvas");
        let mut canvas = window.into_canvas()
            .build()
            .unwrap();
        println!("clearing canvas");
        canvas.clear();
        println!("presenting canvas");
        canvas.present();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        Display {
            canvas: canvas,
            scale: scale,
            pixel_lit: pixels::Color::RGB(0xff, 0xbf, 0),
            pixel_unlit: pixels::Color::RGB(0, 0, 0),
        }
    }

    pub fn draw(&mut self, vram: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        for vram_row in 0..DISPLAY_HEIGHT {
            for vram_column in 0..DISPLAY_WIDTH {
                let is_colored = vram[vram_row][vram_column] != 0;
                let color = if is_colored { self.pixel_lit } else { self.pixel_unlit };
                let y: i32 = vram_row as i32 * self.scale as i32;
                let x: i32 = vram_column as i32 * self.scale as i32;
                self.canvas.set_draw_color(color);
                let _ = self.canvas.fill_rect(Rect::new(x, y, self.scale, self.scale));
            }
        }
        self.canvas.present();
    }
}
