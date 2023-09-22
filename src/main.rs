use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
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
                    graphics::flood_fill(
                        Point::new(x, y),
                        Color::RGB(255, 0, 0).to_u32(unsafe {
                            &PixelFormat::from_ll(sdl2::sys::SDL_AllocFormat(
                                PixelFormatEnum::ARGB8888 as u32, // https://github.com/Rust-SDL2/rust-sdl2/issues/840
                            ))
                        }),
                        &mut canvas,
                    )
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }

        let mut number_series =
            numbers::NumberSeries::new(100, 0, String::from("0123456789"), 0.7, 0.0);
        let mut number_series_2 = numbers::NumberSeries::new(
            400,
            200,
            String::from("2245"),
            0.7,
            180.0 * 0.017453293,
        );

        angle = angle + 1.0;

        number_series.draw(&mut canvas, false);

        number_series_2.draw(&mut canvas, false);

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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
