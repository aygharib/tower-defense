extern crate sdl2;

use sdl2::mouse::{MouseButton, PressedMouseButtonIterator};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;

use sdl2::video::Window;
use sdl2::render::Canvas;

struct Turret {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}
impl Turret {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Turret {
        Turret {
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

struct Enemy {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    speed: i32, // num pixels moved per second
}
impl Enemy {
    fn new(x: i32, y: i32, width: u32, height: u32, speed: i32) -> Enemy {
        Enemy {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(self.x, self.y, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    }

    fn update_position(&mut self, delta_x: i32, delta_y: i32) {
        self.x = self.x + delta_x * self.speed;
        self.y = self.y + delta_y * self.speed;
    }
}

fn draw_arena(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.draw_line(Point::new(0, 50), Point::new(1920-50, 50));
    canvas.draw_line(Point::new(1920-50, 50), Point::new(1920-50, 150));
    canvas.draw_line(Point::new(10, 150), Point::new(1920-50, 150));

    canvas.draw_line(Point::new(0, 50), Point::new(1920-50, 50));
    canvas.draw_line(Point::new(1920-50, 50), Point::new(1920-50, 150));
    canvas.draw_line(Point::new(0, 150), Point::new(1920-50, 150));
}

fn draw_hovered_tile(canvas: &mut Canvas<Window>, mouse_x: i32, mouse_y: i32) {
    let temp_x = (mouse_x / 50) * 50;
    let temp_y = (mouse_y / 50) * 50;

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.draw_rect(Rect::new(temp_x, temp_y, 50, 50));
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let timer_subsystem = sdl_context.timer().unwrap();

    // println!("{:?}", sdl_context.timer().unwrap());

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        // .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();

    let mut listEnemies: Vec<Enemy> = Vec::new();
    listEnemies.push(Enemy::new(100, 0, 50, 50, 25));
    listEnemies.push(Enemy::new(200, 0, 50, 50, 25));
    listEnemies.push(Enemy::new(300, 0, 50, 50, 25));

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut delta_ticks = 0;
    let mut last_tick_time = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    println!("left mouse button pressed!: {:?}", event);
                }
                _ => {}
            }
        }

        // get mouse state
        let mouse_state = event_pump.mouse_state();

        // create set of pressed mouse buttons
        let pressed_mouse_buttons: HashSet<MouseButton> = mouse_state.pressed_mouse_buttons().collect();
        
        if !pressed_mouse_buttons.is_empty() {
            println!("X = {:?}, Y = {:?} : {:?}", mouse_state.x(), mouse_state.y(), pressed_mouse_buttons);
        }

        // Update
        for val in listEnemies.iter_mut() {
            val.update_position(val.speed, 0);
        }

        // Render
        draw_arena(&mut canvas);
        draw_hovered_tile(&mut canvas, mouse_state.x(), mouse_state.y());

        for val in listEnemies.iter() {
            val.draw(&mut canvas);
        }

        let tick_time = timer_subsystem.ticks();
        delta_ticks = tick_time - last_tick_time;
        last_tick_time = tick_time;
        //let total_ticks = whatever.ticks(); // this increments by 1000 per second
        println!("Total Ticks: {} | Delta Ticks: {}", tick_time, delta_ticks); 


        canvas.present();
        // sleeps for 1/60 th of a second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}