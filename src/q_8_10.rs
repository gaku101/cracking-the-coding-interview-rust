#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

/// グリッドの色を塗り替える関数（ペイントフィル）
pub fn paint_fill(screen: &mut Vec<Vec<Color>>, x: usize, y: usize, new_color: Color) {
    if screen.is_empty() || screen[0].is_empty() {
        return;
    }

    let target_color = screen[y][x];
    if target_color == new_color {
        return; // すでに塗りたい色なら何もしない
    }

    fill(screen, x, y, target_color, new_color);
}

fn fill(screen: &mut Vec<Vec<Color>>, x: usize, y: usize, target_color: Color, new_color: Color) {
    // 範囲外チェック
    if y >= screen.len() || x >= screen[0].len() {
        return;
    }

    if screen[y][x] != target_color {
        return;
    }

    // 塗る
    screen[y][x] = new_color;

    // 再帰的に上下左右を塗る
    if x > 0 {
        fill(screen, x - 1, y, target_color, new_color);
    }
    if x + 1 < screen[0].len() {
        fill(screen, x + 1, y, target_color, new_color);
    }
    if y > 0 {
        fill(screen, x, y - 1, target_color, new_color);
    }
    if y + 1 < screen.len() {
        fill(screen, x, y + 1, target_color, new_color);
    }
}
/*
時間計算量： O(N)
N は画面内のセル数（最悪全マスを探索）

空間計算量： O(N)（再帰スタック）
*/