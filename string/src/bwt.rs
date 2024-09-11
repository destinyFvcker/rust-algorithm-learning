//! 这个算法是 Burrows-Wheeler Transform (BWT)，它是一种文本变换算法，
//! 通常用于数据压缩。具体来说，BWT 通过重排字符串的字符，使得相同的字符更容易出现在一起，
//! 从而更容易使用其他压缩算法（如游程编码）来压缩。

/// 这个函数实现了将输入字符串进行 Burrows-Wheeler 变换。基本步骤如下：
///
/// - Step 1: 生成输入字符串的所有循环移位（通过对字符串的每一位进行旋转）。
/// - Step 2: 将这些移位结果按字典序排序。
/// - Step 3: 创建编码后的字符串：从排序后的结果中取出每个字符串的最后一个字符，并记录原始字符串在排序后的位置（index）。
pub fn burrows_wheeler_transform(input: String) -> (String, usize) {
    let len = input.len();

    let mut table = Vec::<String>::with_capacity(len);
    for i in 0..len {
        table.push(input[i..].to_owned() + &input[..i]);
    }
    table.sort_by_key(|a| a.to_lowercase());

    let mut encoded = String::new();
    let mut index: usize = 0;
    for (i, item) in table.iter().enumerate() {
        encoded.push(item.chars().last().unwrap());
        if item.eq(&input) {
            index = i;
        }
    }

    (encoded, index)
}

/// 这个函数实现了 Burrows-Wheeler 变换的逆变换。它通过以下步骤恢复原始字符串：
/// - Step 1: 对编码后的字符串（BWT 结果）中的字符按字母顺序排列。
/// - Step 2: 使用原始位置信息（index）和编码后的字符逐步恢复原始字符串。
pub fn inv_burrows_wheeler_transform(input: (String, usize)) -> String {
    let len = input.0.len();
    let mut table = Vec::<(usize, char)>::with_capacity(len);
    for i in 0..len {
        table.push((i, input.0.chars().nth(i).unwrap()));
    }

    table.sort_by(|a, b| a.1.cmp(&b.1));

    let mut decoded = String::new();
    let mut idx = input.1;
    for _ in 0..len {
        decoded.push(table[idx].1);
        idx = table[idx].0;
    }

    decoded
}
