// crates
extern crate sdl2;

// sdl
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// mine
use shapes::*;
use linear_math::*;

pub mod linear_math;
pub mod shapes;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>) {
    if x >= WIDTH || y >= HEIGHT { return; }

    framedata[((x + y * WIDTH)*4 + 0) as usize] = color.b;
    framedata[((x + y * WIDTH)*4 + 1) as usize] = color.g;
    framedata[((x + y * WIDTH)*4 + 2) as usize] = color.r;
    framedata[((x + y * WIDTH)*4 + 3) as usize] = color.a;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Renderer RS", WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();
    let mut framedata = vec![0; ((WIDTH*HEIGHT)*4) as usize];

    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut points = [RPoint{transform: vec3{x: 0.0, y: 0.0, z: 0.0}, color: Color::WHITE}; 5];

    for i in 0..5 {
        points[i].transform.x = 5.0 *  i as f32;
        points[i].transform.y = 5.0 * -(i as f32);
    }
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }

        for i in points {
            put_pixel(i.transform.x as u32 + WIDTH/2, i.transform.y as u32 + HEIGHT/2, i.color, &mut framedata);
        }


        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
