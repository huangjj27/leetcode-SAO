// https://leetcode-cn.com/problems/daily-temperatures/
pub fn daily_temperatures(mut t: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec::<(usize, i32)> = Vec::with_capacity(t.len());

    for idx in 0..t.len() {
        while stack.last().is_some() && stack.last().unwrap().1 < t[idx] {
            let (i, _) = stack.pop().unwrap();
            t[i] = (idx - i) as i32;
        }

        stack.push((idx, t[idx]));
        t[idx] = 0;
    }

    t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expect = vec![1, 1, 4, 2, 1, 1, 0, 0];

        assert_eq!(daily_temperatures(temperatures), expect);
    }
}
