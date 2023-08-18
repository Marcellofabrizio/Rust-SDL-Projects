use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

fn ipart(x: f32) -> i32 {
    x.floor() as i32
}

fn round(x: f32) -> i32 {
    ipart(x + 0.5)
}

fn fpart(x: f32) -> f32 {
    x - x.floor()
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

pub fn wu_line(x1: i32, y1: i32, x2: i32, y2: i32, canvas: &mut Canvas<Window>) {
    let steep = i32::abs(y2 - y1) > i32::abs(x2 - x1);

    let (x1, y1, x2, y2) = if steep {
        (y1, x1, y2, x2)
    } else {
        (x1, y1, x2, y2)
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
    let mut y_end = y1 as f32 + gradient * (x_end - x1) as f32;
    let mut x_gap = rfpart(x1 as f32 + 0.5);
    let xpxl1 = x_end as i32;
    let ypxl1 = ipart(y_end);

    if steep {
        draw_point(ypxl1, xpxl1, rfpart(y_end) * x_gap, canvas);
        draw_point(ypxl1 + 1, xpxl1, fpart(y_end) * x_gap, canvas);
    } else {
        draw_point(xpxl1, ypxl1, rfpart(y_end) * x_gap, canvas);
        draw_point(xpxl1, ypxl1 + 1, fpart(y_end) * x_gap, canvas);
    }

    let mut intery = y_end + gradient;

    x_end = round(x2 as f32);
    y_end = y2 as f32 + gradient * (x_end - x2) as f32;
    x_gap = rfpart(x2 as f32 + 0.5);
    let xpxl2 = x_end as i32;
    let ypxl2 = ipart(y_end);

    if steep {
        draw_point(ypxl2, xpxl2, rfpart(y_end) * x_gap, canvas);
        draw_point(ypxl2 + 1, xpxl2, fpart(y_end) * x_gap, canvas);
    } else {
        draw_point(xpxl2, ypxl2, rfpart(y_end) * x_gap, canvas);
        draw_point(xpxl2, ypxl2 + 1, fpart(y_end) * x_gap, canvas);
    }

    if steep {
        for x in (xpxl1 + 1)..xpxl2 {
            draw_point(ipart(intery), x, rfpart(intery), canvas);
            draw_point(ipart(intery) + 1, x, fpart(intery), canvas);
            intery += gradient;
        }
    } else {
        for x in (xpxl1 + 1)..xpxl2 {
            draw_point(x, ipart(intery), rfpart(intery), canvas);
            draw_point(x, ipart(intery) + 1, fpart(intery), canvas);
            intery += gradient;
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => println!("{}", keycode),
                _ => {}
            }
        }
        wu_line(100, 100, 700, 500, &mut canvas);
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
