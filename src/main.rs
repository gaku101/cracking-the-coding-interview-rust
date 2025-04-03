mod q_4_10;

use q_4_10::{is_subtree, TreeNode};

fn main() {
    // T1 の構築
    // T1:
    //           10
    //          /  \
    //         5    15
    //        / \     \
    //       3   7     20
    //      /         /
    //     2         17
    let mut root1 = Box::new(TreeNode::new(10));

    let mut node5 = Box::new(TreeNode::new(5));
    node5.left = Some(Box::new(TreeNode::new(3)));
    node5.right = Some(Box::new(TreeNode::new(7)));
    if let Some(ref mut node3) = node5.left {
        node3.left = Some(Box::new(TreeNode::new(2)));
    }

    let mut node15 = Box::new(TreeNode::new(15));
    node15.right = Some(Box::new(TreeNode::new(20)));
    if let Some(ref mut node20) = node15.right {
        node20.left = Some(Box::new(TreeNode::new(17)));
    }

    root1.left = Some(node5);
    root1.right = Some(node15);
    let t1 = Some(root1);

    // T2 の構築 (T1 の部分木とする)
    // T2:
    //         15
    //           \
    //            20
    //           /
    //          17
    let mut root2 = Box::new(TreeNode::new(15));
    root2.right = Some(Box::new(TreeNode::new(20)));
    if let Some(ref mut node20) = root2.right {
        node20.left = Some(Box::new(TreeNode::new(17)));
    }
    let t2 = Some(root2);

    // T2 が T1 の部分木かチェック
    if is_subtree(&t1, &t2) {
        println!("T2 は T1 の部分木です");
    } else {
        println!("T2 は T1 の部分木ではありません");
    }
}
