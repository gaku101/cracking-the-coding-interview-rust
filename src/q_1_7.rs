pub fn rotate_matrix(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {
            let offset = i - first;
            // 上側の要素を一時保存
            let top = matrix[first][i]; //1巡目なら(0,0) = 上の左端
                                        // 左側 -> 上側
            matrix[first][i] = matrix[last - offset][first]; //1巡目なら(3,0) = 下の左端
                                                             // 下側 -> 左側
            matrix[last - offset][first] = matrix[last][last - offset]; //1巡目なら(3,3) = 下の右端
                                                                        // 右側 -> 下側
            matrix[last][last - offset] = matrix[i][last]; //1巡目なら(0,3) = 上の右端
                                                           // 上側（保存しておいた値） -> 右側
            matrix[i][last] = top;
        }
    }
}
/*
時間計算量
レイヤーの数:
行列のサイズが n×n の場合、レイヤーは外側から内側へ n/2 層存在します。

各レイヤーでの操作:
各レイヤーに対して、上下左右の4要素を交換するために、ほぼ n 回のループを実施します。
つまり、各レイヤーでの操作回数は O(n) となります。

総計:
全体では O(n/2 * n) = O(n²) 回の定数時間の操作を行うため、
時間計算量は O(n²) となります。

空間計算量
このアルゴリズムは、行列そのものを in-place（直接上書き）で回転させるため、
追加で使用するのは一時的な変数（例: top など、定数個の変数）だけです。

そのため、空間計算量は O(1)（定数空間） となります。
*/

/*
「in-place」はアルゴリズムの中でデータ構造を直接変更し、追加のメモリを使わない方法を指している。
O(1)の追加空間で動作している。
*/