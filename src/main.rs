mod q_1_6;

fn main() {
    let test_cases = vec![
        "aabcccccaaa", // "a2b1c5a3"
        "abcdef",      // "abcdef"（圧縮されない）
        "aabbcc",      // "aabbcc"（圧縮されない）
        "aaaaa",       // "a5"
        "",            // ""
        "a",           // "a"
    ];

    for test in test_cases {
        println!(
            "Original: '{}', Compressed: '{}'",
            test,
            q_1_6::compress_string(test)
        );
    }
}
