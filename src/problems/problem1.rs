/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

/// ## Examples:
/// ### Example 1
/// 给定 nums = [2, 7, 11, 15], target = 9
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]
/// ```rust
/// # use leetcode_SAO::problems::problem1::two_sum;
/// #
/// assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
/// ```


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();

    for (key, item) in nums.iter().enumerate() {
        if None == map.get(&(item)) {
            map.insert(target - item, key);
        } else {
            return vec![map[&(item)] as i32, key as i32];
        }
    }
    return vec![];
}


