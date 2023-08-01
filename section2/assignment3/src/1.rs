fn add_3(a: i32) -> i32 {
    a + 3
}

fn add_5(a: i32) -> i32 {
    a + 5
}

fn times(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let x = 3;
    let y = 4;

    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}
