use cgmath::Vector2;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Enemy {
    width: u32,
    height: u32,
    speed: i32,
    pub position: Vector2<f64>,
}

impl Enemy {
    pub fn new(width: u32, height: u32, speed: i32, position: Vector2<f64>) -> Enemy {
        Enemy {
            width,
            height,
            speed,
            position,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(self.position.x as i32, self.position.y as i32, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    }

    pub fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed as f64;
        self.position.y = self.position.y + self.speed as f64;
    }
}