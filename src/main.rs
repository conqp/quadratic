mod quadratic;
use quadratic::QuadraticEquation;

fn main() {
    let equation1 = QuadraticEquation::new(3.0, 3.0, -18.0);
    let equation2 = QuadraticEquation::new(1.0, 3.0, -18.0);
    let equation3 = QuadraticEquation::new(-4.0, -5.0, 12.0);
    let equation4 = QuadraticEquation::new(1.0, 0.0, -25.0);
    let equation5 = QuadraticEquation::new(-1.0, 0.0, 1.0);
    let equation6 = QuadraticEquation::new(0.0, 5.0, 1.0);
    println!("The solutions of {} are {}", equation1, equation1.solve());
    println!("The solutions of {} are {}", equation2, equation2.solve());
    println!("The solutions of {} are {}", equation3, equation3.solve());
    println!("The solutions of {} are {}", equation4, equation4.solve());
    println!("The solutions of {} are {}", equation5, equation5.solve());
    println!("The solutions of {} are {}", equation6, equation6.solve());
    println!("Debugging equation 1: {:#?}", equation1);
    println!("Debugging solution 1: {:#?}", equation1.solve());
}
