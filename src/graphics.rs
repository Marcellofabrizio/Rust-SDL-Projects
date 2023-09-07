use std::collections::VecDeque;

use std::{thread, time};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureAccess};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};

pub trait Drawing {
    fn new(&self);
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub struct BezierCurve {
    pub controll_points: Vec<Point>,
    pub color: f32,
}

impl BezierCurve {
    pub fn new(points: Vec<Point>, color: Option<f32>) -> Self {
        BezierCurve {
            controll_points: points,
            color: color.unwrap_or(0.0),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.controll_points.len() < 4 {
            println!("Not enough controll points for drawing");
            return;
        }

        draw_cubic_bezier(
            self.controll_points[0],
            self.controll_points[1],
            self.controll_points[2],
            self.controll_points[3],
            canvas,
        );
    }

    pub fn add_point(&mut self, point: Point) {
        if self.can_receive_points() == false {
            println!("Already has all points");
            return;
        }

        self.controll_points.push(point);
    }

    pub fn can_receive_points(&self) -> bool {
        self.controll_points.len() < 4
    }
}

pub struct Rectangle {
    pub controll_points: Vec<Point>,
}

impl Rectangle {
    pub fn new(points: Vec<Point>) -> Self {
        Rectangle {
            controll_points: points,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        match self.controll_points.as_slice() {
            [first, second, ..] => {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                draw_line(*first, Point::new(first.x, second.y), canvas);
                draw_line(*first, Point::new(second.x, first.y), canvas);
                draw_line(Point::new(first.x, second.y), *second, canvas);
                draw_line(*second, Point::new(second.x, first.y), canvas);
            }
            _ => {
                println!("Rectangle does not have all points");
            }
        }
    }
}

pub fn flood_fill(start: Point, fill_color: u32, canvas: &mut Canvas<Window>) {
    let (width, height) = canvas.output_size().unwrap();

    let canvas_pixels = canvas
        .read_pixels(None, sdl2::pixels::PixelFormatEnum::ARGB8888)
        .expect("Read pixels failes");

    let default_color: u32 = get_color(start, width, &canvas_pixels);
    println!("Default color: {}", default_color);

    let mut stack: VecDeque<Point> = VecDeque::new();
    stack.push_back(start);

    while stack.is_empty() == false {
        let p = stack.pop_back().unwrap();
        if p.y < 0 || p.y > (height as i32 - 1) || p.x < 0 || p.x > (width as i32 - 1) {
            continue;
        }

        let pixel_color: u32 = get_color(p, width, &canvas_pixels);

        if pixel_color == default_color {
            draw_point(p.x, p.y, fill_color as f32, canvas);
            println!("Desenhando no: {:?}", p);
            stack.push_back(Point::new(p.x + 1, p.y));
            stack.push_back(Point::new(p.x - 1, p.y));
            stack.push_back(Point::new(p.x, p.y + 1));
            stack.push_back(Point::new(p.x, p.y - 1));
            canvas.present();
        } else {
            println!("Ponto: {:?}", p);
        }

        let ten_millis = time::Duration::from_millis(100);

        thread::sleep(ten_millis);
    }
}

pub fn get_color(point: Point, screen_width: u32, pixels: &Vec<u8>) -> u32 {
    let index = (point.y as u32 * screen_width + point.x as u32) * 4;

    let b = pixels[index as usize] as u32;
    let g = pixels[(index + 1) as usize] as u32;
    let r = pixels[(index + 2) as usize] as u32;
    let a = pixels[(index + 3) as usize] as u32;

    (a << 24) | (r << 16) | (g << 8) | b

    // println!(
    //     "Collor on click: R:{:?}, G:{:?}, B:{:?}",
    //     get_color_component(color, 'r'),
    //     get_color_component(color, 'g'),
    //     get_color_component(color, 'b')
    // );
}

pub fn get_color_component(color: u32, component: char) -> u8 {
    match component {
        'r' | 'R' => ((color >> 16) & 0xFFF) as u8,
        'g' | 'G' => ((color >> 8) & 0xFFF) as u8,
        'b' | 'B' => (color & 0xFFF) as u8,
        _ => panic!("Invalid color component, use r, g or b"),
    }
}

pub fn draw_point(x: i32, y: i32, c: f32, canvas: &mut Canvas<Window>) {
    let r = get_color_component(c as u32, 'r');
    let g = get_color_component(c as u32, 'g');
    let b = get_color_component(c as u32, 'b');
    let color = Color::RGB(r, g, b);

    canvas.set_draw_color(color);
    canvas
        .draw_point(Point::new(x, y))
        .expect("Drawing point failed");
}

pub fn draw_line(p_1: Point, p_2: Point, canvas: &mut Canvas<Window>) {
    let mut x0 = p_1.x;
    let mut y0 = p_1.y;
    let mut x1 = p_2.x;
    let mut y1 = p_2.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let mut sx = 0;
    let mut sy = 0;

    if x0 < x1 {
        sx = 1;
    } else {
        sx = -1;
    }

    if y0 < y1 {
        sy = 1;
    } else {
        sy = -1;
    }

    let mut err = dx - dy;

    loop {
        canvas
            .draw_point(Point::new(x0, y0))
            .expect("Draw point failed");

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_circle(center: Point, radius: i32, canvas: &mut Canvas<Window>) {
    let mut x: i32 = 0;
    let mut y: i32 = radius;
    let mut decision_param = 3 - 2 * radius;
    display_circle(center, Point::new(x, y), 0.0, canvas);
    while y >= x {
        x += 1;
        if decision_param > 0 {
            y -= 1;
            decision_param = decision_param + 4 * (x - y) + 10;
        } else {
            decision_param = decision_param + 4 * x + 6;
        }

        display_circle(center, Point::new(x, y), 0.0, canvas);
    }
}

fn display_circle(center: Point, point_to_draw: Point, color: f32, canvas: &mut Canvas<Window>) {
    draw_point(
        center.x + point_to_draw.x,
        center.y + point_to_draw.y,
        color,
        canvas,
    );
    draw_point(
        center.x - point_to_draw.x,
        center.y + point_to_draw.y,
        color,
        canvas,
    );
    draw_point(
        center.x + point_to_draw.x,
        center.y - point_to_draw.y,
        color,
        canvas,
    );
    draw_point(
        center.x - point_to_draw.x,
        center.y - point_to_draw.y,
        color,
        canvas,
    );
    draw_point(
        center.x + point_to_draw.y,
        center.y + point_to_draw.x,
        color,
        canvas,
    );
    draw_point(
        center.x - point_to_draw.y,
        center.y + point_to_draw.x,
        color,
        canvas,
    );
    draw_point(
        center.x + point_to_draw.y,
        center.y - point_to_draw.x,
        color,
        canvas,
    );
    draw_point(
        center.x - point_to_draw.y,
        center.y - point_to_draw.x,
        color,
        canvas,
    );
}

pub fn draw_cubic_bezier(
    p_1: Point,
    p_2: Point,
    p_3: Point,
    p_4: Point,
    canvas: &mut Canvas<Window>,
) {
    for u in 0..1000 {
        let u = u as f32 / 1000 as f32;

        let x_u = (1.0 - u).powi(3) * p_1.x as f32
            + 3.0 * u * (1.0 - u).powi(2) * p_2.x as f32
            + 3.0 * u.powi(2) * (1.0 - u) * p_3.x as f32
            + u.powi(3) * p_4.x as f32;

        let y_u = (1.0 - u).powi(3) * p_1.y as f32
            + 3.0 * u * (1.0 - u).powi(2) * p_2.y as f32
            + 3.0 * u.powi(2) * (1.0 - u) * p_3.y as f32
            + u.powi(3) * p_4.y as f32;

        draw_point(x_u as i32, y_u as i32, 0.0, canvas);
    }
}

pub fn draw_target(point: Point, canvas: &mut Canvas<Window>) {
    canvas
        .filled_circle(point.x as i16, point.y as i16, 5, Color::RGB(255, 0, 0))
        .unwrap();
}

pub fn draw_heart(x: i32, y: i32, canvas: &mut Canvas<Window>) {
    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x, y - 30),
        Point::new(x - 50, y - 30),
        Point::new(x - 50, y),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x - 50, y),
        Point::new(x - 50, y + 30),
        Point::new(x, y + 35),
        Point::new(x, y + 60),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x, y + 60),
        Point::new(x, y + 35),
        Point::new(x + 50, y + 30),
        Point::new(x + 50, y),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x + 50, y),
        Point::new(x + 50, y - 30),
        Point::new(x, y - 30),
        Point::new(x, y),
        canvas,
    );
}
