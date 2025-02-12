#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}

/// リストが回文かどうかを判定する関数
///
/// アプローチ：
/// 1. slow と fast の2つのポインタでリストの前半部分を走査しながら、
///    slow が指す値をスタックにプッシュする。
/// 2. fast がリストの末尾に到達した時点で、
///    リストが奇数個の場合は slow を1つ進める（中央の要素はスキップ）。
/// 3. slow の位置から後半部分を走査し、スタックから取り出した値と比較する。
///
/// すべての値が一致すれば回文と判断する。
pub fn is_palindrome<T: PartialEq + Clone>(head: &Option<Box<ListNode<T>>>) -> bool {
    let mut stack = Vec::new();
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while fast.is_some() && fast.unwrap().next.is_some() {
        stack.push(slow.unwrap().value.clone());
        slow = slow.unwrap().next.as_ref();
        fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
    }
    if fast.is_some() {
        slow = slow.unwrap().next.as_ref();
    }

    while let Some(node) = slow {
        let top = stack.pop().unwrap();
        if top != node.value {
            return false;
        }
        slow = node.next.as_ref();
    }
    true
}
