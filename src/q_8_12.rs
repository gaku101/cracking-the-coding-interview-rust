/// Nクイーン問題を解く：すべての配置（列インデックス）を返す
pub fn solve_n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut positions = Vec::with_capacity(n);
    backtrack(n, 0, &mut positions, &mut results);
    results
}

/// 再帰で探索する
fn backtrack(n: usize, row: usize, positions: &mut Vec<usize>, results: &mut Vec<Vec<usize>>) {
    if row == n {
        results.push(positions.clone());
        return;
    }

    for col in 0..n {
        if is_safe(row, col, positions) {
            positions.push(col);
            backtrack(n, row + 1, positions, results);
            positions.pop(); // 巻き戻し
        }
    }
}

/// クイーンを (row, col) に置いてよいかチェック
fn is_safe(row: usize, col: usize, positions: &[usize]) -> bool {
    for (r, &c) in positions.iter().enumerate() {
        if c == col {
            return false; // 同じ列
        }
        if (r as isize - row as isize).abs() == (c as isize - col as isize).abs() {
            return false; // 同じ斜め
        }
    }
    true
}
