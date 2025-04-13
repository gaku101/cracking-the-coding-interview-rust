mod q_5_1;

use q_5_1::insert_number;

fn main() {
    // サンプル例:
    // N = 0b10000000000 (1024)
    // M = 0b10011      (19)
    // i = 2, j = 6 の位置に M を挿入する
    let n: u32 = 0b10000000000;
    let m: u32 = 0b10011;
    let i: u32 = 2;
    let j: u32 = 6;

    let result = insert_number(n, m, i, j);
    println!("N (挿入前): {:b}", n);
    println!("M:            {:b}", m);
    println!("挿入結果:     {:b}", result);
}
