mod q_2_1;

use q_2_1::{ListNode, print_list, remove_dups};

fn main() {
    let mut head: Option<Box<ListNode>> = None;

    // リストに値を追加する（例として 3 -> 4 -> 3 -> 2 -> 4 -> None）
    ListNode::push(&mut head, 4);
    ListNode::push(&mut head, 2);
    ListNode::push(&mut head, 3);
    ListNode::push(&mut head, 4);
    ListNode::push(&mut head, 3);

    println!("重複削除前:");
    print_list(&head);

    remove_dups(&mut head);

    println!("重複削除後:");
    print_list(&head);
}
