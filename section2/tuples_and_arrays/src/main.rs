// compound data types
// tuples and arrays

fn main() {
    // tuples
    // fixed size and not require same data type elements

    let info: (&str, i32) = ("Salary", 40_000);

    println!("Info: {} - Content: {}", info.0, info.1);
    println!("Another way: {:?}", info);

    // destructuring
    let (key, value) = info;

    println!("{} {}", key, value);

    let key_2: &str = info.0;
    let value_2: i32 = info.1;

    // tuples of tuples
    let nested_tuple: (i32, f64, (u32, &str), &str) = (4, 5.5, (0, "foo"), "bar");

    println!("{:?}", nested_tuple);

    let index: u32 = nested_tuple.2 .0;
    let val: &str = nested_tuple.2 .1;

    println!("{} {}", index, val);

    // empty tuple
    let empty: () = ();

    // ----------------------------------------------

    // arrays
    // same data type

    // [data type; array size]
    let mut arr: [i32; 5] = [4, 5, 6, 7, 8];

    // this form of printing will be very useful in general for representing compound data types
    println!("{:?}", arr);

    println!(
        "0 = {}, 1 = {}, 2 = {}, 3 = {} and 4 = {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );

    arr[4] = 10;

    println!("{}", arr[4]);

    // array with 5 elements, all of then equal to 1
    let same_arr: [i32; 5] = [1; 5];

    println!("{:?}", same_arr);

    let string_arr: [&str; 3] = ["apple", "banana", "mango"];
    let new_string_arr: [&str; 3] = ["unknown"; 3];

    let char_arr: [char; 3] = ['a', 'b', 'c'];

    println!("{:?}", string_arr);
    println!("{:?}", new_string_arr);
    println!("{:?}", char_arr);

    // array slices
    let mut new_arr: [i32; 6] = [4, 5, 6, 7, 8, 9];

    let slice: &[i32] = &new_arr[0..3];
    println!("{:?}", slice);

    // ..=3 keep the element at index 3 too
    let containing_slice: &[i32] = &new_arr[0..=3];
    println!("{:?}", containing_slice);

    // compilation error because it's a reference (& indicates that)
    // slice[1] = 0;

    println!("Array length: {}", new_arr.len());

    // 24 = (4 bytes for i32) * (6 elements)
    println!("Number of bytes: {}", std::mem::size_of_val(&new_arr));

    // compilation error
    // new_arr[6] = 10;

    // verify if new_arr has a valid position at index 100
    // returns the element if is valid else returns None (Option is a enum type)
    let check_index: Option<&i32> = new_arr.get(100);

    println!("{:?}", check_index);
}
