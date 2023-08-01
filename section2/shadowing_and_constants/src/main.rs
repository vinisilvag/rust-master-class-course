// rust assume snake_case for variables

fn main() {
    // declaration of multiple variables in a single line
    let (first_number, second_number) = (250, 480.22);

    // more readability
    let large_number: i32 = 1_000_000;

    // overflow
    // let overflow: u8 = 256;

    let x: i32 = 55;

    println!("Binary: {:b}", x);
    println!("Octal: {:o}", x);
    println!("Hexadecimal: {:X}", x);

    // type casting - applied only to the command where the "as {type}" appears
    let y: i32 = 8;
    let w: f64 = 16.44;

    let v = y + w as i32;
    let z = y as f64 + w;

    println!("{} {}", v, z);

    // -------------------------------------

    // shadowing
    // by the same name
    let shad: i32 = 8;
    let shad: i32 = 8 * 8;

    println!("{}", shad);

    // by mutable x immutable
    let mut p: i32 = 5;
    let p: i32 = 5 * 5;

    // throw an error
    // p = 24;

    // changing the type of the variable
    let q: i32 = 32;
    let q: char = 'A';

    println!("{}", q);

    // shadowing by code blocks
    let mut l: i32 = 65;
    {
        // l = 60; -> will modify the variable outside the scope too
        let l: i32 = 60;
        println!("l inside {{}} = {}", l);
        let r: i32 = 1;
    }

    // throw an error: r only exists inside the {} scope
    // println!("{}", r);

    println!("l outside {{}} = {}", l);

    // -------------------------------------

    // constants
    // always immutable, unable to put the mut keyword
    // type need to be annotated by the programmer
    // uppercase letters

    const MAX_SALARY: f64 = 1_000_000.0;
}
