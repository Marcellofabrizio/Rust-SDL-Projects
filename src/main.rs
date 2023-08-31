use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod geometry;

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

    let mut rectangles: Vec<geometry::Rectangle> = Vec::new();
    let mut start_point: Option<Point> = None;
    let mut end_point: Option<Point> = None;

    let mut control_points: Vec<Point> = Vec::with_capacity(4);
    let mut current_point = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse click at {x},{y}");
                    let clicked_point = Point::new(x, y);
                    geometry::draw_target(clicked_point, &mut canvas);
                    control_points.push(clicked_point);
                    current_point += 1;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }

        for rect in rectangles.iter() {
            geometry::draw_wu_rect(rect.point_1, rect.point_2, &mut canvas);
        }

        for point in control_points.iter() {
            geometry::draw_target(*point, &mut canvas);
        }

        if current_point >= 4 && current_point % 4 == 0 {
            geometry::draw_cubic_bezier(
                control_points[current_point - 4],
                control_points[current_point - 3],
                control_points[current_point - 2],
                control_points[current_point - 1],
                &mut canvas,
            );
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
