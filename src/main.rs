// crates
extern crate sdl2;

// sdl
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

// mine
use linear_math::*;
use shapes::*;

pub mod linear_math;
pub mod shapes;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

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

    let mut points = [RPoint {
        transform: vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        color: Color::WHITE,
    }; 5];

    for i in 0..5 {
        points[i].transform.x = 5.0 * i as f32;
        points[i].transform.y = 5.0 * -(i as f32);
    }

    let line = RLine {x0: -105.5, x1: 80.0, y0: 0.0, y1: 80.0};
    let line2 = RLine {x0: -300.0, x1: -105.5, y0: 0.0, y1: -200.0};
    

    'running: loop {
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

        line.draw(&mut framedata, WIDTH, HEIGHT);
        line2.draw(&mut framedata, WIDTH, HEIGHT);

        canvas.clear();
        framebuffer
            .update(None, &framedata, (WIDTH * 4) as usize)
            .expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }
}
