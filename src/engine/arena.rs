use sdl2::{render::Canvas, video::Window, rect::Point, pixels::Color};

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