use std::error::Error;

use crate::graphics::{self, translate, translate_number, CubicBezierCurve, Line};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct NumberSeries {
    pub x: i32,
    pub y: i32,
    pub number_str: String,
    pub numbers: Vec<Number>,
}

impl NumberSeries {
    pub fn new(x: i32, y: i32, number_str: String) -> Self {
        if number_str.is_empty() {
            panic!("Empty number string!!!!");
        }

        let mut numbers: Vec<Number> = Vec::new();

        for c in number_str.chars() {
            numbers.push(create_digit(c).unwrap()); // TODO: at least treat the fucking error
        }

        for (i, number) in numbers.iter_mut().enumerate() {
            translate_number(number, number.w * i as i32, number.h);
        }

        NumberSeries {
            x: x,
            y: y,
            number_str: number_str,
            numbers: numbers,
        }
    }
}

pub struct Number {
    pub number: char,
    pub w: i32,
    pub h: i32,
    pub lines: Vec<graphics::Line>,
    pub bezier_curves: Vec<graphics::CubicBezierCurve>,
}

impl Number {
    pub fn new(
        number: char,
        w: i32,
        h: i32,
        lines: Option<Vec<graphics::Line>>,
        bezier_curves: Option<Vec<graphics::CubicBezierCurve>>,
    ) -> Self {
        Number {
            number: number,
            w: w,
            h: h,
            lines: lines.unwrap_or(Vec::new()),
            bezier_curves: bezier_curves.unwrap_or(Vec::new()),
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for line in self.lines.iter_mut() {
            line.draw(canvas);
        }

        for bezier_curve in self.bezier_curves.iter_mut() {
            bezier_curve.draw(canvas);
        }
    }
}

pub fn create_digit(number: char) -> Option<Number> {
    match number {
        '0' => Some(create_digit_0()),
        '1' => Some(create_digit_1()),
        '2' => Some(create_digit_2()),
        '3' => Some(create_digit_3()),
        '4' => Some(create_digit_4()),
        '5' => Some(create_digit_5()),
        '6' => Some(create_digit_6()),
        '7' => Some(create_digit_7()),
        '8' => Some(create_digit_8()),
        '9' => Some(create_digit_9()),
        _ => {
            println!("Number not recognized: {}", number);
            return None;
        }
    }
}

pub fn create_digit_0() -> Number {
    let w = 80;
    let h = 120;

    let x = 40;
    let y = 10;

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

    Number::new('0', w, h, None, Some(bezier_curves))
}

pub fn create_digit_1() -> Number {
    let w = 80;
    let h = 120;

    let x = 40;
    let y = 20;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(vec![Point::new(32, y), Point::new(10, y + 35)]));
    lines.push(Line::new(vec![
        Point::new(10, y + 35),
        Point::new(x - 8, y + 35),
    ]));
    lines.push(Line::new(vec![Point::new(x - 8, y), Point::new(x + 13, y)]));
    lines.push(Line::new(vec![
        Point::new(x + 13, y),
        Point::new(x + 13, h),
    ]));
    lines.push(Line::new(vec![
        Point::new(x - 8, y + 35),
        Point::new(x - 8, h),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - 8, y + 35),
        Point::new(x - 8, h),
    ]));

    lines.push(Line::new(vec![
        Point::new(32, 120),
        Point::new(x + 13, 120),
    ]));

    Number::new('1', w, h, Some(lines), None)
}

pub fn create_digit_2() -> Number {
    let w = 80;
    let h = 120;

    let x = 40;
    let y = 50;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(x - (w / 2) + 5, y),
        Point::new(x - (w / 2) + 3, y - 30),
        Point::new(x + (w / 2) - 3, y - 30),
        Point::new(x + (w / 2) - 5, y),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(x - (w / 2) + 22, y),
        Point::new(x - (w / 2) + 20, y - 13),
        Point::new(x + (w / 2) - 20, y - 13),
        Point::new(x + (w / 2) - 22, y),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(x + (w / 2) - 22, y),
        Point::new(x + (w / 2) - 16, y + 35),
        Point::new(x - (w / 2) + 10, y + 40),
        Point::new(x - (w / 2) + 5, y + 60),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(x + (w / 2) - 5, y),
        Point::new(x + (w / 2) - 5, y + 30),
        Point::new(x - (w / 2) + 58, y + 35),
        Point::new(x - (w / 2) + 40, y + 50),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - (w / 2) + 22, y),
        Point::new(x - (w / 2) + 5, y),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - (w / 2) + 40, y + 50),
        Point::new(w - 10, y + 50),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - (w / 2) + 5, y + 60),
        Point::new(x - (w / 2) + 5, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - (w / 2) + 5, y + 60),
        Point::new(x - (w / 2) + 5, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(x - (w / 2) + 5, h - 5),
        Point::new(w - 10, h - 5),
    ]));

    lines.push(Line::new(vec![
        Point::new(w - 10, h - 5),
        Point::new(w - 10, y + 50),
    ]));

    Number::new('2', w, h, Some(lines), Some(bezier_curves))
}

pub fn create_digit_3() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

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

    Number::new('3', w, h, Some(lines), Some(bezier_curves))
}

pub fn create_digit_4() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(vec![Point::new(40, 20), Point::new(10, 80)]));
    lines.push(Line::new(vec![Point::new(40, 45), Point::new(25, 80)]));
    lines.push(Line::new(vec![Point::new(40, 45), Point::new(40, 80)]));
    lines.push(Line::new(vec![Point::new(25, 80), Point::new(40, 80)]));
    lines.push(Line::new(vec![Point::new(10, 80), Point::new(10, 90)]));
    lines.push(Line::new(vec![Point::new(10, 90), Point::new(40, 90)]));
    lines.push(Line::new(vec![Point::new(40, 90), Point::new(40, 120)]));
    lines.push(Line::new(vec![Point::new(40, 20), Point::new(55, 20)]));
    lines.push(Line::new(vec![Point::new(55, 20), Point::new(55, 80)]));
    lines.push(Line::new(vec![Point::new(55, 80), Point::new(65, 80)]));
    lines.push(Line::new(vec![Point::new(65, 80), Point::new(65, 90)]));
    lines.push(Line::new(vec![Point::new(65, 90), Point::new(55, 90)]));
    lines.push(Line::new(vec![Point::new(55, 90), Point::new(55, 120)]));
    lines.push(Line::new(vec![Point::new(55, 120), Point::new(40, 120)]));

    Number::new('4', w, h, Some(lines), None)
}

pub fn create_digit_5() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

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

    Number::new('5', w, h, Some(lines), Some(bezier_curves))
}

pub fn create_digit_6() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, 20),
        Point::new(14, 50),
        Point::new(12, 60),
        Point::new(10, 80),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(10, 80),
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

    Number::new('6', w, h, Some(lines), Some(bezier_curves))
}

pub fn create_digit_7() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    lines.push(Line::new(vec![Point::new(10, 20), Point::new(70, 20)]));
    lines.push(Line::new(vec![Point::new(10, 20), Point::new(10, 30)]));
    lines.push(Line::new(vec![Point::new(70, 20), Point::new(70, 30)]));
    lines.push(Line::new(vec![Point::new(25, 30), Point::new(55, 30)]));
    lines.push(Line::new(vec![Point::new(25, 30), Point::new(25, 40)]));
    lines.push(Line::new(vec![Point::new(25, 40), Point::new(10, 40)]));
    lines.push(Line::new(vec![Point::new(10, 40), Point::new(10, 20)]));
    lines.push(Line::new(vec![Point::new(70, 30), Point::new(25, 120)]));
    lines.push(Line::new(vec![Point::new(55, 30), Point::new(10, 120)]));
    lines.push(Line::new(vec![Point::new(10, 120), Point::new(25, 120)]));

    Number::new('7', w, h, Some(lines), None)
}

pub fn create_digit_8() -> Number {
    let w = 80;
    let h = 120;

    let mut lines: Vec<Line> = Vec::new();
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
        Point::new(80 - 30, 20),
        Point::new(80 - 10, 30),
        Point::new(80 - 10, 60),
        Point::new(80 - 30, 70),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, (120 - 20) + 18),
        Point::new(10, (120 - 30) + 18),
        Point::new(10, (120 - 60) + 18),
        Point::new(30, (120 - 70) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 30, (120 - 20) + 18),
        Point::new(80 - 10, (120 - 30) + 18),
        Point::new(80 - 10, (120 - 60) + 18),
        Point::new(80 - 30, (120 - 70) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(30, (120 - 20) + 18),
        Point::new(35, (120 - 17) + 18),
        Point::new(45, (120 - 17) + 18),
        Point::new(50, (120 - 20) + 18),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, 45),
        Point::new(80 - 30, 25),
        Point::new(80 - 50, 25),
        Point::new(80 - 55, 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, 45),
        Point::new(80 - 30, 65),
        Point::new(80 - 50, 65),
        Point::new(80 - 55, 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, 140 - 45),
        Point::new(80 - 30, 140 - 25),
        Point::new(80 - 50, 140 - 25),
        Point::new(80 - 55, 140 - 45),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, 140 - 45),
        Point::new(80 - 30, 140 - 65),
        Point::new(80 - 50, 140 - 65),
        Point::new(80 - 55, 140 - 45),
    ]));

    Number::new('8', w, h, None, Some(bezier_curves))
}

pub fn create_digit_9() -> Number {
    let w = 80;
    let h = 120;
    let x = 40;
    let y = 10;

    let mut lines: Vec<Line> = Vec::new();
    let mut bezier_curves: Vec<CubicBezierCurve> = Vec::new();

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 30, 120),
        Point::new(80 - 14, (120 - 50) + 20),
        Point::new(80 - 12, (120 - 60) + 20),
        Point::new(80 - 10, (120 - 80) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 10, (120 - 80) + 20),
        Point::new(80 - 15, (120 - 130) + 20),
        Point::new(80 - 65, (120 - 130) + 20),
        Point::new(80 - 70, (120 - 90) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 70, (120 - 90) + 20),
        Point::new(80 - 69, (120 - 55) + 20),
        Point::new(80 - 35, (120 - 55) + 20),
        Point::new(80 - 30, (120 - 60) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 30, (120 - 60) + 20),
        Point::new(80 - 33, (120 - 50) + 20),
        Point::new(80 - 36, (120 - 30) + 20),
        Point::new(80 - 43, 120),
    ]));

    lines.push(Line::new(vec![
        Point::new(80 - 43, 120),
        Point::new(80 - 30, 120),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, (120 - 90) + 20),
        Point::new(80 - 30, (120 - 110) + 20),
        Point::new(80 - 50, (120 - 110) + 20),
        Point::new(80 - 55, (120 - 90) + 20),
    ]));

    bezier_curves.push(CubicBezierCurve::new(vec![
        Point::new(80 - 25, (120 - 90) + 20),
        Point::new(80 - 30, (120 - 70) + 20),
        Point::new(80 - 50, (120 - 70) + 20),
        Point::new(80 - 55, (120 - 90) + 20),
    ]));

    Number::new('9', w, h, Some(lines), Some(bezier_curves))
}
