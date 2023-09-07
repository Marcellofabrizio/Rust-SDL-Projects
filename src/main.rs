use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::Point;
use std::time::Duration;

mod graphics;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 480)
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

                    let color = Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 0,
                    };

                    graphics::flood_fill(
                        Point::new(x, y),
                        color.to_u32(unsafe {
                            &PixelFormat::from_ll(sdl2::sys::SDL_AllocFormat(
                                PixelFormatEnum::ARGB8888 as u32,
                            ))
                        }),
                        &mut canvas,
                    );
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }

        let mut pois: Vec<Point> = Vec::new();
        pois.push(Point::new(180, 200));
        pois.push(Point::new(300, 120));
        let rect = graphics::Rectangle::new(pois);
        rect.draw(&mut canvas);

        for point in control_points.iter() {
            graphics::draw_target(*point, &mut canvas);
        }

        if current_point >= 4 && current_point % 4 == 0 {
            graphics::draw_cubic_bezier(
                control_points[current_point - 4],
                control_points[current_point - 3],
                control_points[current_point - 2],
                control_points[current_point - 1],
                &mut canvas,
            );
        }

        let x = 200;
        let y = 200;

        // graphics::draw_heart(x, y, &mut canvas);
        graphics::draw_heart(600, 440, &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
