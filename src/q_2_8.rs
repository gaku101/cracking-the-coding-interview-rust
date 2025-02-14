use std::cell::RefCell;
use std::rc::Rc;

pub type Link = Option<Rc<RefCell<ListNode>>>;

#[derive(Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Link,
}

pub fn find_loop_start(head: &Link) -> Link {
    let mut slow = head.clone();
    let mut fast = head.clone();

    let mut loop_exists = false;
    while let (Some(slow_node), Some(fast_node)) = (slow.clone(), fast.clone()) {
        slow = slow_node.borrow().next.clone();

        if let Some(next_fast) = fast_node.borrow().next.clone() {
            fast = next_fast.borrow().next.clone();
        } else {
            return None;
        }

        if let (Some(ref s), Some(ref f)) = (slow.clone(), fast.clone()) {
            if Rc::ptr_eq(s, f) {
                loop_exists = true;
                break;
            }
        }
    }
    if !loop_exists {
        return None;
    }

    slow = head.clone();
    while let (Some(ref slow_node), Some(ref fast_node)) = (slow.clone(), fast.clone()) {
        if Rc::ptr_eq(slow_node, fast_node) {
            return Some(slow_node.clone());
        }
        slow = slow_node.borrow().next.clone();
        fast = fast_node.borrow().next.clone();
    }
    None
}
/*
時間計算量: O(n)
→ 最悪の場合、2回の走査が行われますが、全体として線形時間で済みます。

空間計算量: O(1)
→ 追加のポインタや定数個の変数しか使用しないため、空間計算量は定数です。
*/

/*
ref
値をムーブ（所有権の移動）せずに参照として利用できるので、パターンマッチ後も元の値の所有権に影響しません。
*/