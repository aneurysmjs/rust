// mod weapons;

// use weapons::primary::rotogun::make;

// use std::collections::HashMap;

// fn main() {
//   let mut hm = HashMap::new();

//   let rate = String::from("rate");
//   let clip = String::from("clip");

//   hm.insert(rate, 12);
//   hm.insert(clip, 100);
//   hm.remove(clip);

//   for (k, v) in &hm {
//     println!("{}: {}", k, v);
//   }
// }

use std::fs::File;

fn main() {
  let f = File::open("test.txt");

  let f = match f {
    Ok(file) => file,
    Err(err) => { panic!("there was a problem opening: {:?}", err) }
  };

  println!("file", f);
}