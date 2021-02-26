/// 1178. 猜字谜
///
/// 外国友人仿照中国字谜设计了一个英文版猜字谜小游戏，请你来猜猜看吧。
/// 
/// 字谜的迷面 puzzle 按字符串形式给出，如果一个单词 word 符合下面两个条件，那么它就可以算作谜底：
/// 
/// 单词 word 中包含谜面 puzzle 的第一个字母。
/// 单词 word 中的每一个字母都可以在谜面 puzzle 中找到。
/// 例如，如果字谜的谜面是 "abcdefg"，那么可以作为谜底的单词有 "faced", "cabbage", 和 "baggage"；而 "beefed"（不含字母 "a"）以及 "based"（其中的 "s" 没有出现在谜面中）。
/// 返回一个答案数组 answer，数组中的每个元素 answer[i] 是在给出的单词列表 words 中可以作为字谜迷面 puzzles[i] 所对应的谜底的单词数目。
///
/// 示例：
/// 
/// 输入：
/// words = ["aaaa","asas","able","ability","actt","actor","access"],
/// puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
/// 输出：[1,1,3,2,4,0]
/// 解释：
/// 1 个单词可以作为 "aboveyz" 的谜底 : "aaaa"
/// 1 个单词可以作为 "abrodyz" 的谜底 : "aaaa"
/// 3 个单词可以作为 "abslute" 的谜底 : "aaaa", "asas", "able"
/// 2 个单词可以作为 "absoryz" 的谜底 : "aaaa", "asas"
/// 4 个单词可以作为 "actresz" 的谜底 : "aaaa", "asas", "actt", "access"
/// 没有单词可以作为 "gaswxyz" 的谜底，因为列表中的单词都不含字母 'g'。
/// 
/// 
/// 提示：
/// 
/// 1 <= words.length <= 10^5
/// 4 <= words[i].length <= 50
/// 1 <= puzzles.length <= 10^4
/// puzzles[i].length == 7
/// words[i][j], puzzles[i][j] 都是小写英文字母。
/// 每个 puzzles[i] 所包含的字符都不重复。

use std::collections::HashSet;
/// Time out.
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res = Vec::new();
    for puzzle in puzzles {
        let mut map = HashSet::with_capacity(7);
        let mut num = 0;
        let first = puzzle.as_bytes().to_vec()[0];
        for w in puzzle.as_bytes() {
            map.insert(w);
        }
        for word in words.clone() {
            if contain_first(&first, word.clone()) && valid_words(map.clone(), word) {
                num += 1;
            }
        }
        res.push(num);
    }
    res
}

pub fn contain_first(first: &u8, word: String) -> bool {
    for c in word.as_bytes() {
        if c == first { return true; }
    }
    false
}

pub fn valid_words(map: HashSet<&u8>, word: String) -> bool {
    for c in word.as_bytes() {
        if let None = map.get(&c) {
            return false;
        }
    }
    true
}

/// use bit operation
pub fn bit_find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res = Vec::with_capacity(puzzles.len());
    let mut bit_words = Vec::with_capacity(words.len());
    for word in words {
        let mut w = 0;
        for c in word.chars() { w |= 1 << (c as i32 - 'a' as i32); }
        bit_words.push(w);
    }
    for puzzle in puzzles {
        let mut w = 0;
        let mut num = 0;
        let first = 1 << (puzzle.as_bytes().to_vec()[0]  as i32 - 'a' as i32);
        for c in puzzle.chars() { w |= 1 << (c as i32 - 'a' as i32); }
        for bw in bit_words.clone() {
            if ((bw & w) == bw) && ((bw & first) > 0) { num += 1; }
        }
        res.push(num);
    }
    res
}

fn main() {
    let words = vec![String::from("aaaa"),String::from("asas"),String::from("able"),String::from("ability"),String::from("actt"),String::from("actor"),String::from("access")];
    let puzzles = vec![String::from("aboveyz"),String::from("abrodyz"),String::from("abslute"),String::from("absoryz"),String::from("actresz"),String::from("gaswxyz")];
    let res = bit_find_num_of_valid_words(words, puzzles);
    for i in res {
        print!("{} ", i);
    }
}
