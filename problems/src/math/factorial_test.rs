use crate::math::factorial::{ factorial_recursive, factorial_iterative };

#[cfg(test)]
#[test]
fn it_test_factorial_recursively() {
  let result = factorial_recursive(5);

  assert_eq!(result, 120);
}

#[cfg(test)]
#[test]
fn it_test_factorial_iteratively() {
  let result = factorial_iterative(5);

  assert_eq!(result, 120);
}
