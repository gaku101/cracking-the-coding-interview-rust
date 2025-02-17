mod q_3_1;

use q_3_1::ThreeStacks;

fn main() {
    // 各スタックの容量を5に設定
    let mut stacks = ThreeStacks::new(5);

    // --- push 操作のテスト ---
    println!("各スタックに値を push します");
    stacks.push(0, 10).unwrap();
    stacks.push(0, 20).unwrap();
    stacks.push(0, 30).unwrap();

    stacks.push(1, 40).unwrap();
    stacks.push(1, 50).unwrap();

    stacks.push(2, 60).unwrap();
    stacks.push(2, 70).unwrap();
    stacks.push(2, 80).unwrap();

    // --- peek 操作のテスト ---
    println!("スタック 0 のトップ: {}", stacks.peek(0).unwrap());
    println!("スタック 1 のトップ: {}", stacks.peek(1).unwrap());
    println!("スタック 2 のトップ: {}", stacks.peek(2).unwrap());

    // --- pop 操作のテスト ---
    println!("スタック 0 から pop: {}", stacks.pop(0).unwrap());
    println!("スタック 0 の新しいトップ: {}", stacks.peek(0).unwrap());

    println!("スタック 1 から pop: {}", stacks.pop(1).unwrap());
    println!("スタック 1 の新しいトップ: {}", stacks.peek(1).unwrap());

    println!("スタック 2 から pop: {}", stacks.pop(2).unwrap());
    println!("スタック 2 のトップ: {}", stacks.peek(2).unwrap());

    // --- エラー処理のテスト ---
    // スタック 1 の残りをすべて pop して空にする
    println!("スタック 1 から pop: {}", stacks.pop(1).unwrap());
    println!("スタック 1 から pop: {}", stacks.pop(1).unwrap());
    // 既に空のスタック 1 に対して pop を試みる
    match stacks.pop(1) {
        Ok(value) => println!("スタック 1 から pop: {}", value),
        Err(e) => println!("スタック 1 の pop エラー: {}", e),
    }
}
