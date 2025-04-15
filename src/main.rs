mod q_5_3;

use q_5_3::{get_next_larger, get_next_smaller};

fn main() {
    // サンプル例
    // 例として n = 13948 を用いる（2進数で表すと 0b11011001111100）
    let n: u32 = 13948;
    println!("元の数: {} -> {:b}", n, n);

    match get_next_larger(n) {
        Some(next_larger) => println!("次に大きい数: {} -> {:b}", next_larger, next_larger),
        None => println!("次に大きい数は存在しません"),
    }

    match get_next_smaller(n) {
        Some(next_smaller) => println!("次に小さい数: {} -> {:b}", next_smaller, next_smaller),
        None => println!("次に小さい数は存在しません"),
    }
}
