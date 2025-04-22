// main.rs
mod q_5_3;
use q_5_3::flip_bit_to_win;

fn main() {
    let n = 0b11011101111; // 1775
    println!("{}", flip_bit_to_win(n)); // 出力: 8
}
