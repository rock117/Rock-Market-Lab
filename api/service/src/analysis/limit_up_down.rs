pub type Price = f64;

pub fn get_continues<F>(prices: &[Price], pred: F) -> usize where F: Fn(&Price, &Price) -> bool {
    if prices.is_empty() {
        return 0;
    }
    if prices.len() == 1 {
        return 1;
    }
    let mut curr_price = &prices[0];
    let mut count = 0usize;
    for price in &prices[1..] {
        if pred(price, curr_price) {
            count += 1;
        } else {
            break;
        }
        curr_price = price;
    }
    count
}


pub fn get_continues_limitup<F>(prices: &[Price]) -> usize {
    get_continues(prices, |price, curr_price| price > curr_price)
}

pub fn get_continues_limitdown<F>(prices: &[Price]) -> usize {
    get_continues(prices, |price, curr_price| price < curr_price)
}