/// 1004. 最大连续1的个数 III
///
/// 给定一个由若干 0 和 1 组成的数组A，我们最多可以将K个值从 0 变成 1 。
///
/// 返回仅包含 1 的最长（连续）子数组的长度。
///
/// 示例 1：
///
/// 输入：A = [1,1,1,0,0,0,1,1,1,1,0], K = 2
/// 输出：6
/// 解释：
/// [1,1,1,0,0,1,1,1,1,1,1]
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 6。
/// 示例 2：
///
/// 输入：A = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], K = 3
/// 输出：10
/// 解释：
/// [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 10。
///
/// 提示：
///
/// 1 <= A.length <= 20000
/// 0 <= K <= A.length
/// A[i] 为0或1
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/width-consecutive-ones-iii
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut res = 0;
    let mut zero_num = 0;
    while j < a.len() {
        if a[j] == 0 { zero_num += 1; }
        while zero_num > k {
            if a[i] == 0 { zero_num -= 1; }
            i += 1;
        }
        res = res.max(j + 1 - i);
        j += 1;
    }
    res as i32
}

fn main() {
    let a = vec![1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1];
    let b = vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0];
    let c = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let d = vec![0, 0, 0, 0];
    let res = longest_ones(c, 3);
    println!("longest ones: {}", res);
}
