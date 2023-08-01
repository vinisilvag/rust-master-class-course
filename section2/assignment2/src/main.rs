fn question_one() {
    let p1: (f64, f64) = (10.5, 2.0);
    let p2: (f64, f64) = (0.0, -4.2);

    println!("x-axis diff: {}", (p1.0 - p2.0).abs());
    println!("y-axis diff: {}", (p1.1 - p2.1).abs());
}

fn question_two() {
    let p1: [f64; 2] = [10.5, 2.0];
    let p2: [f64; 2] = [0.0, -4.2];

    println!("x-axis diff: {}", (p1[0] - p2[0]).abs());
    println!("y-axis diff: {}", (p1[1] - p2[1]).abs());
}

fn question_three() {
    let p1: (f64, f64) = (4.0, 3.0);
    let p2: (f64, f64) = (5.0, 4.5);

    let distance: f64 = ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt();

    println!("distance: {}", distance);
}

fn question_four() {
    let a: i32 = -15;
    let b: i32 = 170;
    let my_name: &str = "Michael";

    println!(
        "My name is {}, and the multiplication result is {}",
        my_name,
        a * b
    );
}

fn main() {
    question_one();
    question_two();
    question_three();
    question_four();
}
