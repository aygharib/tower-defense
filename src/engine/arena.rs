use sdl2::{render::Canvas, video::Window, rect::Point, pixels::Color};

use crate::interface::TILE_SIZE;

pub struct Arena {
    width_in_pixels: i32,
    height_in_pixels: i32,
}

impl Arena {
    pub fn new(width_in_pixels: i32, height_in_pixels: i32) -> Arena {
        Arena {
            width_in_pixels,
            height_in_pixels,
        }
    }
    
    pub fn draw_arena(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    
        canvas.draw_line(Point::new(0, TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, TILE_SIZE));
        canvas.draw_line(Point::new(self.width_in_pixels-TILE_SIZE, TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, 3*TILE_SIZE));
        canvas.draw_line(Point::new(10, 3*TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, 3*TILE_SIZE));
    
        canvas.draw_line(Point::new(0, TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, TILE_SIZE));
        canvas.draw_line(Point::new(self.width_in_pixels-TILE_SIZE, TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, 3*TILE_SIZE));
        canvas.draw_line(Point::new(0, 3*TILE_SIZE), Point::new(self.width_in_pixels-TILE_SIZE, 3*TILE_SIZE));
    }
}