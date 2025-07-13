fn main() {
    let x;

    {
        let y = 5;
        x = &y; // `y` is borrowed here
    }

    println!("The value of x is: {}", x); // x is invalid because y is already drop in the above block
}
