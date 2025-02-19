#[derive(Debug)]
pub struct SetOfStacks<T> {
    stacks: Vec<Vec<T>>,
    capacity: usize,
}

impl<T> SetOfStacks<T> {
    pub fn new(capacity: usize) -> Self {
        SetOfStacks {
            stacks: Vec::new(),
            capacity,
        }
    }

    // push 操作: 最後のサブスタックが存在し、かつ容量未満ならそこに追加、
    // そうでなければ新しいサブスタックを作成して追加する。
    pub fn push(&mut self, value: T) {
        if let Some(last_stack) = self.stacks.last_mut() {
            if last_stack.len() < self.capacity {
                last_stack.push(value);
                return;
            }
        }
        let mut new_stack = Vec::with_capacity(self.capacity);
        new_stack.push(value);
        self.stacks.push(new_stack);
    }

    // pop 操作: 最後のサブスタックから要素を取り出す。
    // サブスタックが空になったら、そのスタック自体を削除する。
    pub fn pop(&mut self) -> Option<T> {
        if let Some(last_stack) = self.stacks.last_mut() {
            let value = last_stack.pop();
            if last_stack.is_empty() {
                self.stacks.pop();
            }
            return value;
        }
        None
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index >= self.stacks.len() {
            return None;
        }
        let value = self.stacks[index].pop();
        if self.stacks[index].is_empty() {
            self.stacks.remove(index);
        }
        value
    }
}

/*
計算量（Big-O 記法）
push
通常は最後のサブスタックへの push なので O(1)。
ただし、新しいサブスタックを生成する場合は（内部の Vec の再割当が発生する可能性もありますが）概ね O(1) と考えてよいです。

pop
最後のサブスタックからの pop なので O(1)。
サブスタックが空になったときの削除も平均して O(1)（最終要素の場合）。

popAt
指定したサブスタックからの pop 自体は O(1)。
ただし、空になったサブスタックを remove すると、内部の Vec の要素がシフトするため、最悪の場合 O(s) （s はサブスタックの数）となります。
ただし、各サブスタックの容量が固定の場合、s は全体の要素数 n に対して n/capacity と見なせるため、容量が十分大きければ平均的には高速に動作します。

空間計算量
各要素はどこかのサブスタックに格納されるため、全体で O(n) の空間が必要です。
また、サブスタックそのもののオーバーヘッドはありますが、これは要素数に対して線形です。
*/