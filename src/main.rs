mod q_2_5;

use q_2_5::{sum_lists, ListNode};

fn main() {
    // リスト A: 7 -> 1 -> 6 (表す数字は 617)
    let mut a1 = Box::new(ListNode::new(7));
    let mut a2 = Box::new(ListNode::new(1));
    let a3 = Box::new(ListNode::new(6));
    a2.next = Some(a3);
    a1.next = Some(a2);
    let list_a = Some(a1);

    // リスト B: 5 -> 9 -> 2 (表す数字は 295)
    let mut b1 = Box::new(ListNode::new(5));
    let mut b2 = Box::new(ListNode::new(9));
    let b3 = Box::new(ListNode::new(2));
    b2.next = Some(b3);
    b1.next = Some(b2);
    let list_b = Some(b1);

    // 2 つのリストの和を求める（617 + 295 = 912）
    let result = sum_lists(list_a, list_b);

    // 結果のリストを表示（逆順なので 2 -> 1 -> 9 が出力される）
    print!("Result list: ");
    let mut current = result.as_ref();
    while let Some(node) = current {
        print!("{} -> ", node.value);
        current = node.next.as_ref();
    }
    println!("None");
}
