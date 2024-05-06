pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut max_profit = 0;
  let mut buy = prices[0];
  for i in 1..prices.len() {
    if prices[i] < buy {
      buy = prices[i];
    } else {
      let profit = prices[i] - buy;
      if profit > max_profit {
        max_profit = profit;
      }
    }
  }
  max_profit
}

#[cfg(test)]
mod test {
  use std::{env, fs::read_to_string, path::Path};

  #[test]
  fn max_profit_example_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(super::max_profit(prices), 5);
  }

  #[test]
  fn max_profit_example_2() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(super::max_profit(prices), 0);
  }

  #[test]
  fn max_profit_example_3() {
    // println!("Current dir: {:?}", env::current_dir().unwrap());
    let result = read_to_string("src/day_10/sample.txt");
    match result {
      Ok(content) => {
        let prices: Vec<i32> = content.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let max = super::max_profit(prices);
        // println!("Max profit: {}", max);
        assert_eq!(max, 999);
      }
      Err(err) => {
        println!("Error reading file: {}", err);
      }
    };
  }
}
