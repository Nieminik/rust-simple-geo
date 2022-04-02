use std::fmt;

const ZERO: f64 = 0.000_001;

fn is_zero(a: f64) -> bool {
    a.abs() < ZERO
}

fn are_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < ZERO
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Line {
    // ax + by + c = 0
    // a and b are normalized (value and sign)
    a: f64,
    b: f64,
    c: f64,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

impl Line {
    pub fn new(a: f64, b: f64, c: f64) -> Line {
        if b == 0.0 && a == 0.0 {
            panic!("Line.a and Line.b cannot both be 0.0")
        }

        let div = (a.powi(2) + b.powi(2)).sqrt();

        let a = a / div;
        let b = b / div;
        let c = c / div;

        if a > 0.0 || (a == 0.0 && b > 0.0) {
            Line { a, b, c }
        } else {
            Line {
                a: -a,
                b: -b,
                c: -c,
            }
        }
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn intersects_point(&self, p: &Point) -> bool {
        is_zero(self.a * p.x + self.b * p.y + self.c)
    }

    pub fn is_parallel(&self, l: &Line) -> bool {
        are_equal(self.a, l.a)
    }

    pub fn intersects_line(&self, l: &Line) -> bool {
        if !self.is_parallel(l) {
            true
        } else {
            are_equal(self.a, l.a) && are_equal(self.c, l.c)
        }
    }
}
