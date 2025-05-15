use std::collections::HashSet;

/// グリッドのセル座標を表す型エイリアス
pub type Point = (usize, usize);

/// グリッド（true: 通行可, false: 通行不可）から
/// (0,0) → (r-1,c-1) へのパスを探索し、
/// 見つかれば座標の Vec を返す。なければ None。
pub fn find_path(grid: &Vec<Vec<bool>>) -> Option<Vec<Point>> {
    // gridが空でないか、行があっても列が無い構造になっていないか
    if grid.is_empty() || grid[0].is_empty() {
        return None;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut path = Vec::new();
    let mut failed = HashSet::new();
    // 末端から再帰的に辿る
    if find_path_rec(grid, rows - 1, cols - 1, &mut path, &mut failed) {
        Some(path)
    } else {
        None
    }
}

fn find_path_rec(
    grid: &Vec<Vec<bool>>,
    row: usize,
    col: usize,
    path: &mut Vec<Point>,
    failed: &mut HashSet<Point>,
) -> bool {
    // 範囲外または通行不可なら失敗
    if row >= grid.len() || col >= grid[0].len() || !grid[row][col] {
        return false;
    }
    let pt = (row, col);
    println!("Adding point: {:?}", pt);

    // 既にこのセルからは到達不能と判定済み
    if failed.contains(&pt) {
        return false;
    }
    // 原点に到達したか、上または左から到達可能か試す
    let at_origin = row == 0 && col == 0;
    if at_origin
        || (row > 0 && find_path_rec(grid, row - 1, col, path, failed))
        || (col > 0 && find_path_rec(grid, row, col - 1, path, failed))
    {
        println!("Adding point2: {:?}", pt);
        path.push(pt);
        return true;
    }
    // ここまで来たら到達不可と記憶
    failed.insert(pt);
    false
}
/*
時間計算量: O(rows * cols)

各セルは一度だけ再帰的に探索され、HashSet による失敗セルのメモ化で重複探索を防いでいます。

空間計算量: O(rows * cols)

failed: HashSet<Point> に最悪すべてのセルを記録しうるほか、再帰スタックが最大で rows+cols 深さになります。
*/
