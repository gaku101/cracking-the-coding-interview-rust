mod q_4_9;

use q_4_9::{bst_sequences, TreeNode};

fn main() {
// サンプルの二分探索木（より深い階層構造）を構築
    //
    //             5
    //           /   \
    //          3     7
    //         / \   / \
    //        2   4 6   8
    //       /
    //      1
    //
    let mut root = Box::new(TreeNode::new(5));
    
    // 左部分木の構築
    let mut node3 = Box::new(TreeNode::new(3));
    node3.left = Some(Box::new(TreeNode::new(2)));
    node3.right = Some(Box::new(TreeNode::new(4)));
    // ノード2に左子として1を追加
    if let Some(ref mut node2) = node3.left {
        node2.left = Some(Box::new(TreeNode::new(1)));
    }
    
    // 右部分木の構築
    let mut node7 = Box::new(TreeNode::new(7));
    node7.left = Some(Box::new(TreeNode::new(6)));
    node7.right = Some(Box::new(TreeNode::new(8)));
    
    // ルートに左右の部分木を追加
    root.left = Some(node3);
    root.right = Some(node7);
    
    let tree = Some(root);
    
    // BSTシーケンスを求める
    let sequences = bst_sequences(&tree);
    
    println!("BSTシーケンスの全パターン:");
    for seq in sequences {
        println!("{:?}", seq);
    }
}
