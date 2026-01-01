use std::env;

#[path = "fp/mod.rs"]
mod fp;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Аргументы: {:?}", args);

    if args.len() < 2 {
        println!("Использование: cargo run <example_name>");
        return;
    }

    match args[1].as_str() {
        "mutable_borrow_for_scoped_mutation" => fp::mutable_borrow_for_scoped_mutation(),

        _ => println!("Неизвестный пример: {}", args[1]),
    }
}
