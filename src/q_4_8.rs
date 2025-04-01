#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn common_ancestor(root: &Option<Box<TreeNode>>, p: i32, q: i32) -> Option<&TreeNode> {
    if let Some(node) = root.as_ref() {
        if node.val == p || node.val == q {
            return Some(node);
        }

        let left = common_ancestor(&node.left, p, q);
        let right = common_ancestor(&node.right, p, q);

        if left.is_some() && right.is_some() {
            return Some(node);
        }

        if left.is_some() {
            return left;
        }
        return right;
    }
    None
}
/*
計算量

時間計算量: O(n) （nはツリーの全ノード数。各ノードを一度ずつ訪問するため）

空間計算量: O(h) （hはツリーの高さ。再帰呼び出しのスタック領域がツリーの高さに依存するため）
*/
