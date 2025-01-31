use std::collections::HashMap;

pub fn is_palindrome_permutation(s: &str) -> bool {
    let mut char_count = HashMap::new();

    for c in s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
    {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let odd_count = char_count.values().filter(|&&count| count % 2 != 0).count();

    odd_count <= 1
}
/*
時間計算量: O(n)（1回スキャンでカウント、もう1回で奇数回の文字数を確認）
空間計算量: O(c)（異なる文字の種類数 c に比例するメモリ使用）
*/

pub fn is_palindrome_permutation_bitwise(s: &str) -> bool {
    let mut bitmask: u32 = 0;

    for c in s.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()) {
        let bit_index = c as u32 - 'a' as u32;
        bitmask ^= 1 << bit_index; // 対応するビットを反転
    }

    // 1ビット以下がONなら回文順列
    bitmask == 0 || (bitmask & (bitmask - 1)) == 0
}
/*
時間計算量: O(n)（1回スキャンのみ）
空間計算量: O(1)（u32 1つだけ）
*/