/// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
/// ## Examples
/// ### Example1:
/// 输入: 123
/// 输出: 321
/// ```rust
/// # use leetcode_SAO::problems::problem7::reverse;
/// assert_eq!(reverse(123), 321);
/// ```
/// 
/// ### Example2:
/// 输入: -123
/// 输出: -321
/// ```rust
/// # use leetcode_SAO::problems::problem7::reverse;
/// assert_eq!(reverse(-123), -321);
/// ```
/// 
/// ### Example3:
/// 输入: 120
/// 输出: 21
/// ```rust
/// # use leetcode_SAO::problems::problem7::reverse;
/// assert_eq!(reverse(120), 21);
/// ```

pub fn reverse(x: i32) -> i32 {
    let mut x: i32 = x;
    let mut result: i32 = 0;
    if x > 0 {
        while x > 0 {
            let i: i32 = x % 10;
            if result > std::i32::MAX / 10 || (result == std::i32::MAX && i > 7) {
                return 0;
            } else {
                result = result * 10 + i;
                x = x / 10;
            }
        }

        result
    } else {
        while x < 0 {
            let i: i32 = x % 10;
            if result < std::i32::MIN / 10 || ((result * -1) == std::i32::MIN && i < -8) {
                return 0;
            } else {
                result = result * 10 + i;
                x = x / 10;
            }
        }

        result
    }
}

