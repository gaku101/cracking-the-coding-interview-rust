use std::rc::Rc;
use std::cell::RefCell;
mod q_2_8;

use q_2_8::{find_loop_start, ListNode};

fn main() {
    // ループのあるリンクリストの例を作成
    // リスト: A -> B -> C -> D -> E -> C (ループ開始は C)
    
    // まず、ループに入るノード C, D, E を作成
    let node_c = Rc::new(RefCell::new(ListNode { value: 3, next: None }));
    let node_d = Rc::new(RefCell::new(ListNode { value: 4, next: None }));
    let node_e = Rc::new(RefCell::new(ListNode { value: 5, next: None }));
    
    // リンクを設定
    // C -> D
    node_c.borrow_mut().next = Some(node_d.clone());
    // D -> E
    node_d.borrow_mut().next = Some(node_e.clone());
    // E -> C でループを形成
    node_e.borrow_mut().next = Some(node_c.clone());
    
    // ループに入る前のノード A, B を作成
    let node_b = Rc::new(RefCell::new(ListNode { value: 2, next: Some(node_c.clone()) }));
    let node_a = Rc::new(RefCell::new(ListNode { value: 1, next: Some(node_b.clone()) }));
    
    let head = Some(node_a);
    
    // ループの開始ノードを検出
    if let Some(loop_start) = find_loop_start(&head) {
        println!("ループの開始ノードの値: {}", loop_start.borrow().value);
    } else {
        println!("ループは存在しません");
    }

}
