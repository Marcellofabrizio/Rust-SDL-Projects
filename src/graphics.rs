use std::collections::VecDeque;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Number {
    pub number: char,
    pub lines: Vec<Line>,
    pub bezier_curves: Vec<CubicBezierCurve>,
}

pub struct Line {
    pub controll_points: Vec<Point>,
}

pub struct CubicBezierCurve {
    pub controll_points: Vec<Point>,
    pub color: f32,
}

impl CubicBezierCurve {
    pub fn new(points: Vec<Point>, color: Option<f32>) -> Self {
        CubicBezierCurve {
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

pub struct SmoothBezierCurve {
    pub controll_points: Vec<Point>,
    pub color: f32,
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

    let mut canvas_pixels = canvas
        .read_pixels(None, sdl2::pixels::PixelFormatEnum::ARGB8888)
        .expect("Read pixels failes");

    let default_color: u32 = get_color(start, width, &canvas_pixels);

    if default_color == fill_color {
        println!("Cannot flood fill region alredy painted");
        return;
    }

    let mut stack: VecDeque<Point> = VecDeque::new();
    stack.push_back(start);

    while stack.is_empty() == false {
        let p = stack.pop_back().unwrap();
        if p.y < 0 || p.y > (height as i32 - 1) || p.x < 0 || p.x > (width as i32 - 1) {
            continue;
        }

        canvas_pixels = canvas
            .read_pixels(None, sdl2::pixels::PixelFormatEnum::ARGB8888)
            .expect("Read pixels failes");

        let pixel_color: u32 = get_color(p, width, &canvas_pixels);

        if pixel_color == default_color {
            draw_point(p.x, p.y, fill_color as f32, canvas);
            stack.push_back(Point::new(p.x + 1, p.y));
            stack.push_back(Point::new(p.x - 1, p.y));
            stack.push_back(Point::new(p.x, p.y + 1));
            stack.push_back(Point::new(p.x, p.y - 1));
            canvas.present();
        }
    }
}

pub fn get_color(point: Point, screen_width: u32, pixels: &Vec<u8>) -> u32 {
    let index = (point.y as u32 * screen_width + point.x as u32) * 4;

    let b = pixels[index as usize] as u32;
    let g = pixels[(index + 1) as usize] as u32;
    let r = pixels[(index + 2) as usize] as u32;
    let a = pixels[(index + 3) as usize] as u32;

    (a << 24) | (r << 16) | (g << 8) | b
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

pub fn draw_quadratic_bezier(p_1: Point, p_2: Point, p_3: Point, canvas: &mut Canvas<Window>) {
    for u in 0..1000 {
        let u = u as f32 / 1000 as f32;

        let x_u = (1.0 - u).powi(2) * p_1.x as f32
            + 2.0 * u * (1.0 - u) * p_2.x as f32
            + u.powi(2) * p_3.x as f32;

        let y_u = (1.0 - u).powi(2) * p_1.y as f32
            + 2.0 * u * (1.0 - u) * p_2.y as f32
            + u.powi(2) * p_3.y as f32;

        draw_point(x_u as i32, y_u as i32, 0.0, canvas);
    }
}

pub fn draw_target(point: Point, canvas: &mut Canvas<Window>) {
    canvas
        .filled_circle(point.x as i16, point.y as i16, 3, Color::RGB(255, 0, 0))
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

pub fn draw_digit_1_v1(point: Point, canvas: &mut Canvas<Window>) {
    let x_i = point.x;
    let y_i = point.y;

    let mut x = x_i;
    let mut y = y_i;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x - 2, y + 5),
        Point::new(x - 5, y + 9),
        Point::new(x - 9, y + 13),
        canvas,
    );

    x = x - 9;
    y = y + 13;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x - 6, y + 6),
        Point::new(x - 15, y + 12),
        Point::new(x - 22, y + 15),
        canvas,
    );

    x = x - 22;
    y = y + 15;

    draw_line(Point::new(x, y), Point::new(x, y + 17), canvas);

    y = y + 17;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x + 8, y - 3),
        Point::new(x + 16, y - 8),
        Point::new(x + 23, y - 13),
        canvas,
    );

    x = x + 23;
    y = y - 13;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x + 2, y - 1),
        Point::new(x + 3, y - 3),
        Point::new(x + 6, y - 5),
        canvas,
    );

    x = x + 6;
    y = y - 5;

    draw_line(Point::new(x, y), Point::new(x, y + 60), canvas);

    y = y + 60;

    draw_line(Point::new(x, y), Point::new(x + 15, y), canvas);

    x = x + 15;

    draw_line(Point::new(x, y), Point::new(x, y_i), canvas);
    draw_line(Point::new(x, y_i), Point::new(x_i, y_i), canvas);
}

pub fn draw_digit_2_v1(point: Point, canvas: &mut Canvas<Window>) {
    let x_i = point.x;
    let y_i = point.y;

    let mut x = x_i;
    let mut y = y_i + 20;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x, y - 35),
        Point::new(x + 60, y - 35),
        Point::new(x + 60, y),
        canvas,
    );

    x = x + 60;

    draw_cubic_bezier(
        Point::new(x - 60, y + 60),
        Point::new(x - 50, y + 35),
        Point::new(x - 20, y + 30),
        Point::new(x - 20, y),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x - 30, y + 50),
        Point::new(x - 17, y + 35),
        Point::new(x - 10, y + 30),
        Point::new(x, y),
        canvas,
    );
    x = x - 20;
    y = y;

    draw_cubic_bezier(
        Point::new(x, y),
        Point::new(x, y - 14),
        Point::new(x - 23, y - 14),
        Point::new(x - 23, y),
        canvas,
    );

    x = x - 23;

    draw_line(Point::new(x, y), Point::new(x - 17, y), canvas);
}

pub fn draw_digit_0(point: Point, canvas: &mut Canvas<Window>) {
    let w = 80;
    let h = 120;

    let x = 40;
    let y = 10;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    draw_cubic_bezier(
        Point::new(5, 70),
        Point::new(10, 0),
        Point::new(70, 0),
        Point::new(75, 70),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(20, 70),
        Point::new(25, 13),
        Point::new(55, 13),
        Point::new(60, 70),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(5, 70),
        Point::new(10, 137),
        Point::new(70, 137),
        Point::new(75, 70),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(20, 70),
        Point::new(25, 124),
        Point::new(55, 124),
        Point::new(60, 70),
        canvas,
    );
}

pub fn draw_digit_1(point: Point, canvas: &mut Canvas<Window>) {
    let x_i = point.x;
    let y_i = point.y;

    let w = 80;
    let h = 120;

    let x = 40;
    let y = 20;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    draw_line(Point::new(x - 8, y), Point::new(10, y + 35), canvas);

    draw_line(Point::new(10, y + 35), Point::new(x - 8, y + 35), canvas);

    draw_line(Point::new(x - 8, y), Point::new(x + 13, y), canvas);

    draw_line(Point::new(x + 13, y), Point::new(x + 13, h), canvas);

    draw_line(Point::new(x - 8, y + 35), Point::new(x - 8, h), canvas);
}

pub fn draw_digit_2(point: Point, canvas: &mut Canvas<Window>) {
    let x_i = point.x;
    let y_i = point.y;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    let w = 80;
    let h = 120;

    let x = 40;
    let y = 50;

    draw_cubic_bezier(
        Point::new((x - (w / 2) + 5), y),
        Point::new(x - (w / 2) + 3, y - 30),
        Point::new(x + (w / 2) - 3, y - 30),
        Point::new((x + (w / 2) - 5), y),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x - (w / 2) + 22, y),
        Point::new(x - (w / 2) + 20, y - 13),
        Point::new(x + (w / 2) - 20, y - 13),
        Point::new(x + (w / 2) - 22, y),
        canvas,
    );

    draw_line(
        Point::new(x - (w / 2) + 22, y),
        Point::new(x - (w / 2) + 5, y),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x + (w / 2) - 22, y),
        Point::new(x + (w / 2) - 16, y + 35),
        Point::new(x - (w / 2) + 10, y + 40),
        Point::new(x - (w / 2) + 5, y + 60),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(x + (w / 2) - 5, y),
        Point::new(x + (w / 2) - 5, y + 30),
        Point::new(x - (w / 2) + 58, y + 35),
        Point::new(x - (w / 2) + 40, y + 50),
        canvas,
    );

    draw_line(
        Point::new(x - (w / 2) + 40, y + 50),
        Point::new(w - 10, y + 50),
        canvas,
    );
    draw_line(
        Point::new(x - (w / 2) + 5, y + 60),
        Point::new(x - (w / 2) + 5, h - 5),
        canvas,
    );

    draw_line(
        Point::new(x - (w / 2) + 5, y + 60),
        Point::new(x - (w / 2) + 5, h - 5),
        canvas,
    );

    draw_line(
        Point::new(x - (w / 2) + 5, h - 5),
        Point::new(w - 10, h - 5),
        canvas,
    );

    draw_line(
        Point::new(w - 10, h - 5),
        Point::new(w - 10, y + 50),
        canvas,
    );
}

pub fn draw_digit_3(point: Point, canvas: &mut Canvas<Window>) {
    let x_i = point.x;
    let y_i = point.y;
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    draw_line(Point::new(10, 20), Point::new(70, 20), canvas);
    draw_line(Point::new(10, 20), Point::new(10, 30), canvas);
    draw_line(Point::new(10, 30), Point::new(55, 30), canvas);
    draw_line(Point::new(70, 20), Point::new(70, 30), canvas);

    draw_line(Point::new(55, 30), Point::new(30, 70), canvas);
    draw_line(Point::new(70, 30), Point::new(50, 65), canvas);

    draw_cubic_bezier(
        Point::new(50, 65),
        Point::new(85, 70),
        Point::new(85, 115),
        Point::new(40, h),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(40, h),
        Point::new(30, h - 2),
        Point::new(20, h - 3),
        Point::new(10, h - 10),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(30, 70),
        Point::new(70, 70),
        Point::new(70, 100),
        Point::new(50, 105),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(50, 105),
        Point::new(40, h - 12),
        Point::new(30, h - 13),
        Point::new(18, h - 20),
        canvas,
    );

    draw_line(Point::new(10, h - 10), Point::new(18, h - 20), canvas);
}

pub fn draw_digit_4(point: Point, canvas: &mut Canvas<Window>) {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    draw_line(Point::new(40, 20), Point::new(10, 80), canvas);
    draw_line(Point::new(40, 45), Point::new(25, 80), canvas);
    draw_line(Point::new(40, 45), Point::new(40, 80), canvas);

    draw_line(Point::new(25, 80), Point::new(40, 80), canvas);

    draw_line(Point::new(10, 80), Point::new(10, 90), canvas);
    draw_line(Point::new(10, 90), Point::new(40, 90), canvas);

    draw_line(Point::new(40, 90), Point::new(40, 120), canvas);

    draw_line(Point::new(40, 20), Point::new(55, 20), canvas);
    draw_line(Point::new(55, 20), Point::new(55, 80), canvas);

    draw_line(Point::new(55, 80), Point::new(65, 80), canvas);
    draw_line(Point::new(65, 80), Point::new(65, 90), canvas);
    draw_line(Point::new(65, 90), Point::new(55, 90), canvas);
    draw_line(Point::new(55, 90), Point::new(55, 120), canvas);

    draw_line(Point::new(55, 120), Point::new(65, 120), canvas);
}

pub fn draw_digit_5(canvas: &mut Canvas<Window>) {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);

    draw_cubic_bezier(
        Point::new(35, 60),
        Point::new(85, 65),
        Point::new(85, 115),
        Point::new(40, h),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(40, h),
        Point::new(30, h - 2),
        Point::new(20, h - 3),
        Point::new(10, h - 10),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(20, 70),
        Point::new(70, 70),
        Point::new(70, 100),
        Point::new(50, 105),
        canvas,
    );

    draw_cubic_bezier(
        Point::new(50, 105),
        Point::new(40, h - 12),
        Point::new(30, h - 13),
        Point::new(18, h - 20),
        canvas,
    );

    draw_line(Point::new(10, h - 10), Point::new(18, h - 20), canvas);

    draw_line(Point::new(20, 70), Point::new(25, 20), canvas);
    draw_line(Point::new(25, 20), Point::new(70, 20), canvas);
    draw_line(Point::new(70, 20), Point::new(70, 30), canvas);
    draw_line(Point::new(70, 30), Point::new(37, 30), canvas);
    draw_line(Point::new(37, 30), Point::new(35, 60), canvas);
}

fn draw_digit_6(canvas: &mut Canvas<Window>) {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    draw_line(Point::new(0, 0), Point::new(80, 0), canvas);
    draw_line(Point::new(0, 0), Point::new(0, 120), canvas);
    draw_line(Point::new(80, 0), Point::new(80, 120), canvas);
    draw_line(Point::new(0, 120), Point::new(80, 120), canvas);
}
