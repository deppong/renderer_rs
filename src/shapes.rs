use super::linear_math::*;
use sdl2::pixels::Color;

pub fn put_pixel(x: f32, y: f32, color: Color, framedata: &mut Vec<u8>, width: u32, height: u32) {
    let x: u32 = (x + 1. * (width as f32) / 2.0) as u32;
    let y: u32 = (y + 1. * (height as f32) / 2.0) as u32;

    if x >= width || y >= height || x < 0 || y < 0 {
        return;
    }

    framedata[((x + y * width) * 4 + 0) as usize] = color.b;
    framedata[((x + y * width) * 4 + 1) as usize] = color.g;
    framedata[((x + y * width) * 4 + 2) as usize] = color.r;
    framedata[((x + y * width) * 4 + 3) as usize] = color.a;
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct RLine {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
}

impl RLine {
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw(&self, framedata: &mut Vec<u8>, width: u32, height: u32) {
        if (self.y1 - self.y0).abs() < (self.x1 - self.x0).abs() {
            if self.x0 > self.x1 {
                Self::draw_low(
                    self, framedata, width, height, self.x1, self.y1, self.x0, self.y0,
                );
            } else {
                Self::draw_low(
                    self, framedata, width, height, self.x0, self.y0, self.x1, self.y1,
                );
            }
        } else {
            if self.y0 > self.y1 {
                Self::draw_high(
                    self, framedata, width, height, self.x1, self.y1, self.x0, self.y0,
                );
            } else {
                Self::draw_high(
                    self, framedata, width, height, self.x0, self.y0, self.x1, self.y1,
                );
            }
        }
    }

    fn draw_low(
        &self,
        framedata: &mut Vec<u8>,
        width: u32,
        height: u32,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
    ) {
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
            put_pixel(x as f32, y as f32, self.color, framedata, width, height);
            if d > 0.0 {
                y = y + yi;
                d = d + (2.0 * (dy - dx));
            } else {
                d = d + (2.0 * dy);
            }
        }
    }

    fn draw_high(
        &self,
        framedata: &mut Vec<u8>,
        width: u32,
        height: u32,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
    ) {
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
            put_pixel(x as f32, y as f32, self.color, framedata, width, height);
            if d > 0.0 {
                x = x + xi;
                d = d + (2.0 * (dx - dy));
            } else {
                d = d + (2.0 * dx);
            }
        }
    }
}

impl Triangle {
    // Algorithm mostly stolen from here (Great article btw)
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html#sunbresenhamarticle
    pub fn draw(&mut self, framedata: &mut Vec<u8>, width: u32, height: u32, color: Color) {
        if self.v0.y > self.v1.y {
            std::mem::swap(&mut self.v0, &mut self.v1)
        }
        if self.v0.y > self.v2.y {
            std::mem::swap(&mut self.v0, &mut self.v2)
        }
        if self.v1.y > self.v2.y {
            std::mem::swap(&mut self.v1, &mut self.v2)
        }

        if self.v1.y == self.v2.y {
            self.flat_bot_tri(framedata, width, height, color);
        } else if self.v0.y == self.v1.y {
            self.flat_top_tri(framedata, width, height, color);
        } else {
            let v3 = Point {
                x: self.v0.x
                    + ((self.v1.y - self.v0.y) / (self.v2.y - self.v0.y)) * (self.v2.x - self.v0.x),
                y: self.v1.y,
            };

            let oldself = self.clone();
            self.v2 = v3;
            self.flat_bot_tri(framedata, width, height, color);

            self.v0 = oldself.v1;
            self.v1 = v3;
            self.v2 = oldself.v2;
            self.flat_top_tri(framedata, width, height, color);
        }
    }

    fn flat_bot_tri(&self, framedata: &mut Vec<u8>, width: u32, height: u32, color: Color) {
        let slope1 = (self.v1.x - self.v0.x) / (self.v1.y - self.v0.y);
        let slope2 = (self.v2.x - self.v0.x) / (self.v2.y - self.v0.y);

        let mut x1 = self.v0.x;
        let mut x2 = self.v0.x;

        let mut y = self.v0.y.clone();
        while y <= self.v1.y {
            let line = RLine {
                x0: x1.floor(),
                x1: x2.floor(),
                y0: y.floor(),
                y1: y.floor(),
                color: Color::CYAN,
            };
            line.draw(framedata, width, height);
            x1 += slope1;
            x2 += slope2;
            y += 1.0;
        }
    }

    fn flat_top_tri(&self, framedata: &mut Vec<u8>, width: u32, height: u32, color: Color) {
        let slope1 = (self.v2.x - self.v0.x) / (self.v2.y - self.v0.y);
        let slope2 = (self.v2.x - self.v1.x) / (self.v2.y - self.v1.y);

        let mut x1 = self.v2.x;
        let mut x2 = self.v2.x;

        let mut y = self.v2.y.clone();
        while y > self.v0.y {
            let line = RLine {
                x0: x1.floor(),
                x1: x2.floor(),
                y0: y.floor(),
                y1: y.floor(),
                color: Color::MAGENTA,
            };
            line.draw(framedata, width, height);
            x1 -= slope1;
            x2 -= slope2;
            y -= 1.0;
        }
    }
}
