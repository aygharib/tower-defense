use cgmath::Vector2;

use sdl2::mouse::{MouseButton};
use sdl2::rect::{Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::render::Canvas;

use crate::engine::Engine;

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

pub fn run(mut engine: Engine) {
    engine.refill_bag();

    let sdl = sdl2::init().expect("Failed to initialize SDL");

    let mut canvas = {
        let video = sdl.video().expect("Failed to acquire display");
        let window = video
            .window("Tower Defense", INIT_SIZE.x, INIT_SIZE.y)
            .position_centered()
            .build()
            .expect("Failed to create window");
        
        window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Failed to render canvas")
    };

    let mut events = sdl.event_pump().expect("Failed to get event loop");

    'running: loop {
        for event in events.poll_iter() {
            match dbg!(event) {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    // println!("left mouse button pressed!: {:?}", event);
                }
                _ => {}
            }
        }

        draw(&mut canvas);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    
    canvas.present();
}

fn draw_hovered_tile(canvas: &mut Canvas<Window>, mouse_x: i32, mouse_y: i32) {
    let temp_x = (mouse_x / 50) * 50;
    let temp_y = (mouse_y / 50) * 50;

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.draw_rect(Rect::new(temp_x, temp_y, 50, 50));
}

pub struct Interface {
    engine: Engine,
}

impl Interface {
    
}