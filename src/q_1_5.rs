pub fn one_edit_away(s1: &str, s2: &str) -> bool {
    let len1 = s1.len();
    let len2 = s2.len();

    if (len1 as isize - len2 as isize).abs() > 1 {
        return false;
    }

    if len1 == len2 {
        return one_replace_away(s1, s2);
    }

    if len1 > len2 {
        one_insert_or_delete_away_optimized(s2, s1)
    } else {
        one_insert_or_delete_away_optimized(s1, s2)
    }
}
/*
時間計算量: O(n)
空間計算量: O(n)(最悪の場合)
*/

fn one_replace_away(s1: &str, s2: &str) -> bool {
    let mut found_difference = false;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found_difference {
                return false;
            }
            found_difference = true;
        }
    }
    true
}
/*
時間計算量: O(n)
空間計算量: O(1)
*/

fn one_insert_or_delete_away(shorter: &str, longer: &str) -> bool {
    let mut s_index = 0;
    let mut l_index = 0;
    let mut found_difference = false;

    let s_chars: Vec<char> = shorter.chars().collect();
    let l_chars: Vec<char> = longer.chars().collect();

    while s_index < s_chars.len() && l_index < l_chars.len() {
        if s_chars[s_index] != l_chars[l_index] {
            if found_difference {
                return false;
            }
            found_difference = true;
            l_index += 1; // longerの方だけ進める（挿入/削除チェック）
        } else {
            s_index += 1;
            l_index += 1;
        }
    }

    true
}
/*
時間計算量: O(n)
空間計算量: O(n)
*/

fn one_insert_or_delete_away_optimized(shorter: &str, longer: &str) -> bool {
    let iter1 = shorter.chars();
    let mut iter2 = longer.chars();
    let mut found_difference = false;

    for c1 in iter1 {
        let c2 = iter2.next().unwrap(); // shorter の長さ以内なので安全
        if c1 != c2 {
            if found_difference {
                return false;
            }
            found_difference = true;
            // longer のみ1つ進める（挿入/削除の考慮）
            iter2.next();
        }
    }
    true
}
/*
時間計算量: O(n)
空間計算量: O(1)
*/


/*
zip メソッドは、2つのイテレータをペアにして返すメソッドです。
s1.chars() と s2.chars() が返すイテレータを、それぞれの要素が組み合わせてタプル (c1, c2) を作りながら順に進めます。
*/

/*
isize は、Rust の整数型の一つで、符号付き整数（signed integer）であり、プラットフォームのアーキテクチャに依存するサイズを持つ型です。
*/