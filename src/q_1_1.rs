// src/problem_1_1.rs

pub fn is_unique(s: &str) -> bool {
    let mut char_set = std::collections::HashSet::new();
    for c in s.chars() {
        if !char_set.insert(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique("abcdef")); // 一意
        assert!(!is_unique("hello")); // 重複あり
    }
}
