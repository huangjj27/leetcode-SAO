// 给定两个二进制字符串，返回他们的和（用二进制表示）。

// 输入为非空字符串且只包含数字 1 和 0。

// 示例 1:

// 输入: a = "11", b = "1"
// 输出: "100"
// 示例 2:

// 输入: a = "1010", b = "1011"
// 输出: "10101"
// 

struct Solution{}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: &[u8] = a.as_bytes();
        let b: &[u8] = b.as_bytes();
        let a_length: usize = a.len();
        let mut a_point: usize = 0;
        let b_length: usize = b.len();
        let mut b_point: usize = 0;
        let mut a_result: u128 = 0;
        let mut b_result: u128 = 0;

        while a_point < a_length {
            a_result += ((a[a_point]) as u128 - 48).rotate_left((a_length - 1 - a_point) as u32);
            a_point += 1;
        }

        while b_point < b_length {
            b_result += ((b[b_point]) as u128 - 48).rotate_left((b_length - 1 - b_point) as u32);
            b_point += 1;
        }


        format!("{:b}", a_result + b_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(Solution::add_binary(String::from("11"), String::from("1")), String::from("100"));
        assert_eq!(Solution::add_binary(String::from("1010"), String::from("1011")), String::from("10101"));
        assert_eq!(Solution::add_binary(String::from("1110001"), String::from("110100100")), String::from("1000010101"));
        assert_eq!(Solution::add_binary(String::from("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101"), String::from("110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011")), String::from("110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000"));
    }
}