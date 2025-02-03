pub fn compress_string(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut compressed = String::new();
    let mut count = 1;
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            compressed.push(chars[i - 1]);
            compressed.push_str(&count.to_string());
            count = 1;
        }
    }

    compressed.push(chars[chars.len() - 1]);
    compressed.push_str(&count.to_string());

    if compressed.len() >= s.len() {
        return s.to_string();
    }

    compressed
}
/*
時間計算量: O(n)(1回のスキャン)
空間計算量: O(n)(圧縮用の String に新しい文字列を作成)
*/

fn compress_string_optimized(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut compressed = Vec::new();
    let mut count = 1;
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            compressed.push(chars[i - 1]);
            for digit in count.to_string().chars() {
                compressed.push(digit);
            }
            count = 1;
        }
    }

    compressed.push(chars[chars.len() - 1]);
    for digit in count.to_string().chars() {
        compressed.push(digit);
    }

    if compressed.len() >= s.len() {
        return s.to_string();
    }

    compressed.into_iter().collect()
}
/*
時間計算量: O(n)
空間計算量: O(n)(メモリ再割り当てを減らせることで効率向上)
*/

/*
なぜ for digit in count.to_string().chars() が必要なのか？
実は push_str() を使えば for ループを回さなくても良いです。

push_str() を使う方法
compressed.push_str(&count.to_string());

メリット: for ループが不要になり、コードが簡潔になる。
デメリット: push_str() は String の長さを変更するためにメモリ再確保が発生する可能性がある。
*/