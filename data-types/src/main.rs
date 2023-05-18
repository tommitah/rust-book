use std::io;

fn main() {
    // integer overflow
    // let mut x: u8 = 256;

    // array typing
    let months = [
        "Jan", "Feb", "March", "April", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    for month in months {
        println!("{month}");
    }

    // array types can be either inferred or, like in this case, specified the type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in a {
        print!("{i}\t");
    }

    // also you can just fill an array with the same value
    let b = [3; 5];
    for i in b {
        print!("{i}\t");
    }
    println!("");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}.");
}
