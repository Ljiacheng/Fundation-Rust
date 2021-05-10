/// 208. 实现 Trie (前缀树)
///
/// Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补完和拼写检查。
/// 
/// 请你实现 Trie 类：
/// 
/// Trie() 初始化前缀树对象。
/// void insert(String word) 向前缀树中插入字符串 word 。
/// boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回 false 。
/// boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true ；否则，返回 false 。
///
/// 示例：
/// 
/// 输入
/// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
/// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
/// 输出
/// [null, null, true, false, true, null, true]
/// 
/// 解释
/// Trie trie = new Trie();
/// trie.insert("apple");
/// trie.search("apple");   /// 返回 True
/// trie.search("app");     /// 返回 False
/// trie.startsWith("app"); /// 返回 True
/// trie.insert("app");
/// trie.search("app");     /// 返回 True
///
/// 提示：
/// 
/// 1 <= word.length, prefix.length <= 2000
/// word 和 prefix 仅由小写英文字母组成
/// insert、search 和 startsWith 调用次数 总计 不超过 3 * 104 次

#[derive(Default)]
struct Trie {
    is_end: bool,
    child: [Option<Box<Trie>>; 26]
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node = self;
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            if node.child[index].is_none() {
                node.child[index] = Some(Box::new(Trie::new()));
            }
            node = node.child[index].as_deref_mut().unwrap();
        }
        node.is_end = true;
    }

    fn search_prefix(&self, prefix: String) -> Option<bool> {
        let mut node = self;
        for byte in prefix.bytes() {
            let index = (byte - b'a') as usize;
            if node.child[index].is_none() {
                return None;
            }
            node = &node.child[index].as_ref().unwrap();
        }
        Some(node.is_end)
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.search_prefix(word).unwrap_or(false)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.search_prefix(prefix).is_some()
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("search1: {}", trie.search("apple".to_string()));
    println!("search2: {}", trie.search("app".to_string()));
    println!("start1: {}", trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    println!("search3: {}", trie.search("app".to_string()));
}
