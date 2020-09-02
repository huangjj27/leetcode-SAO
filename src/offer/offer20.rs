//! 请实现一个函数用来判断字符串是否表示数值（包括整数和小数）。
//! 例如，字符串"+100"、"5e2"、"-123"、"3.1416"、"-1E-16"、"0123"都表示数值，
//! 但"12e"、"1a3.14"、"1.2.3"、"+-5"及"12e+5.4"都不是。

pub fn is_number(s: String) -> bool {
    let s = s.trim();
    return s.parse::<i32>().is_ok() || s.parse::<f32>().is_ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integers() {
        assert!(is_number("100".to_string()));
        assert!(is_number("123".to_string()));
        assert!(is_number("0123".to_string()));
        assert!(is_number("314".to_string()));
        assert!(is_number("0".to_string()));
        assert!(is_number("000".to_string()));
    }

    #[test]
    fn test_fix_point_numbers() {
        assert!(is_number("3.14".to_string()));
        assert!(!is_number("3.1.4".to_string()));
    }

    #[test]
    fn test_signs() {
        assert!(is_number("+100".to_string()));
        assert!(is_number("-123".to_string()));
        assert!(!is_number("+-5".to_string()));
    }

    #[test]
    fn test_exponent() {
        assert!(is_number("1e0".to_string()));
        assert!(is_number("1e1".to_string()));
        assert!(is_number("1e-1".to_string()));
        assert!(is_number("5e2".to_string()));
        assert!(is_number("31.4e-1".to_string()));
        assert!(!is_number("31.4e".to_string()));
        assert!(!is_number("12e+5.4".to_string()));
    }

    #[test]
    fn test_with_spaces() {
        assert!(is_number("1 ".to_string()));
        assert!(is_number(" 1".to_string()));
    }
}
