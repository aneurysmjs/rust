use crate::arrays::best_time_to_buy_and_sell_stock::calculate_profit;

#[cfg(test)]
#[test]
fn it_should_work() {
  let mut my_vec = vec![7, 1, 5, 3, 6, 4];

  let result = calculate_profit(&mut my_vec);

  assert_eq!(result, 5);
}
