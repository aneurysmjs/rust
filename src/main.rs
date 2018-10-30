mod methods;

fn main() {
  let circle = methods::circle::make(50, 35, 43.96);
  circle.show();
  println!("{:#?}", circle);
}