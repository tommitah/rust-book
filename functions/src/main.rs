fn main() {
    another_function();
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let x = five();
    println!("This is x (five()): {x}.");
    let x_plus_one = plus_one(x);
    println!("This is x + 1: {x_plus_one}");

    // let if
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("{num}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop return value: {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            // in rust it's possible to break out of nested loops like this
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn another_function() {
    println!("Another function!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// implicit return
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
