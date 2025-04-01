mod q_4_8;

use q_4_8::{common_ancestor, TreeNode};

fn main() {
    // 以下はツリー構造の例
    //         3
    //        / \
    //       5   1
    //      / \ / \
    //     6  2 0  8
    //       / \
    //      7  4
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode::new(6))),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode::new(7))),
                right: Some(Box::new(TreeNode::new(4))),
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode::new(0))),
            right: Some(Box::new(TreeNode::new(8))),
        })),
    }));

    // 例1: ノード 7 と 4 の共通祖先を探索 (期待される共通祖先は 2)
    if let Some(ancestor) = common_ancestor(&root, 6, 4) {
        println!("共通の祖先: {}", ancestor.val);
    } else {
        println!("共通の祖先は見つかりませんでした");
    }

    // 例2: ノード 5 と 1 の共通祖先を探索 (期待される共通祖先は 3)
    if let Some(ancestor) = common_ancestor(&root, 5, 1) {
        println!("共通の祖先: {}", ancestor.val);
    } else {
        println!("共通の祖先は見つかりませんでした");
    }
}
