use cgmath::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use self::arena::Arena;
use self::enemy::Enemy;

mod enemy;
mod arena;

pub struct Engine {
    arena: Arena,
    pub bag: Vec<Enemy>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            arena: Arena::new(1920, 1080),
            bag: Vec::new()
        }
    }

    pub fn refill_bag(&mut self) {
        // generate enemies for the wave
        for i in 0..5 {
            let new_enemy = Enemy::new(50, 50, Vector2::new(5 as f64, 0 as f64), Vector2::new((i*100) as f64, 0 as f64));
            self.bag.push(new_enemy);
        }
    }
}