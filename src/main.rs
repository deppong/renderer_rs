extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sdlrs_template", WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();

    let mut framedata: Vec<u8> = vec![0; (WIDTH*HEIGHT*4) as usize];


    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    let mut multiplier: f32 = 0.0;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                    break;
                },
                _ => {}
            }
        }

        // edit framedata as you see fit
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = (x + y*WIDTH)*4;
                let r = (255.0 * multiplier.sin()) as u32 * x / WIDTH;
                let g = (255.0 * multiplier.sin()) as u32 * y / HEIGHT;
                framedata[(index + 0) as usize ] = 0 as u8; // B
                framedata[(index + 1) as usize ] = g as u8; // G
                framedata[(index + 2) as usize ] = r as u8; // R
                framedata[(index + 3) as usize ] = 255; // R
            }
        }

        multiplier += 0.1;

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
