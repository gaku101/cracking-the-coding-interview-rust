mod q_2_4;

use q_2_4::{partition_list, ListNode};

fn main() {
    // テスト用のリストを作成（例: 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1）
    let mut n1 = Box::new(ListNode::new(3));
    let mut n2 = Box::new(ListNode::new(5));
    let mut n3 = Box::new(ListNode::new(8));
    let mut n4 = Box::new(ListNode::new(5));
    let mut n5 = Box::new(ListNode::new(10));
    let mut n6 = Box::new(ListNode::new(2));
    let n7 = Box::new(ListNode::new(1));

    // リストを連結
    n6.next = Some(n7);
    n5.next = Some(n6);
    n4.next = Some(n5);
    n3.next = Some(n4);
    n2.next = Some(n3);
    n1.next = Some(n2);
    let head = Some(n1);

    // 分割の基準値（例：5）
    let x = 5;
    let partitioned = partition_list(head, x);

    // 結果のリストを走査して出力する
    let mut current: Option<&ListNode<i32>> = partitioned.as_deref();
    while let Some(node) = current {
        print!("{} ", node.value);
        current = node.next.as_deref();
    }
    println!();
}
