/// 224. 基本计算器
/// 实现一个基本的计算器来计算一个简单的字符串表达式 s 的值。
///
/// 示例 1：
/// 
/// 输入：s = "1 + 1"
/// 输出：2
/// 示例 2：
/// 
/// 输入：s = " 2-1 + 2 "
/// 输出：3
/// 示例 3：
/// 
/// 输入：s = "(1+(4+5+2)-3)+(6+8)"
/// 输出：23
/// 
/// 
/// 提示：
/// 
/// 1 <= s.length <= 3 * 105
/// s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
/// s 表示一个有效的表达式

pub fn calculate(s: String) -> i32 {
    let s = s.as_bytes();

    let mut res: i32 = 0;
    let mut signs = vec![];
    let mut num: i32 = 0;
    let mut ope = 0;

    for i in 0..s.len() {
        let mut flag: u8 = if signs.len() > 0 {
            signs[signs.len() - 1]
        } else {
            0
        };

        match s[i] {
            b'(' => {
                flag = flag ^ ope;
                signs.push(flag);
                ope = 0;
            }
            b')' => {
                signs.pop();
            }
            b'+' => ope = 0,
            b'-' => ope = 1,
            b' ' => {}
            n => {
                num = num * 10 + (n - b'0') as i32;
                if i < s.len() - 1 && s[i + 1] >= b'0' && s[i + 1] <= b'9' {
                    continue;
                }
                if ope ^ flag == 0 {
                    res += num as i32;
                } else {
                    res -= num as i32;
                }
                num = 0;
            }
        }
    }
    res
}

fn main() {
    let a = "1 + 1".to_string();
    let b = " 2-1 + 2 ".to_string();
    let c = "(1+(4+5+2)-3)+(6+8)".to_string();
    println!("{}",calculate(c));
}
