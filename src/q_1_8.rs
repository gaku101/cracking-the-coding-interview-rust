pub fn zero_matrix(matrix: &mut [Vec<i32>]) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();

    let mut row_flags = vec![false; rows];
    let mut col_flags = vec![false; cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                row_flags[i] = true;
                col_flags[j] = true;
            }
        }
    }

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if row_flags[i] || col_flags[j] {
                *cell = 0;
            }
        }
    }
}
/*
時間計算量
1回目の走査:
マトリックス全体（ M×N のセルすべて）を走査するので、O(M×N)。

2回目の走査:
同様に全セルに対して判定・更新を行うため、O(M×N)。

総計:
両方の走査を合わせると、時間計算量は O(M×N) となります。

空間計算量
行ごとに 0 の有無を記録するために O(M) の追加空間、
列ごとに 0 の有無を記録するために O(N) の追加空間を使用します。

総計:
空間計算量は O(M + N) です。
*/

pub fn zero_matrix_constant(matrix: &mut [Vec<i32>]) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();
    if cols == 0 {
        return;
    }

    // 1. 最初の行と最初の列に0があるかを確認する
    let mut first_row_has_zero = false;
    let mut first_col_has_zero = false;

    for j in 0..cols {
        if matrix[0][j] == 0 {
            first_row_has_zero = true;
            break;
        }
    }
    for i in 0..rows {
        if matrix[i][0] == 0 {
            first_col_has_zero = true;
            break;
        }
    }

    // 2. 内部部分（1行目、1列目以降）を走査し、0を見つけたら最初の行・列にフラグを設定する
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0; // この行を0にすべきフラグ
                matrix[0][j] = 0; // この列を0にすべきフラグ
            }
        }
    }

    // 3. 内部セルをフラグに基づいて0に更新する
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    // 4. 最初の行に0があった場合、行全体を0にする
    if first_row_has_zero {
        for j in 0..cols {
            matrix[0][j] = 0;
        }
    }

    // 5. 最初の列に0があった場合、列全体を0にする
    if first_col_has_zero {
        for i in 0..rows {
            matrix[i][0] = 0;
        }
    }
}
