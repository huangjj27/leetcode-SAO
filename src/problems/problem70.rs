// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

// 注意：给定 n 是一个正整数。

// 示例 1：

// 输入： 2
// 输出： 2
// 解释： 有两种方法可以爬到楼顶。
// 1.  1 阶 + 1 阶
// 2.  2 阶
// 示例 2：

// 输入： 3
// 输出： 3
// 解释： 有三种方法可以爬到楼顶。
// 1.  1 阶 + 1 阶 + 1 阶
// 2.  1 阶 + 2 阶
// 3.  2 阶 + 1 阶

struct Solution{}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let result: i32 = n - 3;
        if result > 0 {
            ((1f64 / 5f64.sqrt()) * (((1f64 + 5f64.sqrt()) / 2f64).powi(n + 1) - ((1f64 - 5f64.sqrt()) / 2f64).powi(n + 1))).round() as i32
        } else {
            n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(6), 13);
        assert_eq!(Solution::climb_stairs(35), 14930352);
    }
}