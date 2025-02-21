pub struct SortableStack<T: Ord> {
    pub input_stack: Vec<T>,
}

impl<T: Ord> SortableStack<T> {
    pub fn new() -> Self {
        Self {
            input_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.input_stack.push(value);
    }

    pub fn sort(&mut self) {
        let mut sorted_stack: Vec<T> = Vec::new();

        while let Some(temp) = self.input_stack.pop() {
            // `sorted_stack` に適切な位置で挿入
            while let Some(top) = sorted_stack.last() {
                if *top > temp {
                    self.input_stack.push(sorted_stack.pop().unwrap()); // `sorted_stack` から取り出し、`input_stack` に戻す
                } else {
                    break;
                }
            }
            sorted_stack.push(temp);
        }

        // `sorted_stack` の内容を `input_stack` に戻す（降順のままにする場合は不要）
        self.input_stack = sorted_stack;
    }
}
/*
時間計算量: O(n^2)（各要素を適切な位置に挿入するため）
空間計算量: O(n)（補助スタックを使用）

n 個の要素を sorted_stack に適切な位置で挿入する必要がある
各要素の挿入で最悪 O(n) 回 pop / push を行う
そのため、全体の時間計算量は O(n^2) となる
補助スタック sorted_stack を使うため O(n) の追加空間 を消費
*/

/*
struct SortableStack<T: Ord> {
    input_stack: Vec<T>,
}
このコードの T: Ord は 「T 型は Ord トレイトを実装している必要がある」 という制約を意味します。

Ord トレイトとは？
Ord は Rust の標準トレイトで、大小比較（<, >, <=, >=）が可能な型に適用される ものです。

つまり、この SortableStack<T> は 大小比較ができる型（i32, f64, String など） でしか利用できません。
*/