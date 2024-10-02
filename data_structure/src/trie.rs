#[derive(Default)]
struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    // 初始化 Trie 对象
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    // 向 Trie 中插入字符串
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
    }

    // 搜索字符串是否存在于 Trie 中
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }

    // 检查 Trie 中是否有以 prefix 为前缀的字符串
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
            } else {
                return false;
            }
        }
        true
    }
}
