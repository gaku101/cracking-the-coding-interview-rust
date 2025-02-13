use std::rc::Rc;
use std::cell::RefCell;
mod q_2_7;

use q_2_7::{find_intersection, ListNode};

fn main() {
    // 交差するリストを作成する例

    // 共通部分（交差部分）の作成
    let common = Rc::new(RefCell::new(ListNode {
        value: 8,
        next: Some(Rc::new(RefCell::new(ListNode {
            value: 10,
            next: None,
        }))),
    }));

    // リスト1: 3 -> 1 -> 5 -> 8 -> 10
    let node5 = Rc::new(RefCell::new(ListNode {
        value: 5,
        next: Some(common.clone()),
    }));
    let node1 = Rc::new(RefCell::new(ListNode {
        value: 1,
        next: Some(node5),
    }));
    let head1 = Some(Rc::new(RefCell::new(ListNode {
        value: 3,
        next: Some(node1),
    })));

    // リスト2: 4 -> 8 -> 10 （8,10 はリスト1の共通部分と同じ）
    let head2 = Some(Rc::new(RefCell::new(ListNode {
        value: 4,
        next: Some(common.clone()),
    })));

    // 交差ノードの探索
    if let Some(intersection) = find_intersection(&head1, &head2) {
        println!("交差ノードの値: {}", intersection.borrow().value);
    } else {
        println!("交差はありません");
    }
}
