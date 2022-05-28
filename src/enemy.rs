
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Enemy {
    x: f64,
    y: f64,
    width: u32,
    height: u32,
    speed: i32, // num pixels moved per second
}
impl Enemy {
    pub fn new(x: f64, y: f64, width: u32, height: u32, speed: i32) -> Enemy {
        Enemy {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(self.x as i32, self.y as i32, self.width.try_into().unwrap(), self.height.try_into().unwrap())).unwrap();
    }

    pub fn update_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x = self.x + delta_x * self.speed as f64;
        self.y = self.y + delta_y * self.speed as f64;
    }
}