mod sum_vec;

use sum_vec::sum_vector;

fn main() {
  let my_vec = vec![1, 2, 3];
  let result = sum_vector(&my_vec);

  println!("result {}", result);
}
