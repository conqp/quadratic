pub fn with_sign(number: f64, omit_one: bool, first: bool) -> String {
    if number < 0.0 {
        with_minus(abs_str(number, omit_one), first)
    } else {
        with_plus(abs_str(number, omit_one), first)
    }
}

fn abs_str(number: f64, omit_one: bool) -> String {
    if omit_one {
        empty_if_one(number.abs())
    } else {
        number.abs().to_string()
    }
}

fn with_minus(number: String, first: bool) -> String {
    if first {
        format!("-{}", number)
    } else {
        format!("- {}", number)
    }
}

fn with_plus(number: String, first: bool) -> String {
    if first {
        number
    } else {
        format!("+ {}", number)
    }
}

fn empty_if_one(number: f64) -> String {
    if number == 1.0 {
        "".to_string()
    } else {
        number.to_string()
    }
}
