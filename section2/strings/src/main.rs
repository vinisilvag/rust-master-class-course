fn main() {
    // strings
    // fixed length -> &str (string slice, immutable, is a reference)
    // dynamic length -> String

    let name: &str = "Vinicius Gomes";

    println!("{}", name);

    let mut grow: String = String::from("Nice string");

    println!("String content: {}", grow);

    // valid operations with type String

    let mut curr: String = String::from("Some string");

    println!("{}", curr);

    curr.push('a');

    println!("{}", curr);

    curr.pop();

    println!("{}", curr);

    curr.push_str(" string pushed");

    println!("{}", curr);

    println!(
        "Basic string operations,
        is_empty(): {},
        length(): {},
        Bytes() (bytes allocated in memory): {},
        contains 'string': {}",
        curr.is_empty(),
        curr.len(),
        curr.capacity(),
        curr.contains("string")
    );

    curr.push_str("     ");

    let str_len: usize = curr.trim().len();

    println!("{}", str_len);

    // convert other data types to string

    let num: i32 = 30;
    let num_str: String = num.to_string();

    let c: char = 'a';
    let c_str: String = c.to_string();

    // &str to String with "to_string()"
    let my_name: String = "Vinicius Gomes".to_string();

    // empty string
    let empty: String = String::new();

    println!("is_empty(): {}", empty.is_empty());
    println!("size: {}", empty.len());

    let first_name: String = String::from("Vinicius");
    let last_name: String = "Gomes".to_string();
    let full_name: String = format!("My full name is {} {}", first_name, last_name);

    println!("{}", full_name);

    let concat: String = format!("{}{}", first_name, last_name);

    println!("{}", concat);
}
