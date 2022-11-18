use std::ops;

// Simple vector 3 for matrix stuff
#[derive(Clone, Copy, Debug)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4f {
    pub data: [[f32; 4]; 4],
}

impl Mat4f {
    pub fn new() -> Mat4f {
        Mat4f { 
            data: [[0.0; 4]; 4]
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f { 
            data: [
                [1.0, 0.0, 0.0,   x],
                [0.0, 1.0, 0.0,   y],
                [0.0, 0.0, 1.0,   z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    // Turns out scaling is really easy !
    pub fn scale(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f { 
            data: [
                [  x, 0.0, 0.0, 0.0],
                [0.0,   y, 0.0, 0.0],
                [0.0, 0.0,   z, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
    
    /*
        Rotation
     */
    pub fn rotation(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::rot_x(x) * Mat4f::rot_y(y) * Mat4f::rot_z(z)
    }

    fn rot_x(angle: f32) -> Mat4f {
        Mat4f { 
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), -angle.sin(), 0.0],
                [0.0, angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    fn rot_y(angle: f32) -> Mat4f {
        Mat4f { 
            data: [
                [angle.cos() , 0.0, angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    fn rot_z(angle: f32) -> Mat4f {
        Mat4f { 
            data: [
                [angle.cos() , -angle.sin(), 0.0,  0.0],
                [angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
    // --------------------------------------------------

}


impl ops::Mul<Vec4f> for Mat4f {
    type Output = Vec4f;

    fn mul(self, other: Vec4f) -> Vec4f {
        Vec4f { 
            x: self.data[0][0]*other.x + self.data[0][1]*other.y + self.data[0][2]*other.z + self.data[0][3]*other.w,
            y: self.data[1][0]*other.x + self.data[1][1]*other.y + self.data[1][2]*other.z + self.data[1][3]*other.w,
            z: self.data[2][0]*other.x + self.data[2][1]*other.y + self.data[2][2]*other.z + self.data[2][3]*other.w,
            w: self.data[3][0]*other.x + self.data[3][1]*other.y + self.data[3][2]*other.z + self.data[3][3]*other.w,
        }
    }
}

impl ops::Mul<Mat4f> for Mat4f {
    type Output = Mat4f;

    fn mul(self, other: Mat4f) -> Mat4f {
        let mut out = Mat4f::new();
        for i in 0..self.data.len() {
            for j in 0..other.data.len() {
                for k in 0..4 {
                    out.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        
        out
    }
}