mod q_4_3;

use q_4_3::{list_of_depths, TreeNode};

fn main() {
    // 以下のような二分木を作成
    //         0
    //        / \
    //      -3   11
    //     /  \  / \
    //  -10  -2 9  13
    //   /
    // -12
    let mut root = Box::new(TreeNode::new(0));
    root.left = Some(Box::new(TreeNode::new(-3)));
    root.right = Some(Box::new(TreeNode::new(11)));

    if let Some(ref mut left) = root.left {
        left.left = Some(Box::new(TreeNode::new(-10)));
        left.right = Some(Box::new(TreeNode::new(-2)));
        if let Some(ref mut left_left) = left.left {
            left_left.left = Some(Box::new(TreeNode::new(-12)));
        }
    }

    if let Some(ref mut right) = root.right {
        right.left = Some(Box::new(TreeNode::new(9)));
        right.right = Some(Box::new(TreeNode::new(13)));
    }

    let lists = list_of_depths(Some(&root));

    // 各深さごとの連結リストを出力
    for (depth, list) in lists.iter().enumerate() {
        println!("Depth {}: {:?}", depth, list);
    }
}
