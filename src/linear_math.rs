use std::ops;

// Simple vector 3 for matrix stuff
#[derive(Clone, Copy, Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4f {
    pub data: [[f32; 4]; 4],
}

impl Mat4f {
    pub fn rot_x(angle: f32) -> Mat4f {
        Mat4f { 
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), -angle.sin(), 0.0],
                [0.0, angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }

    pub fn rot_y(angle: f32) -> Mat4f {
        Mat4f { 
            data: [
                [angle.cos() , 0.0, angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }
}


impl ops::Mul<Vec3f> for Mat4f {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f { 
            x: self.data[0][0]*other.x + self.data[0][1]*other.y + self.data[0][2]*other.z,
            y: self.data[1][0]*other.x + self.data[1][1]*other.y + self.data[1][2]*other.z,
            z: self.data[2][0]*other.x + self.data[2][1]*other.y + self.data[2][2]*other.z,
        }
    }
}