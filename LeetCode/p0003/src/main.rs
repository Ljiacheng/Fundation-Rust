/// 3. 无重复字符的最长子串
///
/// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 示例 1:
/// 
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
///
/// 示例 2:
/// 
/// 输入: s = "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
///
/// 示例 3:
/// 
/// 输入: s = "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
/// 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
///
/// 示例 4:
/// 
/// 输入: s = ""
/// 输出: 0
/// 
/// 
/// 提示：
/// 
/// 0 <= s.length <= 5 * 104
/// s 由英文字母、数字、符号和空格组成

use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res = 0;
    let mut map = HashMap::new();
    let mut left = 0;
    let s = s.as_bytes();
    for right in 0..s.len() {
        if let Some(&l) = map.get(&s[right]) {
            if l >= left { left = l + 1; }
        }
        map.insert(s[right], right);
        res = res.max(right + 1 - left);
    }
    res as i32
}

/// Better answer from
/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/solution/rustwu-zhong-fu-zi-fu-de-zui-chang-zi-chuan-by-xia/
pub fn s_length_of_longest_substring(s: String) -> i32 {
    if s.len()==0{
        return 0;
    }
    let mut ans:i32 = 0;
    let mut index:Vec<i32>=vec![-1;128];
    let mut i=0;
    for (j,c) in s.chars().enumerate(){
        i = index[c as usize].max(i);
        ans = ans.max((j as i32) -i+1);
        index[c as usize] = (j+1) as i32;
    }
    ans
}

fn main() {
    let s = String::from("pwwkew");
    println!("{}", s_length_of_longest_substring(s));
}
