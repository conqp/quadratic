use std::fmt;

mod functions;
use self::functions::format_coefficient;
use self::functions::format_sign;

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
            result.push(format_sign(self.a, result.is_empty()));
            result.push(format_coefficient(self.a, true));
            result.push("x²".to_string());
        }

        if self.b != 0.0 {
            result.push(format_sign(self.b, result.is_empty()));
            result.push(format_coefficient(self.b, true));
            result.push("x".to_string());
        }

        if self.c != 0.0 {
            result.push(format_sign(self.c, result.is_empty()));
            result.push(format_coefficient(self.c, false));
        }

        result.join("")
    }
}

impl fmt::Debug for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuadraticEquation")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .finish()
    }
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
