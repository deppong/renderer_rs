use super::linear_math::*;
use sdl2::pixels::Color;

pub fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>, width: u32, height: u32) {
    if x >= width || y >= height {
        return;
    }

    framedata[((x + y * width) * 4 + 0) as usize] = color.b;
    framedata[((x + y * width) * 4 + 1) as usize] = color.g;
    framedata[((x + y * width) * 4 + 2) as usize] = color.r;
    framedata[((x + y * width) * 4 + 3) as usize] = color.a;
}

#[derive(Clone, Copy)]
pub struct RPoint {
    pub transform: vec3,
    pub color: Color,
}

pub struct RLine {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}

impl RLine {
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw(&self, framedata: &mut Vec<u8>, width: u32, height: u32) {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        let mut d = 2.0 * dy - dx;
        let mut y = self.y0;

        for x in self.x0 as i32..self.x1 as i32 {
            put_pixel(x as u32, y as u32, Color::WHITE, framedata, width, height);
            if d > 0.0 {
                y += 1.0;
                d = d - 2.0 * dx;
            }
            d = d + 2.0 * dy;
        }
    }
}
