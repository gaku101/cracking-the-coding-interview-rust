mod q_8_14;
use q_8_14::count_ways;

fn main() {
    let expr = "1^0|0|1";
    let true_ways  = count_ways(expr, true);
    let false_ways = count_ways(expr, false);
    println!("式 `{}` を true にする方法: {}", expr, true_ways);
    println!("式 `{}` を false にする方法: {}", expr, false_ways);
}