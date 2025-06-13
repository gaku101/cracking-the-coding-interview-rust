use std::collections::HashMap;

/// 式 `expr`（例: "1^0|0|1"）を、desired（true/false）になるように
/// カッコ付けする方法の数を返す。
pub fn count_ways(expr: &str, desired: bool) -> usize {
    // 文字列を Vec<char> に
    let chars: Vec<char> = expr.chars().collect();
    // オペランド（リテラル）の数 = (chars.len() + 1) / 2
    let n_operands = (chars.len() + 1) / 2;
    // 部分式 (i, j, desired) ごとの結果を格納するメモ化テーブルを空で作成。
    let mut memo: HashMap<(usize, usize, bool), usize> = HashMap::new();
    // オペランドインデックス 0..n_operands-1 で再帰
    helper(&chars, 0, n_operands - 1, desired, &mut memo)
}

fn helper(
    chars: &[char],
    i: usize,
    j: usize,
    desired: bool,
    memo: &mut HashMap<(usize, usize, bool), usize>,
) -> usize {
    if let Some(&v) = memo.get(&(i, j, desired)) {
        return v;
    }

    if i == j {
        let val = chars[2 * i] == '1';
        let res = if val == desired { 1 } else { 0 };
        memo.insert((i, j, desired), res);
        return res;
    }

    let mut ways = 0;
    for k in i..j {
        let op = chars[2 * k + 1];
        let lt = helper(chars, i, k, true, memo);
        let lf = helper(chars, i, k, false, memo);

        let rt = helper(chars, k + 1, j, true, memo);
        let rf = helper(chars, k + 1, j, false, memo);

        let true_count = match op {
            '&' => lt * rt,
            '|' => lt * rt + lt * rf + lf * rt,
            '^' => lt * rf + lf * rf,
            _ => 0,
        };
        let false_count = match op {
            '&' => lf * rf + lt * rf + lf * rt,
            '|' => lf * rf,
            '^' => lt * rt + lf * rf,
            _ => 0,
        };

        ways += if desired { true_count } else { false_count };
    }

    memo.insert((i, j, desired), ways);
    ways
}
