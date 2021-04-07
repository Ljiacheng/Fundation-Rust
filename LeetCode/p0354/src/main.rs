/// 354. 俄罗斯套娃信封问题
///
/// 给你一个二维整数数组 envelopes ，其中 envelopes[i] = [wi, hi] ，表示第 i 个信封的宽度和高度。
/// 当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。
/// 请计算 最多能有多少个 信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。
/// 注意：不允许旋转信封。
///
/// 示例 1：
/// 
/// 输入：envelopes = [[5,4],[6,4],[6,7],[2,3]]
/// 输出：3
/// 解释：最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。
/// 示例 2：
/// 
/// 输入：envelopes = [[1,1],[1,1],[1,1]]
/// 输出：1
/// 
/// 
/// 提示：
/// 
/// 1 <= envelopes.length <= 5000
/// envelopes[i].length == 2
/// 1 <= wi, hi <= 104
pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));
    let mut dp = vec![envelopes[0][1]];
    for i in 1..envelopes.len() {
        let mut j = 0;
        while j < dp.len() {
            if dp[j] >= envelopes[i][1] {
                dp[j] = envelopes[i][1];
                break;
            }
            j += 1;
        }
        if j == dp.len() {
            dp.push(envelopes[i][1]);
        }
    }
    dp.len() as i32
}

fn main() {
    let v = vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]];
    let v1 = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
    println!("{}", max_envelopes(v));
}
