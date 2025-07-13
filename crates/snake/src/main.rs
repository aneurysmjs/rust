#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Right(Point),
    Bottom(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    RightKey(String),
    BottomKey(String),
    LeftKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("pressed w")),
            Direction::Right(_) => Keys::RightKey(String::from("pressed d")),
            Direction::Bottom(_) => Keys::BottomKey(String::from("pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("pressed a")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::RightKey(ref s) => s,
            Keys::BottomKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
        }
    }
}

fn main() {
    let u = Direction::Up(Point { x: 2, y: 5 });
    let k = u.match_direction();
    let s = k.destruct();

    println!("{:?}", k);
    println!("{}", s)
}
