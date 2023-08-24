use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct Rectangle {
    point_1: Point,
    point_2: Point,
}

fn ipart(x: f32) -> i32 {
    x.floor() as i32
}

fn round(x: f32) -> i32 {
    ipart(x + 0.5)
}

fn fpart(x: f32) -> f32 {
    x - ipart(x) as f32
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

fn draw_point(x: i32, y: i32, c: f32, canvas: &mut Canvas<Window>) {
    let color_value = (c * 255.0) as u8;
    let color = Color::RGB(color_value, color_value, color_value);

    canvas.set_draw_color(color);
    canvas
        .draw_point(Point::new(x, y))
        .expect("Drawing point failed");
}

pub fn wu_line(p_start: Point, p_end: Point, canvas: &mut Canvas<Window>) {
    let steep = i32::abs(p_end.y - p_start.y) > i32::abs(p_end.x - p_start.x);

    let (x1, y1, x2, y2) = if steep {
        (p_start.y, p_start.x, p_end.y, p_end.x)
    } else {
        (p_start.x, p_start.y, p_end.x, p_end.y)
    };

    let (x1, x2, y1, y2) = if x1 > x2 {
        (x2, x1, y2, y1)
    } else {
        (x1, x2, y1, y2)
    };

    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut gradient = dy as f32 / dx as f32;

    if dx == 0 {
        gradient = 1.0;
    }

    let mut x_end = round(x1 as f32);
    let y_end = y1 as f32 + gradient * (x_end - x1) as f32;
    let xpxl1 = x_end as i32;
    let mut intery = y_end + gradient;

    x_end = round(x2 as f32);
    let xpxl2 = x_end as i32;

    if steep {
        for x in xpxl1..xpxl2 {
            draw_point(ipart(intery), x, rfpart(intery), canvas);
            draw_point(ipart(intery) - 1, x, fpart(intery), canvas);
            intery += gradient;
        }
    } else {
        for x in (xpxl1 + 1)..xpxl2 {
            draw_point(x, ipart(intery), rfpart(intery), canvas);
            draw_point(x, ipart(intery) - 1, fpart(intery), canvas);
            intery += gradient;
        }
    }
}

pub fn draw_wu_rect(p_1: Point, p_2: Point, canvas: &mut Canvas<Window>) {
    wu_line(p_1, Point::new(p_1.x, p_2.y), canvas);
    wu_line(p_1, Point::new(p_2.x, p_1.y), canvas);

    wu_line(Point::new(p_1.x, p_2.y), p_2, canvas);
    wu_line(p_2, Point::new(p_2.x, p_1.y), canvas);

    let x_is_reversed = p_2.x < p_1.x;
    let y_is_reversed = p_2.y < p_1.y;

    let (start_x, end_x) = if x_is_reversed {
        (p_2.x, p_1.x)
    } else {
        (p_1.x, p_2.x)
    };

    let (start_y, end_y) = if y_is_reversed {
        (p_2.y, p_1.y)
    } else {
        (p_1.y, p_2.y)
    };

    for i in start_x..end_x {
        for j in start_y..end_y {
            draw_point(i, j, 0.0, canvas);
        }
    }

}

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

    let mut rectangles: Vec<Rectangle> = Vec::new();
    let mut start_point: Option<Point> = None;
    let mut end_point: Option<Point> = None;

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

                    if start_point.is_none() {
                        start_point = Some(Point::new(x, y));
                    } else if end_point.is_none() {
                        end_point = Some(Point::new(x, y));
                    }

                    if start_point.is_some() && end_point.is_some() {
                        // draw_wu_rect(start_point.unwrap(), end_point.unwrap(), &mut canvas);

                        let rect = Rectangle {
                            point_1: start_point.unwrap(),
                            point_2: end_point.unwrap(),
                        };

                        rectangles.push(rect);

                        start_point = None;
                        end_point = None;
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }

        for rect in rectangles.iter() {
            draw_wu_rect(rect.point_1, rect.point_2, &mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
