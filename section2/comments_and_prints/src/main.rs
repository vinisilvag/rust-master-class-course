fn main() {
    // Comments

    // single-line comment
    // another single-line comment

    /*
     * multi-line
     * comment
     */

    // annotations for rustdoc (documentation inside the code)
    /**
     * doc
     */

    // ---------------------------

    // Print commands

    /*
     * ln at the end means that any subsequent print commands will appear at the next line
     *
     * ! is a symbol to denote a Rust macro (will appear on other commands)
     */
    println!("Hello, world!");

    print!("Text on");
    println!(" the same line");

    println!("{}", 10);

    println!("5 + {} = {}", 4, 9); // 5 + 4 = 9

    println!("My name is {} {}", "Vinicius", "Gomes");

    println!("\n\nNice text");

    println!("\tNice text too");

    print!(
        "Nice
    formatted
        text\n"
    );

    println!("\\n\nHellooo");

    println!("\nText \' some other text \" and so on");

    println!(
        "This {1} of {0} arguments is {2}",
        "positioned", "type", "cool"
    ); // zero based

    println!(
        "And {firstArg} some {secondArg} arguments, with is {thirdArg} too",
        firstArg = "now",
        secondArg = "named",
        thirdArg = "cool"
    );

    println!("Expressions on println!, {} + {} = {}", 5, 7, 5 + 7);
}
