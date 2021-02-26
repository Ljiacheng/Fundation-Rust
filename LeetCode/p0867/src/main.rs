/// 给你一个二维整数数组 matrix， 返回 matrix 的 转置矩阵 。
/// 
/// 矩阵的 转置 是指将矩阵的主对角线翻转，交换矩阵的行索引与列索引。
///
/// 
/// 示例 1：
/// 
/// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
/// 输出：[[1,4,7],[2,5,8],[3,6,9]]
/// 示例 2：
/// 
/// 输入：matrix = [[1,2,3],[4,5,6]]
/// 输出：[[1,4],[2,5],[3,6]]
///
/// m == matrix.length
/// n == matrix[i].length
/// 1 <= m, n <= 1000
/// 1 <= m * n <= 105
/// -109 <= matrix[i][j] <= 109
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/transpose-matrix
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![ vec![0;matrix.len()]; matrix[0].len()];
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            res[i][j] = matrix[j][i];
        }
    }
    res
}

fn main() {
    let a = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let b = vec![vec![1,2,3], vec![4,5,6]];
    let res = transpose(a);
    for v in res {
        for i in v {
            print!("{} ", i);
        }
        println!("\n");
    }
}
