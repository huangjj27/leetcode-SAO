/// You are given an n x n 2D matrix representing an image. Rotate the image by 90 degrees (clockwise).
///
/// ## Note
/// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly.
/// DO NOT allocate another 2D matrix and do the rotation.
///
/// ## Examples
/// ### Example1
/// ```rust
/// # use leetcode_SAO::problems::problem48::rotate;
/// let mut input: Vec<Vec<i32>> = vec![
///   vec![1,2,3],
///   vec![4,5,6],
///   vec![7,8,9]
/// ];
///
/// let output: Vec<Vec<i32>> = vec![
///   vec![7,4,1],
///   vec![8,5,2],
///   vec![9,6,3]
/// ];
///
/// rotate(&mut input);
/// assert_eq!(input, output);
/// ```
///
/// ### Example2
/// ```rust
/// # use leetcode_SAO::problems::problem48::rotate;
/// let mut input: Vec<Vec<i32>> = vec![
///   vec![ 5, 1, 9,11],
///   vec![ 2, 4, 8,10],
///   vec![13, 3, 6, 7],
///   vec![15,14,12,16]
/// ];
///
/// let output: Vec<Vec<i32>> = vec![
///   vec![15,13, 2, 5],
///   vec![14, 3, 4, 1],
///   vec![12, 6, 8, 9],
///   vec![16, 7,10,11]
/// ];
///
/// rotate(&mut input);
/// assert_eq!(input, output);
/// ```
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut tmp;
    let len = matrix.len();
    let half = matrix.len() / 2;

    for lap in 0..half {
        let end = len - 1 - lap;
        println!("{} {}", lap, end);

        for idx in 0..(end-lap) {
            tmp = matrix[lap+idx][lap];
            matrix[lap+idx][lap] = matrix[end][lap+idx];
            matrix[end][lap+idx] = matrix[end - idx][end];
            matrix[end - idx][end] = matrix[lap][end - idx];
            matrix[lap][end - idx] = tmp;
        }
    }
}
