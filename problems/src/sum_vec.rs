pub fn sum_vector(some_vec: &Vec<i32>) -> i32 {
  let mut total = 0;

  for i in some_vec {
    total += i;
  }

  total
}

#[cfg(test)]
#[path = "./sum_vec_test.rs"]
mod sum_vec_test;
