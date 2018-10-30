mod geometry;

fn main() {
  let circle = geometry::circle::make(50, 35, 43.96);
  circle.show();
  println!("{:#?}", circle);
}