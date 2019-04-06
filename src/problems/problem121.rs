/// # 121. Best Time to Buy and Sell Stock
///
/// Say you have an array for which the ith element is the price of
/// a given stock on day i. If you were only permitted to complete at most
/// one transaction (i.e., buy one and sell one share of the stock), design
/// an algorithm to find the maximum profit. Note that you cannot sell a stock
/// before you buy one.
///
/// ## Examples
/// ### Example 1
/// Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
/// Not 7-1 = 6, as selling price needs to be larger than buying price.
/// ```
/// # use leetcode_SAO::problems::problem121::max_profit;
/// #
/// let prices = vec![7,1,5,3,6,4];
/// assert_eq!(max_profit(prices), 5);
/// ```
///
/// ### Example 2
/// In this case, no transaction is done, i.e. max profit = 0.
///
/// ```
/// # use leetcode_SAO::problems::problem121::max_profit;
/// #
/// let prices = vec![7,6,4,3,1];
/// assert_eq!(max_profit(prices), 0);
/// ```
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut max = 0;

    for i in 0..len {
        let mut iter_max = 0;

        for j in i + 1..len {
            if prices[j] - prices[i] > iter_max {
                iter_max = prices[j] - prices[i];
            }
        }

        if iter_max > max {
            max = iter_max;
        }
    }

    max
}
