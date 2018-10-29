fn main() {
  // define tuple
  // let t: (i32, f64, char) = (89, 2.14, 'j');
  // define array of signed 32bit integer with a length of 5
  // let xs: [i32; 5] = [1, 2, 3, 4, 5];
 
  // slice array starting from the second index up to the fourth index
  // '&' stands for reference
  // let ys = &xs[2..4];

  let my_string = String::from("hablamelo");

  println!("{:?}", &my_string[2..2]);
}