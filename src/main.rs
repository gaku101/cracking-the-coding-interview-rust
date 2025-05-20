mod q_8_5;
use q_8_5::recursive_multiply;

fn main() {
    let a = 13;
    let b = 9;
    println!("{} * {} = {}", a, b, recursive_multiply(a, b));
}
