const PI: f32 = 3.14159265358979323846264338327950288f32;

#[derive(Debug)]
pub struct Circle {
  x: i32,
  y: i32,
  radius: f32,
}

impl Circle {
  pub fn area(&self) -> f32 {
    PI * (self.radius * self.radius)
  }
  pub fn show(&self) {
    println!("Area is: {}", self.area());
  }
}

impl Circle {
  fn new(x: i32, y: i32, radius: f32) -> Circle {
    Circle {
      x,
      y,
      radius
    }
  }
}

pub fn make(x: i32, y:i32, radius: f32) -> Circle {
  Circle::new(x, y, radius)
}
