/*
文字列に含まれるすべての文字が一意かどうかを判定する関数を実装してください。
他のデータ構造を使用せずにそれを実現できるかどうかも検討してください。
 */

use std::collections::HashSet;

pub fn is_unique1(s: &str) -> bool {
    let mut char_set = HashSet::new();
    for c in s.chars() {
        if !char_set.insert(c) {
            return false;
        }
    }
    true
}
/*
時間計算量: O(n)（nは文字列の長さ）
空間計算量: O(n)（HashSetが必要）
*/

pub fn is_unique2(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return false;
        }
    }
    true
}
/*
時間計算量: O(n)（ソートにかかる時間）
空間計算量: O(n)（ソート用の配列が必要）
*/

pub fn is_unique3(s: &str) -> bool {
    if s.len() > 128 {
        // ASCIIの範囲外なら一意でない
        return false;
    }
    let mut checker: u32 = 0; // ビットベクトル
    for c in s.chars() {
        let val = (c as u32) - ('a' as u32);
        if (checker & (1 << val)) > 0 {
            return false; // ビットがすでに立っている場合、一意でない
        }
        checker |= 1 << val; // ビットを立てる
    }
    true
}
/*
時間計算量: O(n)
空間計算量: O(n)（ビット操作を利用）
*/

pub fn is_unique4(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}
/*
時間計算量: O(n2)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique1("abcdef")); // 一意
        assert!(!is_unique1("hello")); // 重複あり
    }
}

/*
HashSet は、Rustの標準ライブラリ std::collections に含まれるデータ構造の一つで、重複しない要素の無順序コレクションを管理するために使用されます。
具体的には、セット（集合）として機能し、各要素は一意であることが保証されます。
*/

/*
値が新しく挿入されたかどうかを返します。 つまり、セットが以前にこの値を含んでいなかった場合は、true が返されます。
セットがすでにこの値を含んでいた場合は、false が返され、セットは変更されません： 元の値は置き換えられず、引数として渡された値は削除されます。
*/

/*
.chars() の動作 .chars() は &str に対して利用できるメソッドで、文字列を**文字（Unicodeスカラー値）**単位で分割し、それらを順番に取り出すためのイテレータを生成します。
戻り値 .chars() の戻り値は、Chars 型というイテレータです。このイテレータを使って文字列を1文字ずつ操作できます。
*/

/*
.collect()
    •	説明: イテレーターからすべての要素を収集し、指定したコレクションに変換します。
    •	戻り値: コレクションの型はコンテキストや明示的な型指定によって決まります（例: Vec<char>、HashSet<char> など）。
    •	用途: イテレーターの結果を具体的なデータ構造にまとめたいときに使用します。
*/