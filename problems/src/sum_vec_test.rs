use super::{ sum_vector };

#[cfg(test)]
#[test]
fn it_works() {
  let my_vec = vec![1, 2, 3];
  let result = sum_vector(&my_vec);

  assert_eq!(result, 6);
}
