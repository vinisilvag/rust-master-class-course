fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y): (f32, f32) = point;

    ((x - 0.0).powf(2.0) + (y - 0.0).powf(2.0)).sqrt()
}

fn main() {
    println!(
        "The distance of the point from the origin is {}",
        print_distance((5.0, 4.0))
    );
}
