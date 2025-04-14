mod q_5_2;

use q_5_2::binary_to_string;

fn main() {
    // サンプル例:
    // 0.625 は正確に 0.101 となる
    let num1: f64 = 0.625;
    // 0.1 は2進表現で無限小数となるため、32桁以内に収まらず "ERROR" が返る
    let num2: f64 = 0.1;

    println!("{} -> {}", num1, binary_to_string(num1));
    println!("{} -> {}", num2, binary_to_string(num2));
}
