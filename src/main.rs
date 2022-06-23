#[allow(dead_code)]

extern crate sdl2;

use engine::{Engine};
use interface::{Interface};

mod engine;
mod interface;

pub fn main() {
    let engine = Engine::new();
    let mut interface = Interface::new(engine);
    interface.run();
}