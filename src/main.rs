extern crate sdl2;

use sdl2::mouse::MouseButton;
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
}

impl Enemy {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Enemy {
        Enemy {
            x,
            y,
            width,
            height,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(self.x, self.y, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let enemy = Enemy::new(0, 0, 100, 100);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    println!("arrow key pressed");
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    // let new_event = event.clone();
                    // let { timestamp, window_id, which, mouse_btn, clicks, x, y } = event.clone();
                    println!("left mouse button pressed!: {:?}", event);
                    //event.mouse_state();

                    // println!("event_pump.mouse_state(): {:?}", event_pump.mouse_state());
                    //println!();
                }
                _ => {
                    println!("nothin happenin");
                }
            }
        }
        // The rest of the game loop goes here...
        enemy.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}