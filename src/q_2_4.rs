#[derive(Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}

// partition_list 関数
// head: リストの先頭（Option<Box<ListNode<T>>>）
// x: 分割の基準となる値
// 戻り値: パーティション済みリストの先頭（Option<Box<ListNode<T>>>）
pub fn partition_list<T: PartialOrd>(
    head: Option<Box<ListNode<T>>>,
    x: T,
) -> Option<Box<ListNode<T>>> {
    let mut less: Option<Box<ListNode<T>>> = None;
    let mut greater: Option<Box<ListNode<T>>> = None;

    let mut less_tail = &mut less;
    let mut greater_tail = &mut greater;

    let mut current = head;
    while let Some(mut node) = current {
        current = node.next.take();
        if node.value < x {
            *less_tail = Some(node);
            if let Some(ref mut tail) = less_tail {
                less_tail = &mut tail.next;
            }
        } else {
            *greater_tail = Some(node);
            if let Some(ref mut tail) = greater_tail {
                greater_tail = &mut tail.next;
            }
        }
    }

    *less_tail = greater;
    less
}
/*
時間計算量: 1回の走査で全ノードを処理するので O(n)
空間計算量: ノード自体を再利用しているため、追加で使用するのは定数個のポインタ（less_tail, greater_tail）であり O(1)
*/