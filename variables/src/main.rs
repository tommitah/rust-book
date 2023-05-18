fn main() {
    // i32 is a signed integer ([0, 4294967295])
    // u32 is an unsigned integer ([-2147483648, 2147483648])
    let x = 5;

    // Shadowing
    // in the inner scope the original value of x is not changed
    // the second variable named x 'overshadows' the first, taking use of the
    // variable name to itself until it gets shadowed or the inner scope ends.
    // using 'let x' again just really creates a new variable rather than changes the original.
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // 0x2a30 is 10800 in hex
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
}
