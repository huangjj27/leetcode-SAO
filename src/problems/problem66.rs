// 给定一个由整数组成的非空数组所表示的非负整数，在该数的基础上加一。

// 最高位数字存放在数组的首位， 数组中每个元素只存储一个数字。

// 你可以假设除了整数 0 之外，这个整数不会以零开头。

// 示例 1:

// 输入: [1,2,3]
// 输出: [1,2,4]
// 解释: 输入数组表示数字 123。
// 示例 2:

// 输入: [4,3,2,1]
// 输出: [4,3,2,2]
// 解释: 输入数组表示数字 4321。
// 

struct Solution{}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let length: usize = digits.len();
        let mut result: Vec<i32> = Vec::new();
        let mut tmp: i32 = 1;
        let mut j: usize = length - 1;

        while j >= 0 {
            tmp += digits[j];
            result.insert(0, tmp % 10);
            tmp /= 10;

            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }

        while tmp > 0 {
            result.insert(0, tmp % 10);
            tmp /= 10;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![9,9,9]), vec![1,0,0,0]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
    }
}