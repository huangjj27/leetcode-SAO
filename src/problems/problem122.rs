/// # 122. Best Time to Buy and Sell Stock II
/// Say you have an array for which the ith element is the price of a given 
/// stock on day i. Design an algorithm to find the maximum profit. You may
/// complete as many transactions as you like (i.e., buy one and sell one
/// share of the stock multiple times).
/// 
/// Note: You may not engage in multiple transactions at the same time (i.e.,
/// you must sell the stock before you buy again).
/// 
/// ## Examples
/// ### Example 1
/// Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
/// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
/// 
/// ```
/// # use leetcode_SAO::problems::problem122::max_profit;
/// # 
/// let prices = vec![7,1,5,3,6,4];
/// assert_eq!(max_profit(prices), 7);
/// ```
/// 
/// ### Example 2
/// Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
/// Note that you cannot buy on day 1, buy on day 2 and sell them later,
/// as you are engaging multiple transactions at the same time. You must sell
/// before buying again.
/// 
/// ```
/// # use leetcode_SAO::problems::problem122::max_profit;
/// # 
/// let prices = vec![1,2,3,4,5];
/// assert_eq!(max_profit(prices), 4);
/// ```
/// 
/// ### Example 3
/// In this case, no transaction is done, i.e. max profit = 0.
/// 
/// ```
/// # use leetcode_SAO::problems::problem122::max_profit;
/// # 
/// let prices = vec![7,6,4,3,1];
/// assert_eq!(max_profit(prices), 0);
/// ```
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2)
        .filter(|win| win[0] < win[1])
        .map(|win| win[1] - win[0])
        .sum::<i32>()
}