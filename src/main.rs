mod q_5_7;
use q_5_7::pairwise_swap;

fn main() {
    let n = 23;               // 0b00010111
    let swapped = pairwise_swap(n);
    println!("{}", swapped);  // 出力: 43
    println!("{:08b}", swapped); // 出力: 00101011
}
