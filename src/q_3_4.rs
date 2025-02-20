use std::vec::Vec;

pub struct QueueViaStacks<T> {
    stack_newest: Vec<T>,
    stack_oldest: Vec<T>,
}

impl<T> QueueViaStacks<T> {
    pub fn new() -> Self {
        Self {
            stack_newest: Vec::new(),
            stack_oldest: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.stack_newest.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.stack_oldest.is_empty() {
            self.shift_stacks();
        }
        self.stack_oldest.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.stack_oldest.is_empty() {
            self.shift_stacks();
        }
        self.stack_oldest.last()
    }

    pub fn is_empty(&self) -> bool {
        self.stack_newest.is_empty() && self.stack_oldest.is_empty()
    }

    pub fn shift_stacks(&mut self) {
        while let Some(value) = self.stack_newest.pop() {
            self.stack_oldest.push(value);
        }
    }
}
/*
時間計算量: 平均 O(1), 最悪 O(n)（dequeue 時に shift_stacks() を呼び出すため）
空間計算量: O(n)（2つのスタックに最大で n 個の要素を保持するため）
*/