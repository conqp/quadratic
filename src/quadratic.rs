use std::fmt;

mod functions;
use self::functions::with_sign;

mod solution;
use self::solution::abc;
use self::solution::pq;
use self::solution::Solution;

pub struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    pub fn new(a: f64, b: f64, c: f64) -> QuadraticEquation {
        QuadraticEquation { a: a, b: b, c: c }
    }

    pub fn solve(&self) -> Solution {
        if self.a == 1.0 {
            pq(self.b, self.c)
        } else {
            abc(self.a, self.b, self.c)
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = Vec::new();

        if self.a != 0.0 {
            result.push(format!("{}x²", with_sign(self.a, true, result.is_empty())));
        }

        if self.b != 0.0 {
            result.push(format!("{}x", with_sign(self.b, true, result.is_empty())));
        }

        if self.c != 0.0 {
            result.push(with_sign(self.c, false, result.is_empty()));
        }

        result.join(" ")
    }
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
