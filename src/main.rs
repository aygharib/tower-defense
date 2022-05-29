#[allow(dead_code)]

extern crate sdl2;

use engine::{Engine};

mod engine;
mod interface;

pub fn main() {
    let engine = Engine::new();
    interface::run(engine);
}