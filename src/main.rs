use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

mod graphics;
mod numbers;
mod sdl_to_bmp;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 840, 680)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rectangles: Vec<graphics::Rectangle> = Vec::new();
    let mut start_point: Option<Point> = None;
    let mut end_point: Option<Point> = None;

    let mut control_points: Vec<Point> = Vec::with_capacity(4);
    let mut current_point = 0;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    'running: loop {
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let mut num = numbers::create_digit('9').unwrap();

        for line in num.lines.iter_mut() {
            for point in line.controll_points.iter_mut() {
                graphics::translate(point, 200, 100);
            }
        }

        for curve in num.bezier_curves.iter_mut() {
            for point in curve.controll_points.iter_mut() {
                graphics::translate(point, 200, 100);
            }
        }

        num.draw(&mut canvas);

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
                    840,
                    680,
                )
                .unwrap();

            sdl_to_bmp::save_canvas_to_bmp(&mut text, &mut canvas)
                .expect("Failed to save BMP file");
            println!("Canvas saved as 'output.bmp'");
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
