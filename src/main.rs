use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

mod graphics;
mod numbers;
mod sdl_to_bmp;

const W: u32 = 840;
const H: u32 = 680;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", W, H)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut angle = 0.0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Clicked at {x}, {y}");
                    graphics::flood_fill(Point::new(x, y), 0, &mut canvas)
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }

        let mut number_series =
            numbers::NumberSeries::new(100,0, String::from("123"), 0.1, 0.1);
        angle = angle + 0.001;

        for num in number_series.numbers.iter_mut() {
            num.draw(&mut canvas, false);
        }

        canvas.present();

        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::S)
        {
            let texture_creator = canvas.texture_creator();

            let mut text = texture_creator
                .create_texture(
                    sdl2::pixels::PixelFormatEnum::ARGB8888,
                    sdl2::render::TextureAccess::Streaming,
                    W,
                    H,
                )
                .unwrap();

            sdl_to_bmp::save_canvas_to_bmp(&mut text, &mut canvas)
                .expect("Failed to save BMP file");
            println!("Canvas saved as 'output.bmp'");
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
