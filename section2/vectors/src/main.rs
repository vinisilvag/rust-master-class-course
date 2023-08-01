fn main() {
    // vectors
    // similar to arrays but more flexible
    // collection of similar values (values of the same type) but the structure is resizable
    // created with the "Vec" macro

    let mut vec: Vec<i32> = vec![4, 5, 6, 7, 8];

    println!("{:?}", vec);

    println!("first val: {}", vec[0]);

    vec[2] = 10;

    println!("{:?}", vec);

    // vec of size 7 containing only zeros
    let vec_same_el: Vec<i32> = vec![0; 7];

    println!("{:?}", vec_same_el);

    let mut str_vec: Vec<&str> = vec!["apple", "banana", "mango"];
    let def_str_vec: Vec<&str> = vec!["unknown"; 3];

    println!("{:?}", str_vec);
    println!("{:?}", def_str_vec);

    str_vec[0] = "kiwi";

    println!("{:?}", str_vec);

    let char_vec: Vec<char> = vec!['a', 'b', 'c'];

    println!("{:?}", char_vec);

    // vec slices

    let subset_vec: &[i32] = &vec[0..3];

    println!("{:?}", subset_vec);

    let subset_containing_vec: &[i32] = &vec[0..=3];

    println!("{:?}", subset_containing_vec);

    // common vec functions

    let mut new_vec: Vec<i32> = vec![3, 4, 5, 6, 7];

    println!("Vec length: {}", new_vec.len());

    // verify if new_vec has a valid position at index 100
    // returns the element if is valid else returns None (Option is a enum type)
    let check_index: Option<&i32> = new_vec.get(100);

    println!("{:?}", check_index);

    // push values to the Vec

    new_vec.push(10);
    new_vec.push(20);

    println!("{:?}", new_vec);

    // remove values from Vec at specified index
    // removing at index 2 = 5

    new_vec.remove(2);

    println!("{:?}", new_vec);

    // check if Vec contains an element
    // the input is a reference to a number

    println!("new_vec contains 20? {}", new_vec.contains(&20));
}
