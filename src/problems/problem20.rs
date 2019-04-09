// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。

// 有效字符串需满足：

// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
// 注意空字符串可被认为是有效字符串。

// 示例 1:

// 输入: "()"
// 输出: true
// 示例 2:

// 输入: "()[]{}"
// 输出: true
// 示例 3:

// 输入: "(]"
// 输出: false
// 示例 4:

// 输入: "([)]"
// 输出: false
// 示例 5:

// 输入: "{[]}"
// 输出: true

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::str::Chars;

        let length: usize = s.len();

        if length == 0 {
            return true;
        }

        if length % 2 > 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        let tmp: Chars = s.chars();

        for item in tmp {
            if item == '(' || item == '{' || item == '[' {
                stack.push(item);
            } else if item == ']' {
                match stack.pop() {
                    Some(i) if i == '[' => {}
                    _ => {
                        return false;
                    }
                }
            } else if item == '}' {
                match stack.pop() {
                    Some(i) if i == '{' => {}
                    _ => {
                        return false;
                    }
                }
            } else if item == ')' {
                match stack.pop() {
                    Some(i) if i == '(' => {}
                    _ => {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        if stack.len() > 0 {
            return false;
        }

        true
    }
}

#[test]
fn test_is_valid() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    assert_eq!(Solution::is_valid(String::from("[")), false);
    assert_eq!(Solution::is_valid(String::from("((")), false);
}
