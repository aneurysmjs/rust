pub fn factorial_recursive(n: u64) -> u64 {
  // Base case: If n is 0 or 1, the factorial is 1.
  if n == 0 || n == 1 {
    1
  } else {
    // Recursive case: Calculate factorial by calling the function recursively.
    n * factorial_recursive(n - 1)
  }
}

pub fn factorial_iterative(n: u64) -> u64 {
  let mut result = 1;

  for i in 1..n + 1 {
    result *= i;
  }

  result
}

#[cfg(test)]
#[path = "./factorial_test.rs"]
mod factorial_test;
