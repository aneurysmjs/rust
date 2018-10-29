pub struct Rectangle {
  height: u32,
  width: u32,
}

impl Rectangle {
  // method
  pub fn area(&self) -> u32 {
    self.height * self.width
  }
  // related function
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
