pub fn calculate_profit(prices: &Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;

    for &price in prices.iter().skip(1) {
        if price < buy {
            buy = price;
        } else if price - buy > profit {
            profit = price - buy;
        }
    }

    profit
}

#[cfg(test)]
#[path = "./best_time_to_buy_and_sell_stock_test.rs"]
mod best_time_to_buy_and_sell_stock_test;
