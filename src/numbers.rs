use crate::graphics::{self, translate_number, CubicBezierCurve, Line};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const DEFAULT_NUM_WIDTH: i32 = 80;
const DEFAULT_NUM_HEIGHT: i32 = 120;

pub struct NumberSeries {
    pub x: i32,
    pub y: i32,
    pub number_str: String,
    pub numbers: Vec<Number>,
}

impl NumberSeries {
    pub fn new(x: i32, y: i32, number_str: String, scale: f32, angle: f32) -> Self {
        if number_str.is_empty() {
            panic!("Empty number string!!!!");
        }

        let mut numbers: Vec<Number> = Vec::new();

        let num_x = DEFAULT_NUM_WIDTH;
        for (i, c) in number_str.chars().enumerate() {
            numbers.push(create_digit(c, num_x * i as i32, y).unwrap()); // TODO: at least treat the fucking error
        }

        for (i, number) in numbers.iter_mut().enumerate() {
            translate_number(number, number.x + x, number.y + y);
        }

        for (i, num) in numbers.iter_mut().enumerate() {
            graphics::scale_number(num, scale, Point::new(x, y));
        }

        for (i, num) in numbers.iter_mut().enumerate() {
            graphics::rotate_number(num, angle, Point::new(x, y));
        }

        NumberSeries {
            x: x,
            y: y,
            number_str: number_str,
            numbers: numbers,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, fill: bool) {
        for num in self.numbers.iter_mut() {
            num.draw(canvas, fill);
        }
    }
}

pub struct Number {
    pub number: char,
    pub w: i32,
    pub h: i32,
    pub x: i32,
    pub y: i32,
    pub center: Point,
    pub lines: Vec<graphics::Line>,
    pub bezier_curves: Vec<graphics::CubicBezierCurve>,
}

impl Number {
    pub fn new(
        number: char,
        w: i32,
        h: i32,
        x: i32,
        y: i32,
        center: Point,
        lines: Option<Vec<graphics::Line>>,
        bezier_curves: Option<Vec<graphics::CubicBezierCurve>>,
    ) -> Self {
        Number {
            number: number,
            w: w,
            h: h,
            x: x,
            y: y,
            center,
            lines: lines.unwrap_or(Vec::new()),
            bezier_curves: bezier_curves.unwrap_or(Vec::new()),
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, fill: bool) {
        for line in self.lines.iter_mut() {
            line.draw(canvas);
        }

        for bezier_curve in self.bezier_curves.iter_mut() {
            bezier_curve.draw(canvas);
        }

        if fill {
            graphics::flood_fill(Point::new(self.x, self.y), 0, canvas);
        }
    }
}

pub fn create_digit(number: char, x: i32, y: i32) -> Option<Number> {
    match number {
        '0' => Some(create_digit_0(x, y)),
        '1' => Some(create_digit_1(x, y)),
        '2' => Some(create_digit_2(x, y)),
        '3' => Some(create_digit_3(x, y)),
        '4' => Some(create_digit_4(x, y)),
        '5' => Some(create_digit_5(x, y)),
        '6' => Some(create_digit_6(x, y)),
        '7' => Some(create_digit_7(x, y)),
        '8' => Some(create_digit_8(x, y)),
        '9' => Some(create_digit_9(x, y)),
        _ => {
            println!("Number not recognized: {}", number);
            return None;
        }
    }
}

pub fn create_digit_0(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(5, 70),
        Point::new(10, 0),
        Point::new(70, 0),
        Point::new(75, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(20, 70),
        Point::new(25, 13),
        Point::new(55, 13),
        Point::new(60, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(5, 70),
        Point::new(10, 137),
        Point::new(70, 137),
        Point::new(75, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(20, 70),
        Point::new(25, 124),
        Point::new(55, 124),
        Point::new(60, 70),
    ]));

    Number::new(
        '0',
        w,
        h,
        x,
        y,
        Point::new(40, 25),
        None,
        Some(bezier_curves),
    )
}

pub fn create_digit_1(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(vec![Point::new(32, 20), Point::new(10, 55)]));
    lines.push(Line::new(vec![Point::new(10, 55), Point::new(32, 55)]));
    lines.push(Line::new(vec![Point::new(32, 20), Point::new(53, 20)]));
    lines.push(Line::new(vec![
        Point::new(53, 20),
        Point::new(53, DEFAULT_NUM_HEIGHT),
    ]));
    lines.push(Line::new(vec![
        Point::new(32, 55),
        Point::new(32, DEFAULT_NUM_HEIGHT),
    ]));

    lines.push(Line::new(vec![
        Point::new(32, 55),
        Point::new(32, DEFAULT_NUM_HEIGHT),
    ]));

    lines.push(Line::new(vec![
        Point::new(32, DEFAULT_NUM_HEIGHT),
        Point::new(54, DEFAULT_NUM_HEIGHT),
    ]));

    Number::new('1', w, h, x, y, Point::new(40, 30), Some(lines), None)
}

pub fn create_digit_2(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40 - (w / 2) + 5, 50),
        Point::new(40 - (w / 2) + 3, 20),
        Point::new(40 + (w / 2) - 3, 20),
        Point::new(40 + (w / 2) - 5, 50),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40 - (w / 2) + 22, 50),
        Point::new(40 - (w / 2) + 20, 37),
        Point::new(40 + (w / 2) - 20, 37),
        Point::new(40 + (w / 2) - 22, 50),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40 + (w / 2) - 22, 50),
        Point::new(40 + (w / 2) - 16, 85),
        Point::new(40 - (w / 2) + 10, 90),
        Point::new(40 - (w / 2) + 5, 110),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40 + (w / 2) - 5, 50),
        Point::new(40 + (w / 2) - 5, DEFAULT_NUM_WIDTH),
        Point::new(40 - (w / 2) + 58, 85),
        Point::new(40 - (w / 2) + 40, 100),
    ]));

    lines.push(Line::new(vec![
        Point::new(40 - (w / 2) + 22, 50),
        Point::new(40 - (w / 2) + 5, 50),
    ]));

    lines.push(Line::new(vec![
        Point::new(40 - (w / 2) + 40, 100),
        Point::new(w - 10, 100),
    ]));

    lines.push(Line::new(vec![
        Point::new(40 - (w / 2) + 5, 110),
        Point::new(40 - (w / 2) + 5, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(40 - (w / 2) + 5, 110),
        Point::new(40 - (w / 2) + 5, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(40 - (w / 2) + 5, h - 5),
        Point::new(w - 10, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(w - 10, h - 5),
        Point::new(w - 10, 100),
    ]));

    Number::new(
        '2',
        w,
        h,
        x,
        y,
        Point::new(40, 30),
        Some(lines),
        Some(bezier_curves),
    )
}

pub fn create_digit_3(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    lines.push(Line::new(vec![Point::new(10, 20), Point::new(70, 20)]));
    lines.push(Line::new(vec![Point::new(10, 20), Point::new(10, 30)]));
    lines.push(Line::new(vec![Point::new(10, 30), Point::new(55, 30)]));
    lines.push(Line::new(vec![Point::new(70, 20), Point::new(70, 30)]));

    lines.push(Line::new(vec![Point::new(55, 30), Point::new(30, 70)]));
    lines.push(Line::new(vec![Point::new(70, 30), Point::new(50, 65)]));

    lines.push(Line::new(vec![
        Point::new(10, h - 10),
        Point::new(18, h - 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(50, 65),
        Point::new(85, 70),
        Point::new(85, 115),
        Point::new(40, h),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40, h),
        Point::new(30, h - 2),
        Point::new(20, h - 3),
        Point::new(10, h - 10),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 70),
        Point::new(70, 70),
        Point::new(70, 100),
        Point::new(50, 105),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(50, 105),
        Point::new(40, h - 12),
        Point::new(30, h - 13),
        Point::new(18, h - 20),
    ]));

    Number::new(
        '3',
        w,
        h,
        x,
        y,
        Point::new(40, 15),
        Some(lines),
        Some(bezier_curves),
    )
}

pub fn create_digit_4(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(vec![
        Point::new(40, 20),
        Point::new(10, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(40, 45),
        Point::new(25, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(40, 45),
        Point::new(40, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(25, DEFAULT_NUM_WIDTH),
        Point::new(40, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(10, DEFAULT_NUM_WIDTH),
        Point::new(10, 90),
    ]));
    lines.push(Line::new(vec![Point::new(10, 90), Point::new(40, 90)]));
    lines.push(Line::new(vec![
        Point::new(40, 90),
        Point::new(40, DEFAULT_NUM_HEIGHT),
    ]));
    lines.push(Line::new(vec![Point::new(40, 20), Point::new(55, 20)]));
    lines.push(Line::new(vec![
        Point::new(55, 20),
        Point::new(55, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(55, DEFAULT_NUM_WIDTH),
        Point::new(65, DEFAULT_NUM_WIDTH),
    ]));
    lines.push(Line::new(vec![
        Point::new(65, DEFAULT_NUM_WIDTH),
        Point::new(65, 90),
    ]));
    lines.push(Line::new(vec![Point::new(65, 90), Point::new(55, 90)]));
    lines.push(Line::new(vec![
        Point::new(55, 90),
        Point::new(55, DEFAULT_NUM_HEIGHT),
    ]));
    lines.push(Line::new(vec![
        Point::new(55, DEFAULT_NUM_HEIGHT),
        Point::new(40, DEFAULT_NUM_HEIGHT),
    ]));

    Number::new('4', w, h, x, y, Point::new(40, 15), Some(lines), None)
}

pub fn create_digit_5(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(35, 60),
        Point::new(85, 65),
        Point::new(85, 115),
        Point::new(40, h),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(40, h),
        Point::new(30, h - 2),
        Point::new(20, h - 3),
        Point::new(10, h - 10),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(20, 70),
        Point::new(70, 70),
        Point::new(70, 100),
        Point::new(50, 105),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(50, 105),
        Point::new(40, h - 12),
        Point::new(30, h - 13),
        Point::new(18, h - 20),
    ]));

    lines.push(Line::new(vec![
        Point::new(10, h - 10),
        Point::new(18, h - 20),
    ]));
    lines.push(Line::new(vec![Point::new(20, 70), Point::new(25, 20)]));
    lines.push(Line::new(vec![Point::new(25, 20), Point::new(70, 20)]));
    lines.push(Line::new(vec![Point::new(70, 20), Point::new(70, 30)]));
    lines.push(Line::new(vec![Point::new(70, 30), Point::new(37, 30)]));
    lines.push(Line::new(vec![Point::new(37, 30), Point::new(35, 60)]));

    Number::new(
        '5',
        w,
        h,
        x,
        y,
        Point::new(40, 15),
        Some(lines),
        Some(bezier_curves),
    )
}

pub fn create_digit_6(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 20),
        Point::new(14, 50),
        Point::new(12, 60),
        Point::new(10, DEFAULT_NUM_WIDTH),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(10, DEFAULT_NUM_WIDTH),
        Point::new(15, 130),
        Point::new(65, 130),
        Point::new(70, 90),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(70, 90),
        Point::new(69, 55),
        Point::new(35, 55),
        Point::new(30, 60),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 60),
        Point::new(33, 50),
        Point::new(36, 30),
        Point::new(43, 20),
    ]));

    lines.push(Line::new(vec![Point::new(43, 20), Point::new(30, 20)]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(25, 90),
        Point::new(30, 110),
        Point::new(50, 110),
        Point::new(55, 90),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(25, 90),
        Point::new(30, 70),
        Point::new(50, 70),
        Point::new(55, 90),
    ]));

    Number::new(
        '6',
        w,
        h,
        x,
        y,
        Point::new(40, 15),
        Some(lines),
        Some(bezier_curves),
    )
}

pub fn create_digit_7(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(vec![Point::new(10, 20), Point::new(70, 20)]));
    lines.push(Line::new(vec![Point::new(10, 20), Point::new(10, 30)]));
    lines.push(Line::new(vec![Point::new(70, 20), Point::new(70, 30)]));
    lines.push(Line::new(vec![Point::new(25, 30), Point::new(55, 30)]));
    lines.push(Line::new(vec![Point::new(25, 30), Point::new(25, 40)]));
    lines.push(Line::new(vec![Point::new(25, 40), Point::new(10, 40)]));
    lines.push(Line::new(vec![Point::new(10, 40), Point::new(10, 20)]));
    lines.push(Line::new(vec![
        Point::new(70, 30),
        Point::new(25, DEFAULT_NUM_HEIGHT),
    ]));
    lines.push(Line::new(vec![
        Point::new(55, 30),
        Point::new(10, DEFAULT_NUM_HEIGHT),
    ]));
    lines.push(Line::new(vec![
        Point::new(10, DEFAULT_NUM_HEIGHT),
        Point::new(25, DEFAULT_NUM_HEIGHT),
    ]));

    Number::new('7', w, h, x, y, Point::new(40, 15), Some(lines), None)
}

pub fn create_digit_8(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 20),
        Point::new(35, 17),
        Point::new(45, 17),
        Point::new(50, 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 20),
        Point::new(10, 30),
        Point::new(10, 60),
        Point::new(30, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 30, 20),
        Point::new(DEFAULT_NUM_WIDTH - 10, 30),
        Point::new(DEFAULT_NUM_WIDTH - 10, 60),
        Point::new(DEFAULT_NUM_WIDTH - 30, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, (DEFAULT_NUM_HEIGHT - 20) + 18),
        Point::new(10, (DEFAULT_NUM_HEIGHT - 30) + 18),
        Point::new(10, (DEFAULT_NUM_HEIGHT - 60) + 18),
        Point::new(30, (DEFAULT_NUM_HEIGHT - 70) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 20) + 18),
        Point::new(DEFAULT_NUM_WIDTH - 10, (DEFAULT_NUM_HEIGHT - 30) + 18),
        Point::new(DEFAULT_NUM_WIDTH - 10, (DEFAULT_NUM_HEIGHT - 60) + 18),
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 70) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, (DEFAULT_NUM_HEIGHT - 20) + 18),
        Point::new(35, (DEFAULT_NUM_HEIGHT - 17) + 18),
        Point::new(45, (DEFAULT_NUM_HEIGHT - 17) + 18),
        Point::new(50, (DEFAULT_NUM_HEIGHT - 20) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, 45),
        Point::new(DEFAULT_NUM_WIDTH - 30, 25),
        Point::new(DEFAULT_NUM_WIDTH - 50, 25),
        Point::new(DEFAULT_NUM_WIDTH - 55, 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, 45),
        Point::new(DEFAULT_NUM_WIDTH - 30, 65),
        Point::new(DEFAULT_NUM_WIDTH - 50, 65),
        Point::new(DEFAULT_NUM_WIDTH - 55, 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, 140 - 45),
        Point::new(DEFAULT_NUM_WIDTH - 30, 140 - 25),
        Point::new(DEFAULT_NUM_WIDTH - 50, 140 - 25),
        Point::new(DEFAULT_NUM_WIDTH - 55, 140 - 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, 140 - 45),
        Point::new(DEFAULT_NUM_WIDTH - 30, 140 - 65),
        Point::new(DEFAULT_NUM_WIDTH - 50, 140 - 65),
        Point::new(DEFAULT_NUM_WIDTH - 55, 140 - 45),
    ]));

    Number::new(
        '8',
        w,
        h,
        x,
        y,
        Point::new(40, 15),
        None,
        Some(bezier_curves),
    )
}

pub fn create_digit_9(x: i32, y: i32) -> Number {
    let w = DEFAULT_NUM_WIDTH;
    let h = DEFAULT_NUM_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 30, DEFAULT_NUM_HEIGHT),
        Point::new(DEFAULT_NUM_WIDTH - 14, (DEFAULT_NUM_HEIGHT - 50) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 12, (DEFAULT_NUM_HEIGHT - 60) + 20),
        Point::new(
            DEFAULT_NUM_WIDTH - 10,
            (DEFAULT_NUM_HEIGHT - DEFAULT_NUM_WIDTH) + 20,
        ),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(
            DEFAULT_NUM_WIDTH - 10,
            (DEFAULT_NUM_HEIGHT - DEFAULT_NUM_WIDTH) + 20,
        ),
        Point::new(DEFAULT_NUM_WIDTH - 15, (DEFAULT_NUM_HEIGHT - 130) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 65, (DEFAULT_NUM_HEIGHT - 130) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 70, (DEFAULT_NUM_HEIGHT - 90) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 70, (DEFAULT_NUM_HEIGHT - 90) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 69, (DEFAULT_NUM_HEIGHT - 55) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 35, (DEFAULT_NUM_HEIGHT - 55) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 60) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 60) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 33, (DEFAULT_NUM_HEIGHT - 50) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 36, (DEFAULT_NUM_HEIGHT - 30) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 43, DEFAULT_NUM_HEIGHT),
    ]));

    lines.push(Line::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 43, DEFAULT_NUM_HEIGHT),
        Point::new(DEFAULT_NUM_WIDTH - 30, DEFAULT_NUM_HEIGHT),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, (DEFAULT_NUM_HEIGHT - 90) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 110) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 50, (DEFAULT_NUM_HEIGHT - 110) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 55, (DEFAULT_NUM_HEIGHT - 90) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(DEFAULT_NUM_WIDTH - 25, (DEFAULT_NUM_HEIGHT - 90) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 30, (DEFAULT_NUM_HEIGHT - 70) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 50, (DEFAULT_NUM_HEIGHT - 70) + 20),
        Point::new(DEFAULT_NUM_WIDTH - 55, (DEFAULT_NUM_HEIGHT - 90) + 20),
    ]));

    Number::new(
        '9',
        w,
        h,
        x,
        y,
        Point::new(40, 15),
        Some(lines),
        Some(bezier_curves),
    )
}
