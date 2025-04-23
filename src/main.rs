mod q_5_6;
use q_5_6::conversion;

fn main() {
    let a = 31; // 0b11111
    let b = 14; // 0b01110
    println!("{}", conversion(a, b)); // 出力: 2
}