fn some_condition() -> bool {
    false
}

fn main() {
    let foo = 10;
    let mut x;

    {
        let y = 5;
        x = &y; // `y` is borrowed here

        println!("{}", x)
    }

    if some_condition() {
        x = &foo;
    }

    // x = &foo;

    println!("The value of x is: {}", x);
}
