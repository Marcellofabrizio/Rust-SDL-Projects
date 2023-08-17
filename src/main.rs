extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut rng = rand::thread_rng();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::SizeChanged(w, h),
                    ..
                } => {
                    println!("Window changed to size w {w} and h {h}");
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("Some keycode {}", keycode),
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i,64,255-i));
        let (w, h) = canvas.output_size().unwrap();
        let mut points = [Point::new(0, 0); 256];
        points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
        canvas.draw_points(&points[..]).unwrap();
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
