mod q_8_11;
use q_8_11::count_ways;

fn main() {
    let amount = 10;
    let ways = count_ways(amount);
    println!("{} セントを構成する方法は {} 通りあります。", amount, ways);
}
