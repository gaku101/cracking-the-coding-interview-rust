mod q_1_5;

fn main() {
    let test_cases = vec![
        ("pale", "ple"),   // 1文字削除
        ("pales", "pale"), // 1文字削除
        ("pale", "bale"),  // 1文字置換
        ("pale", "bake"),  // 2文字変更（NG）
        ("pale", "pale"),  // 同じ文字列（OK）
        ("apple", "aple"), // 1文字削除（OK）
        ("aple", "apple"), // 1文字挿入（OK）
        ("abc", "def"),    // 3文字変更（NG）
    ];

    for (s1, s2) in test_cases {
        println!("'{}' and '{}' are one edit away? {}", s1, s2, q_1_5::one_edit_away(s1, s2));
    }
}
