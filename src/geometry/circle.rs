// https://stackoverflow.com/questions/36476775/why-is-the-std-module-undeclared
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
  x: i32,
  y: i32,
  radius: f64,
}

impl Circle {
  pub fn area(&self) -> f64 {
    PI * (f64::powi(self.radius, 2))
  }
  pub fn show(&self) {
    println!("Area is: {}", self.area());
  }
}

impl Circle {
  fn new(x: i32, y: i32, radius: f64) -> Circle {
    Circle {
      x,
      y,
      radius
    }
  }
}

pub fn make(x: i32, y:i32, radius: f64) -> Circle {
  Circle::new(x, y, radius)
}
