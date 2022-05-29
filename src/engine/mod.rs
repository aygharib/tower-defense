use cgmath::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use self::enemy::{Kind as EnemyKind, Enemy};

mod enemy;

pub struct Engine {
    arena: Arena,
    pub bag: Vec<Enemy>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            arena: Arena { width_in_pixels: 1920, height_in_pixels: 1080 },
            bag: Vec::new()
        }
    }

    pub fn refill_bag(&mut self) {
        // generate enemies for the wave
        for i in 0..5 {
            let new_enemy = Enemy::new(EnemyKind::Square, Vector2::new(0, 0));
            self.bag.push(new_enemy);
        }
    }
}

pub struct Arena {
    width_in_pixels: u32,
    height_in_pixels: u32,
}

impl Arena {
    pub fn new(width_in_pixels: u32, height_in_pixels: u32) -> Arena {
        Arena {
            width_in_pixels,
            height_in_pixels,
        }
    }
    
    pub fn draw_arena(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    
        canvas.draw_line(Point::new(0, 50), Point::new(1920-50, 50));
        canvas.draw_line(Point::new(1920-50, 50), Point::new(1920-50, 150));
        canvas.draw_line(Point::new(10, 150), Point::new(1920-50, 150));
    
        canvas.draw_line(Point::new(0, 50), Point::new(1920-50, 50));
        canvas.draw_line(Point::new(1920-50, 50), Point::new(1920-50, 150));
        canvas.draw_line(Point::new(0, 150), Point::new(1920-50, 150));
    }
}