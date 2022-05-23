extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2::video::Window;
use sdl2::render::Canvas;

struct Enemy {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rect: Rect,
}

impl Enemy {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Enemy {
        Enemy {
            x,
            y,
            width,
            height,
            rect: Rect::new(x, y, width, height),
        }
    }

    fn move_enemy(&mut self) {
        self.x = self.x + 100;
    }

    // fn draw(&self, &mut canvas: &mut Canvas<sdl2::video::Window>) {
    //     canvas.fill_rect(Rect::new(self.x, self.y, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    // }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        //canvas.fill_rect(Rect::new(self.x, self.y, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
        canvas.fill_rect(self.rect).unwrap();
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut enemy = Enemy::new(0, 0, 100, 100);

    let abc = Rect::new(100, 100, 10, 10);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.fill_rect(Rect::new(enemy.x, enemy.y, enemy.width.try_into().unwrap(), enemy.height.try_into().unwrap())).unwrap();
        
        canvas.fill_rect(abc);

        // println!("{}", i);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    enemy.move_enemy();
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        enemy.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}