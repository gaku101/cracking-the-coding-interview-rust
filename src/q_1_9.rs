pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    if s1.is_empty() {
        return true;
    }

    let combined = format!("{}{}", s1,s1);
    combined.contains(s2)
}
/*
時間計算量: O(n)（ただし最悪ケースでO(n2) の可能性も）
空間計算量: O(n)(連結した文字列のための追加領域)
*/
/*
Rust の標準ライブラリにおける contains は内部で効率的な文字列探索アルゴリズム（たとえば Two-Way アルゴリズム）を用いているため、平均してO(n)
最悪ケースではO(n2)となる可能性もありますが、一般的には効率的です
*/