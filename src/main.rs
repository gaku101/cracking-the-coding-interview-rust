mod q_2_3;

use q_2_3::{delete_middle_node, ListNode};

fn main() {
    // リストの作成例: 1 -> 2 -> 3 -> 4 -> 5
    let mut head = Box::new(ListNode::new(1));
    head.next = Some(Box::new(ListNode::new(2)));
    if let Some(ref mut node2) = head.next {
        node2.next = Some(Box::new(ListNode::new(3)));
        if let Some(ref mut node3) = node2.next {
            node3.next = Some(Box::new(ListNode::new(4)));
            if let Some(ref mut node4) = node3.next {
                node4.next = Some(Box::new(ListNode::new(5)));
            }
        }
    }

    // ここでは、値が 3 のノードを削除対象とする
    // head -> 1 -> 2 -> 3 -> 4 -> 5 となっているので、
    // head.next (値 2) の次、つまり head.next.next が値 3 のノードに該当
    if let Some(ref mut node2) = head.next {
        if let Some(ref mut node3) = node2.next {
            // node3 を削除（正確には node3 に次ノードの内容をコピーしている）
            let result = delete_middle_node(node3);
            println!("削除結果: {}", result); // true が出力されれば削除成功
        }
    }

    // リストの状態を表示して結果を確認する
    let mut curr: Option<&ListNode<i32>> = Some(&*head); // head は Box<ListNode<i32>> 型
    while let Some(node) = curr {
        print!("{} ", node.value);
        curr = node.next.as_ref().map(|n| n.as_ref());
    }
    println!();
    
}
