pub fn check_permutation1(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();

    chars1.sort_unstable();
    chars2.sort_unstable();

    chars1 == chars2
}
/*
時間計算量: O(n log n)（ソートが支配的）
空間計算量: O(n)（ソート用の配列を作成）
*/

pub fn check_permutation2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    // ASCII文字のカウント用配列（全て0で初期化）
    let mut char_count = [0; 128];

    for c in s1.chars() {
        char_count[c as usize] += 1;
    }

    for c in s2.chars() {
        char_count[c as usize] -= 1;
        if char_count[c as usize] < 0 {
            return false;
        }
    }

    true
}
/*
時間計算量: O(n)（文字列の長さに比例）
空間計算量: O(1)（固定サイズの配列使用）
*/

/*
 c as usize
    •	cは1文字（char型）で、例えばASCII文字の場合、その文字に対応するASCII値（u32型）を持ちます。
    •	as usizeで、cをインデックスとして利用可能な型（usize型）に変換します。
    •	例えば：
        'a' as usize = 97
        'A' as usize = 65
        '1' as usize = 49
*/
