use super::linear_math::*;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct ObjLoader {
    pub verts: Vec<Vec3f>,
    pub faces: Vec<[u32; 3]>,
}

impl ObjLoader {
    pub fn new() -> Self{
        ObjLoader { verts: vec![], faces: vec![] }
    }

    pub fn load_file(&mut self, path: String) {
        let file = File::open(&path).expect("Unable to read file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let line_as_vec = line.split(' ').collect::<Vec<&str>>();

            match line_as_vec[0] {
                // if the line is a vertex
                "v" => {
                    self.verts.push(Vec3f {
                        x: -line_as_vec[1].parse::<f32>().unwrap(),
                        y: -line_as_vec[2].parse::<f32>().unwrap(),
                        z: -line_as_vec[3].parse::<f32>().unwrap(),
                    })
                },
                "f" => {
                    self.faces.push([
                        line_as_vec[1].parse::<u32>().unwrap() - 1, 
                        line_as_vec[2].parse::<u32>().unwrap() - 1, 
                        line_as_vec[3].parse::<u32>().unwrap() - 1, 
                    ]);
                }
                _ => (),
            }
        }

    }

}