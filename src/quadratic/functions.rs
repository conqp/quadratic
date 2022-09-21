pub fn format_sign(number: f64, first: bool) -> String {
    if number < 0.0 {
        if first { "-".to_string() } else { " - ".to_string() }
    } else {
        if first { "".to_string() } else { " + ".to_string() }
    }
}

pub fn format_coefficient(number: f64, omit_one: bool) -> String {
    if number.abs() == 1.0 && omit_one {
        "".to_string()
    } else {
        number.abs().to_string()
    }
}
