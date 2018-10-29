mod methods;

fn main() {
  let rectangle = methods::rectangle::make(50, 35);
  let area = rectangle.area();
  println!("{:?}", area);
}