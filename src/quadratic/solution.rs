use std::fmt;

pub struct Solution {
    pub x1: f64,
    pub x2: f64,
}

impl Solution {
    pub fn new(x1: f64, x2: f64) -> Solution {
        Solution { x1: x1, x2: x2 }
    }

    pub fn error(&self) -> bool {
        self.x1.is_nan() && self.x2.is_nan()
    }

    pub fn to_string(&self) -> String {
        if self.error() {
            "N/A".to_string()
        } else {
            format!("x₁ = {}, x₂ = {}", self.x1, self.x2)
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn abc(a: f64, b: f64, c: f64) -> Solution {
    let root = f64::sqrt(f64::powi(b, 2) - 4.0 * a * c);
    let x1 = (-b + root) / (2f64 * a);
    let x2 = (-b - root) / (2f64 * a);
    Solution::new(x1, x2)
}

pub fn pq(p: f64, q: f64) -> Solution {
    let minus_p_half = -p / 2.0;
    let root = f64::sqrt(f64::powi(p / 2.0, 2) - q);
    let x1 = minus_p_half + root;
    let x2 = minus_p_half - root;
    Solution::new(x1, x2)
}
