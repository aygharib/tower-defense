#[allow(dead_code)]

extern crate sdl2;

use engine::{Arena, Engine};
use interface::Interface;
use sdl2::mouse::{MouseButton};
use sdl2::rect::{Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::render::Canvas;

mod engine;
mod interface;

pub fn main() -> Result<(), String> {
    let engine = Engine::new();
    Interface::run(engine);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // let mut listEnemies: Vec<Enemy> = Vec::new();
    // listEnemies.push(Enemy::new(100.0, 0.0, 50, 50, 1));
    // listEnemies.push(Enemy::new(200.0, 0.0, 50, 50, 2));
    // listEnemies.push(Enemy::new(300.0, 0.0, 50, 50, 3));

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

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

        // for val in listEnemies.iter_mut() {
        //     val.update_position(1.0, 0.0);
        // }

        // Render
        // arena.draw_arena(&mut canvas); 
        // draw_hovered_tile(&mut canvas, mouse_state.x(), mouse_state.y());

        // for val in listEnemies.iter() {
        //     val.draw(&mut canvas);
        // }

        canvas.present();
        // sleeps for 1/60 th of a second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}

fn draw_hovered_tile(canvas: &mut Canvas<Window>, mouse_x: i32, mouse_y: i32) {
    let temp_x = (mouse_x / 50) * 50;
    let temp_y = (mouse_y / 50) * 50;

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.draw_rect(Rect::new(temp_x, temp_y, 50, 50));
}