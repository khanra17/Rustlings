pub fn calculate_price(order_amount: u32) -> u32 {
    if order_amount < 40 { order_amount * 2 } else { order_amount }
}
