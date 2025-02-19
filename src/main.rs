mod q_3_3;

use q_3_3::SetOfStacks;

fn main() {
    // 各サブスタックの容量を 3 として SetOfStacks を生成
    let mut set_of_stacks = SetOfStacks::new(3);

    // 複数の要素を push する
    for i in 1..=10 {
        set_of_stacks.push(i);
    }
    println!("SetOfStacks の状態: {:?}", set_of_stacks);

    // 通常の pop 操作
    println!("pop: {:?}", set_of_stacks.pop());
    println!("pop: {:?}", set_of_stacks.pop());
    println!("pop: {:?}", set_of_stacks.pop());
    println!("SetOfStacks の状態: {:?}", set_of_stacks);

    // popAt 操作: 第 1 番目（index 0 から数えて）のサブスタックから pop
    println!("popAt(0): {:?}", set_of_stacks.pop_at(0));
    println!("SetOfStacks の状態: {:?}", set_of_stacks);
}
