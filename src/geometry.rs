use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct BezierCurve {
    pub controll_points: Vec<Point>,
    pub color: f32,
}

pub struct Rectangle {
    pub point_1: Point,
    pub point_2: Point,
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

pub fn draw_wu_line(p_start: Point, p_end: Point, canvas: &mut Canvas<Window>) {
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
    draw_wu_line(p_1, Point::new(p_1.x, p_2.y), canvas);
    draw_wu_line(p_1, Point::new(p_2.x, p_1.y), canvas);

    draw_wu_line(Point::new(p_1.x, p_2.y), p_2, canvas);
    draw_wu_line(p_2, Point::new(p_2.x, p_1.y), canvas);

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
