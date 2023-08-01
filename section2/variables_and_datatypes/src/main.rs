fn main() {
    // naming variables:
    // only letters, digits and underscore
    // must begin with letter or underscore only
    // case sensitive (capital x != small x)

    let x: i32 = 15;
    let y: f32 = 15.5;

    println!("{} {}", x, y);

    let mut mutable_x: i32 = 16;

    println!("{}", mutable_x);

    mutable_x = 17;

    println!("{}", mutable_x);

    // basic datatypes (scalar):
    // i8, i16, i32, i64 - integer signed
    // u8, u16, u32, u64 - integer unsigned
    // f32, f64 - floating pointers (real numbers)
    // char, str, bool

    // max values

    println!("i8 = {}", std::i8::MAX);
    println!("i64 = {}", std::i64::MAX);

    // min values

    println!("i8 = {}", std::i8::MIN);
    println!("i64 = {}", std::i64::MIN);

    // std -> standard library auto included in every program

    let not_eq: bool = 18 != 18;

    let u: i32 = 64;
    let v: f64 = 3.14;
    let status: bool = true;

    println!("{}", not_eq);

    println!("Nice printing {:?}", (u, v, status, not_eq));

    let c: char = 'c';
    let ch: char = '\u{288A}';

    println!("{} {}", c, ch);
}
