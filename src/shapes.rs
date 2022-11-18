use super::linear_math::*;
use sdl2::pixels::Color;

pub fn put_pixel(x: f32, y: f32, color: Color, framedata: &mut Vec<u8>, width: u32, height: u32) {
    if x >= width as f32 || y >= height as f32 {
        return;
    }

    let x: u32 = (x + 1. * (width as f32)/2.0) as u32;
    let y: u32 = (y + 1. * (height as f32)/2.0) as u32;

    framedata[((x + y * width) * 4 + 0) as usize] = color.b;
    framedata[((x + y * width) * 4 + 1) as usize] = color.g;
    framedata[((x + y * width) * 4 + 2) as usize] = color.r;
    framedata[((x + y * width) * 4 + 3) as usize] = color.a;
}

#[derive(Clone, Copy)]
pub struct RPoint {
    pub transform: Vec3f,
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
        if (self.y1 - self.y0).abs() < (self.x1 - self.x0).abs() {
            if self.x0 > self.x1 {
                Self::draw_low(framedata, width, height, self.x1, self.y1, self.x0, self.y0);
            } else {
                Self::draw_low(framedata, width, height, self.x0, self.y0, self.x1, self.y1);
            }
        } else {
            if self.y0 > self.y1 {
                Self::draw_high(framedata, width, height, self.x1, self.y1, self.x0, self.y0);
            } else {
                Self::draw_high(framedata, width, height, self.x0, self.y0, self.x1, self.y1);
            }
        }
    }

    fn draw_low(framedata: &mut Vec<u8>, width: u32, height: u32, x0: f32, y0: f32, x1: f32, y1: f32) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;

        let mut yi = 1.;
        if dy < 0. {
            yi = -1.;
            dy = -dy;
        }

        let mut d = (2.0 * dy) - dx;
        let mut y = y0;

        for x in x0 as i32..x1 as i32 {
            put_pixel(x as f32, y as f32, Color::WHITE, framedata, width, height);
            if d > 0.0 {
                y = y + yi;
                d = d + (2.0 * (dy - dx));
            } else {
                d = d + (2.0 * dy);
            }
        }
    }
    
    fn draw_high(framedata: &mut Vec<u8>, width: u32, height: u32, x0: f32, y0: f32, x1: f32, y1: f32) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;

        let mut xi = 1.;
        if dx < 0. {
            xi = -1.;
            dx = -dx;
        }

        let mut d = (2.0 * dx) - dy;
        let mut x = x0;

        for y in y0 as i32..y1 as i32 {
            put_pixel(x as f32, y as f32, Color::WHITE, framedata, width, height);
            if d > 0.0 {
                x = x + xi;
                d = d + (2.0 * (dx - dy)); 
            } else {
                d = d + (2.0 * dx);
            }
        }
    }
}
