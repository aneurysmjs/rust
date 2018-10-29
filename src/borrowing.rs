pub fn print_message() {
  // s 'owns' the string
  let s = String::from("I'm a fucking string");
  // y 'borrows' from s
  let y = &s; 
  println!("{}", y);
}