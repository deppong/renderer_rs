// crates
extern crate sdl2;

// sdl
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

// mine
use linear_math::*;
use shapes::*;
use obj_loader::*;

pub mod linear_math;
pub mod shapes;
pub mod obj_loader;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 700;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Renderer RS", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator
        .create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT)
        .unwrap();
    let mut framedata: Vec<u8>;

    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut loader = ObjLoader::new();

    // https://groups.csail.mit.edu/graphics/classes/6.837/F03/models/teapot.obj
    loader.load_file("cow.obj".to_string());

    let mut angle: f32 = 0.01;
    
    'running: loop {
        angle += 0.01;
        let rotation_x = Mat4f::rot_x(angle);
        let rotation_y = Mat4f::rot_y(angle);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        framedata = vec![0; ((WIDTH*HEIGHT)*4) as usize];

        for face in &mut loader.faces {
            for j in 0..3 {
                let v0 = rotation_y * loader.verts[face[j] as usize];
                let v1 = rotation_y * loader.verts[face[(j + 1) % 3] as usize];
                let v0 = rotation_x * v0;
                let v1 = rotation_x * v1;
                let line = RLine {
                    x0: v0.x * 50.,
                    x1: v1.x * 50.,
                    y0: v0.y * 50.,
                    y1: v1.y * 50.,
                };
                line.draw(&mut framedata, WIDTH, HEIGHT);
            }
        }

        canvas.clear();
        framebuffer
            .update(None, &framedata, (WIDTH * 4) as usize)
            .expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }
}
