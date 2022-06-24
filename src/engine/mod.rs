use cgmath::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::interface::INIT_SIZE;

use self::arena::Arena;
use self::enemy::Enemy;

mod enemy;
mod arena;

pub struct Engine {
    pub arena: Arena,
    pub bag: Vec<Enemy>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            arena: Arena::new(INIT_SIZE.x as i32, INIT_SIZE.y as i32),
            bag: Vec::new()
        }
    }

    pub fn refill_bag(&mut self) {
        // generate enemies for the wave
        for i in 0..5 {
            let new_enemy = Enemy::new(50, 50, Vector2::new(5 as f64, 0 as f64), Vector2::new((i*100) as f64, 0 as f64));
            self.bag.push(new_enemy);
        }

        let new_enemy = Enemy::new(50, 50, Vector2::new(0 as f64, 5 as f64), Vector2::new((800-50) as f64, 0 as f64));
        self.bag.push(new_enemy);
    }
}