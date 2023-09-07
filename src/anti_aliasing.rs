mod graphics;

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

/*
    Cool, but wont be really using this.
    Anti-aliasing will fuck up the flood fill
*/
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
            graphics::draw_point(ipart(intery), x, rfpart(intery), canvas);
            graphics::draw_point(ipart(intery) - 1, x, fpart(intery), canvas);
            intery += gradient;
        }
    } else {
        for x in (xpxl1 + 1)..xpxl2 {
            graphics::draw_point(x, ipart(intery), rfpart(intery), canvas);
            graphics::draw_point(x, ipart(intery) - 1, fpart(intery), canvas);
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
}
