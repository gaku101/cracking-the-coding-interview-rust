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

pub fn is_subtree(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    if t2.is_none() {
        return true;
    }

    if t1.is_none() {
        return false;
    }

    if are_identical(t1, t2) {
        return true;
    }

    let t1_ref = t1.as_ref().unwrap();
    is_subtree(&t1_ref.left, t2) || is_subtree(&t1_ref.right, t2)
}

fn are_identical(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    if t1.is_none() && t2.is_none() {
        return true;
    }

    if t1.is_none() || t2.is_none() {
        return false;
    }

    let node1 = t1.as_ref().unwrap();
    let node2 = t2.as_ref().unwrap();

    node1.val == node2.val
        && are_identical(&node1.left, &node2.left)
        && are_identical(&node1.right, &node2.right)
}
/*
時間計算量: T1 の各ノードで T2 との同一性チェック（are_identical）が行われるため、最悪の場合 O(n * m)（n: T1 のノード数、m: T2 のノード数）。

空間計算量: 再帰呼び出しのスタックはツリーの高さに依存し、O(h)（h: T1 の高さ）となります。
*/