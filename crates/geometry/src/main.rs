mod figures;

use figures::circle::make;

fn main() {
    let circle = make(50, 35, 43.96);

    circle.show();

    let hotkeys: Vec<&str> = "ctrl+shift+a, ctrl+alt+backspace".split(",").collect();

    println!("{:?}", hotkeys);
}
