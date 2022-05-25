extern crate sdl2;

use sdl2::mouse::{MouseButton, PressedMouseButtonIterator};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;

use sdl2::video::Window;
use sdl2::render::Canvas;

struct Enemy {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    speed: i32,
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
        canvas.fill_rect(Rect::new(self.x, self.y, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    }

    fn update_position(&mut self, delta_x: i32, delta_y: i32) {
        self.x = self.x + delta_x * self.speed;
        self.y = self.y + delta_y * self.speed;
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();

    let mut enemy = Enemy::new(0, 0, 100, 100, 25);

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
                Event::KeyDown { keycode: Some(Keycode::Left), .. }  => {
                    enemy.update_position(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. }  => {
                    enemy.update_position(1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. }  => {
                    enemy.update_position(0, -1);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    enemy.update_position(0, 1);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    println!("left mouse button pressed!: {:?}", event);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // get mouse state
        let mouse_state = event_pump.mouse_state();

        // create set of pressed mouse buttons
        let pressed_mouse_buttons: HashSet<MouseButton> = mouse_state.pressed_mouse_buttons().collect();
        
        if !pressed_mouse_buttons.is_empty() {
            println!("X = {:?}, Y = {:?} : {:?}", mouse_state.x(), mouse_state.y(), pressed_mouse_buttons);
        }

        enemy.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// extern crate sdl2;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use std::collections::HashSet;
// use std::time::Duration;

// pub fn main() -> Result<(), String> {
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;

//     let _window = video_subsystem
//         .window("Mouse", 800, 600)
//         .position_centered()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut events = sdl_context.event_pump()?;

//     let mut prev_buttons = HashSet::new();

//     'running: loop {
//         for event in events.poll_iter() {
//             match event {
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 }
//                 | Event::Quit { .. } => break 'running,
//                 _ => {}
//             }
//         }

//         // get a mouse state
//         let state = events.mouse_state();

//         // Create a set of pressed Keys.
//         let buttons = state.pressed_mouse_buttons().collect();

//         // Get the difference between the new and old sets.
//         let new_buttons = &buttons - &prev_buttons;
//         let old_buttons = &prev_buttons - &buttons;

//         if !new_buttons.is_empty() || !old_buttons.is_empty() {
//             println!(
//                 "X = {:?}, Y = {:?} : {:?} -> {:?}",
//                 state.x(),
//                 state.y(),
//                 new_buttons,
//                 old_buttons
//             );
//         }

//         prev_buttons = buttons;

//         std::thread::sleep(Duration::from_millis(100));
//     }

//     Ok(())
// }