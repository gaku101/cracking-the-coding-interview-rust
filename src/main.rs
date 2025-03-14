use std::rc::Rc;
mod q_4_6;

use q_4_6::{successor, TreeNode};

fn main() {
    // 例として、以下のBSTを作成する:
    //
    //          20
    //         /  \
    //       10    30
    //       / \     \
    //      5  15     35
    //         /
    //        13
    //
    // 各ノードは親へのポインタも正しく設定する

    let root = TreeNode::new(20);

    let node10 = TreeNode::new(10);
    let node30 = TreeNode::new(30);
    {
        root.borrow_mut().left = Some(Rc::clone(&node10));
        root.borrow_mut().right = Some(Rc::clone(&node30));
        node10.borrow_mut().parent = Some(Rc::downgrade(&root));
        node30.borrow_mut().parent = Some(Rc::downgrade(&root));
    }

    let node5 = TreeNode::new(5);
    let node15 = TreeNode::new(15);
    {
        node10.borrow_mut().left = Some(Rc::clone(&node5));
        node10.borrow_mut().right = Some(Rc::clone(&node15));
        node5.borrow_mut().parent = Some(Rc::downgrade(&node10));
        node15.borrow_mut().parent = Some(Rc::downgrade(&node10));
    }

    let node13 = TreeNode::new(13);
    {
        node15.borrow_mut().left = Some(Rc::clone(&node13));
        node13.borrow_mut().parent = Some(Rc::downgrade(&node15));
    }

    let node35 = TreeNode::new(35);
    {
        node30.borrow_mut().right = Some(Rc::clone(&node35));
        node35.borrow_mut().parent = Some(Rc::downgrade(&node30));
    }

    // テストケース
    // 1. ノード 13 の後継は、親である 15 になるはず
    if let Some(succ) = successor(&node13) {
        println!(
            "Successor of {} is {}",
            node13.borrow().value,
            succ.borrow().value
        );
    } else {
        println!("No successor for {}", node13.borrow().value);
    }

    // 2. ノード 15 の後継は、15 は右部分木を持たないので、親ノードをたどり、node10 の右側にいるため、最終的に root (20) が後継となる
    if let Some(succ) = successor(&node15) {
        println!(
            "Successor of {} is {}",
            node15.borrow().value,
            succ.borrow().value
        );
    } else {
        println!("No successor for {}", node15.borrow().value);
    }

    // 3. ノード 20 (root) の後継は、右部分木があるので、右部分木の最も左側のノード、すなわち node30 自体（または node30 の左部分木があればその最小値）が後継
    if let Some(succ) = successor(&root) {
        println!(
            "Successor of {} is {}",
            root.borrow().value,
            succ.borrow().value
        );
    } else {
        println!("No successor for {}", root.borrow().value);
    }

    // 4. ノード 35 は右部分木もなく、かつ親をたどっても後継が見つからないので、後継は存在しない
    if let Some(succ) = successor(&node35) {
        println!(
            "Successor of {} is {}",
            node35.borrow().value,
            succ.borrow().value
        );
    } else {
        println!("No successor for {}", node35.borrow().value);
    }
}
