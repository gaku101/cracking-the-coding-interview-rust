mod q_8_1;
use q_8_1::count_ways;

fn main() {
    let n = 5;
    println!("{} 段の階段の登り方は {} 通りです", n, count_ways(n));
}
