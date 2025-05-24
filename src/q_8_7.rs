pub fn permutations(s: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut chars: Vec<char> = s.chars().collect();
    permutate_helper(0, &mut chars, &mut results);
    results
}

pub fn permutate_helper(start: usize, chars: &mut Vec<char>, results: &mut Vec<String>) {
    if start == chars.len() - 1 {
        results.push(chars.iter().collect());
    } else {
        for i in start..chars.len() {
            chars.swap(start, i);
            permutate_helper(start + 1, chars, results);
            chars.swap(start, i);
        }
    }
}

/*
時間計算量:
各階層でループを回しながら再帰するため、
𝑛
n 文字に対して 
𝑛
!
n! 通りの順列を生成し、各生成時に長さ 
𝑛
n の文字列を作成するのでおおよそ O(n · n!)。

空間計算量:
再帰の深さは最大で 
𝑛
n（O(n)）、返却用のベクタには 
𝑛
!
n! 要素それぞれに長さ 
𝑛
n の文字列を保持するため実質 O(n · n!)。
*/