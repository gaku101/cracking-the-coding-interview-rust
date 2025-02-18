mod q_3_2;

use q_3_2::MinStack;

fn main() {
    let mut min_stack = MinStack::new();

    min_stack.push(5);
    println!("最小値: {:?}", min_stack.min().unwrap()); // 出力: 5

    min_stack.push(3);
    println!("最小値: {:?}", min_stack.min().unwrap()); // 出力: 3

    min_stack.push(7);
    println!("最小値: {:?}", min_stack.min().unwrap()); // 出力: 3

    min_stack.pop();
    println!("最小値: {:?}", min_stack.min().unwrap()); // 出力: 3

    min_stack.pop();
    println!("最小値: {:?}", min_stack.min().unwrap()); // 出力: 5
}
