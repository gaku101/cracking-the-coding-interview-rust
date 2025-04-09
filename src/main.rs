mod q_4_12;

use q_4_12::{count_paths_with_sum, TreeNode};

fn main() {
    // サンプルの二分木の作成
    // 例として以下の木を作成：
    //         10
    //        /  \
    //       5   -3
    //      / \    \
    //     3   2   11
    let mut root = Box::new(TreeNode::new(10));
    root.left = Some(Box::new(TreeNode::new(5)));
    root.right = Some(Box::new(TreeNode::new(-3)));

    if let Some(ref mut left_node) = root.left {
        left_node.left = Some(Box::new(TreeNode::new(3)));
        left_node.right = Some(Box::new(TreeNode::new(2)));
    }
    if let Some(ref mut right_node) = root.right {
        right_node.right = Some(Box::new(TreeNode::new(11)));
    }

    let target = 8;
    let result = count_paths_with_sum(&Some(root), target);
    println!("合計 {} となるパスの数: {}", target, result);
}
