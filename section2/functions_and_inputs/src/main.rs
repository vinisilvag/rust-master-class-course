// functions and user inputs

fn main() {
    basic_fn();
    function_with_inputs("Vinicius", 1_000);

    let full_name: &str = "Vinicius Gomes";
    let salary: i32 = 1;

    function_with_inputs(full_name, salary);

    println!("{}", function_with_inputs_and_outputs(2, 3));

    println!("{:?}", function_with_inputs_and_multiple_outputs(2, 3));

    // let result: (i32, i32, i32) = function_with_inputs_and_multiple_outputs(2, 3);
    // result.0
    // result.1
    // result.2
    let (a, b, ans): (i32, i32, i32) = function_with_inputs_and_multiple_outputs(2, 3);

    println!("{} * {} = {}", a, b, ans);

    let full_name: String = {
        let first_name: &str = "Vinicius";
        let last_name: &str = "Gomes";

        // a return statement
        // returns a String, not &str
        format!("{} {}", first_name, last_name)
    };

    println!("{}", full_name);

    // ------------------------------------

    // taking inputs from keyboard

    let mut age: String = String::new();

    std::io::stdin()
        .read_line(&mut age)
        .expect("Failed to read age.");

    let age: i32 = age.trim().parse().expect("Invalid age input.");

    println!("My age is {:?}", age);
}

fn basic_fn() {
    println!("Basic fn call");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("{} {}", name, salary);
}

fn function_with_inputs_and_outputs(a: i32, b: i32) -> i32 {
    // return a * b;
    a * b
}

fn function_with_inputs_and_multiple_outputs(a: i32, b: i32) -> (i32, i32, i32) {
    // return a * b;
    (a, b, a * b)
}
