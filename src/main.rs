mod q_4_2;

use q_4_2::minimal_tree;

fn main() {
    let sorted_array = vec![-12, -10, -3, -2, 0, 5, 9, 11, 13];
    let tree = minimal_tree(&sorted_array);
    println!("最小の高さのBST: {:#?}", tree);
}
/*
時間計算量: O(n)（各要素が一度ずつ処理される）
空間計算量: O(n)（再帰呼び出しとノード格納に n 個の要素を保持）
*/