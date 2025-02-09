mod q_2_2;

use q_2_2::{kth_to_last, ListNode};

fn main() {
    // リストの作成例: 1 -> 2 -> 3 -> 4 -> 5
    let node5 = Box::new(ListNode::new(5));
    let mut node4 = ListNode::new(4);
    node4.next = Some(node5);
    let node4 = Box::new(node4);

    let mut node3 = ListNode::new(3);
    node3.next = Some(node4);
    let node3 = Box::new(node3);

    let mut node2 = ListNode::new(2);
    node2.next = Some(node3);
    let node2 = Box::new(node2);

    let mut head_node = ListNode::new(1);
    head_node.next = Some(node2);
    let head = Some(Box::new(head_node));

    let k = 2;
    if let Some(val) = kth_to_last(&head, k) {
        println!("最後から {} 番目の要素は {}", k, val);
    } else {
        println!("リストの要素数が不足しています");
    }
}
