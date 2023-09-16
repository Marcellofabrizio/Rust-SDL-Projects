use crate::graphics;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

    pub fn draw_v1(&self, canvas: &mut Canvas<Window>) {
        match self.number {
            '0' => graphics::draw_digit_0(canvas),
            '2' => graphics::draw_digit_2(canvas),
            '3' => graphics::draw_digit_3(canvas),
            '4' => graphics::draw_digit_4(canvas),
            '5' => graphics::draw_digit_5(canvas),
            '6' => graphics::draw_digit_6(canvas),
            '7' => graphics::draw_digit_7(canvas),
            '8' => graphics::draw_digit_8(canvas),
            '9' => graphics::draw_digit_9(canvas),
            _ => println!("Number not recognized: {}", self.number),
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
