use std::fmt;

#[derive(Clone, Debug)]
pub struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    pub fn new(a: f64, b: f64, c: f64) -> QuadraticEquation {
        QuadraticEquation { a, b, c }
    }

    pub fn solve(&self) -> Solution {
        if self.a == 0.0 {
            bc(self.b, self.c)
        } else if self.a == 1.0 {
            pq(self.b, self.c)
        } else {
            abc(self.a, self.b, self.c)
        }
    }
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        if self.a != 0.0 {
            write!(
                f,
                "{}{}x²",
                format_sign(self.a, first),
                format_coefficient(self.a, true)
            )?;
            first = false;
        }

        if self.b != 0.0 {
            write!(
                f,
                "{}{}x",
                format_sign(self.b, first),
                format_coefficient(self.b, true)
            )?;
            first = false;
        }

        if self.c != 0.0 {
            write!(
                f,
                "{}{}",
                format_sign(self.c, first),
                format_coefficient(self.c, false)
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Solution {
    x1: f64,
    x2: f64,
}

impl Solution {
    pub fn new(x1: f64, x2: f64) -> Solution {
        Solution { x1, x2 }
    }

    pub fn is_empty(&self) -> bool {
        self.x1.is_nan() && self.x2.is_nan()
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "N/A")
        } else {
            write!(f, "x₁ = {}, x₂ = {}", self.x1, self.x2)
        }
    }
}

pub fn abc(a: f64, b: f64, c: f64) -> Solution {
    let root = f64::sqrt(f64::powi(b, 2) - 4.0 * a * c);
    Solution::new((-b + root) / (2f64 * a), (-b - root) / (2f64 * a))
}

pub fn bc(b: f64, c: f64) -> Solution {
    Solution::new(-c / b, f64::NAN)
}

pub fn pq(p: f64, q: f64) -> Solution {
    let minus_p_half = -p / 2.0;
    let root = f64::sqrt(f64::powi(p / 2.0, 2) - q);
    Solution::new(minus_p_half + root, minus_p_half - root)
}

fn format_coefficient(number: f64, omit_one: bool) -> String {
    if number.abs() == 1.0 && omit_one {
        "".to_string()
    } else {
        number.abs().to_string()
    }
}

fn format_sign(number: f64, first: bool) -> String {
    if number < 0.0 {
        if first {
            "-".to_string()
        } else {
            " - ".to_string()
        }
    } else if first {
        "".to_string()
    } else {
        " + ".to_string()
    }
}
