pub fn generate_parens(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::with_capacity(n * 2);
    backtrack(n, 0, 0, &mut current, &mut result);
    result
}

fn backtrack(n: usize, open: usize, close: usize, current: &mut String, result: &mut Vec<String>) {
    if current.len() == n * 2 {
        result.push(current.clone());
        return;
    }

    if open < n {
        current.push('(');
        backtrack(n, open + 1, close, current, result);
        current.pop();
    }

    if close < open {
        current.push(')');
        backtrack(n, open, close + 1, current, result);
        current.pop();
    }
}

/*
時間計算量： O(2^n) より少し多い（正確には n 番目のカタラン数）
→ n = 3 のとき、出力は 5 個。n = 4 なら 14 個、n = 5 なら 42 個 など。

空間計算量： O(2n * Cₙ)
→ 各パターンは長さ 2n の文字列で、Cₙ はカタラン数（組合せ数
*/