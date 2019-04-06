/// # 1018. Binary Prefix Divisible By 5
/// 
/// Given an array A of 0s and 1s, consider N_i: the i-th subarray from A[0]
/// to A[i] interpreted as a binary number (from most-significant-bit to 
/// least-significant-bit.) Return a list of booleans answer, where answer[i]
/// is true if and only if N_i is divisible by 5.
/// 
/// ## Examples
/// ### example1
/// The input numbers in binary are 0, 01, 011; which are 0, 1, and 3 in 
/// base-10.  Only the first number is divisible by 5, so answer[0] is true.
/// 
/// ```
/// # use leetcode_SAO::contests::contest130::prefixes_div_by5;
/// #
/// let a = vec![0, 1, 1];
/// assert_eq!(prefixes_div_by5(a), [true, false, false]);
/// ```
/// 
/// ### example2
/// ```
/// # use leetcode_SAO::contests::contest130::prefixes_div_by5;
/// #
/// let a = vec![1, 1, 1];
/// assert_eq!(prefixes_div_by5(a), [false, false, false]);
/// ```
/// 
/// ### example3
/// ```
/// # use leetcode_SAO::contests::contest130::prefixes_div_by5;
/// #
/// let a = vec![0,1,1,1,1,1];
/// assert_eq!(prefixes_div_by5(a), [true,false,false,false,true,false]);
/// ```
/// 
/// ### example4
/// ```
/// # use leetcode_SAO::contests::contest130::prefixes_div_by5;
/// #
/// let a = vec![1,1,1,0,1];
/// assert_eq!(prefixes_div_by5(a), [false,false,false,false,false]);
/// ```
/// 
/// ## Input Range
/// 1 <= A.length <= 30000
/// A[i] is 0 or 1
pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut result = Vec::with_capacity(a.len());
    
    a.iter()
        .fold(0i32, |N_i, &a_i| {
            // use Congruence properties to prevent numberic overflow
            let N_i = (N_i * 2 + a_i) % 5;
            result.push(N_i == 0);
            
            N_i
        });
    
    result
}