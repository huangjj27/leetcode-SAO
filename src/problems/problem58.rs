// 给定一个仅包含大小写字母和空格 ' ' 的字符串，返回其最后一个单词的长度。

// 如果不存在最后一个单词，请返回 0 。

// 说明：一个单词是指由字母组成，但不包含任何空格的字符串。

// 示例:

// 输入: "Hello World"
// 输出: 5
// 

struct Solution{}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let list: Vec<&str> = s.split_whitespace().collect();
        let length: usize = list.len();

        if length == 0 {
            0
        } else {
            list[length - 1].len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word(String::from("Hello World")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("Hello ")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("")), 0);
    }
}