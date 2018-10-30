#[derive(Debug)]
pub struct Rectangle {
  height: u32,
  width: u32,
}

 // methods
impl Rectangle {

  pub fn area(&self) -> u32 {
    self.height * self.width
  }

  pub fn show(&self) {
    println!("Area of {}X{}: {}", self.height, self.width, self.area());
  }
 
}

 // related functions
impl Rectangle {
  fn new(height: u32, width: u32) -> Rectangle {
    Rectangle {
      height,
      width
    }
  }
}


pub fn make(height: u32, width:u32) -> Rectangle {
  Rectangle::new(height, width)
}
