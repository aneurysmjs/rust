// src/lib.rs

mod arrays;
mod math;

// pub fn rotate(some_vec: &mut Vec<i32>, k: i32) -> i32 {
//   let mut total = 0;

//   for i in 0..k {
//     let last = some_vec.pop();
//     println!("last {:?}", last);
//     total += i;
//   }

//   total
// }

// pub fn max_profit(prices: &Vec<i32>) -> i32 {
//   // if prices.is_empty() {
//   //   0
//   // }

//   let mut buy = prices[0];
//   let mut profit = 0;

//   for &price in prices.iter().skip(1) {
//     if price < buy {
//       buy = price;
//     } else if price - buy > profit {
//       profit = price - buy;
//     }
//   }

//   profit
// }
