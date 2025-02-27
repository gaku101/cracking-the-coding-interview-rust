use std::collections::{LinkedList, VecDeque};

#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

// 与えられた二分木の各深さごとに、ノードの値を連結リストにまとめた Vec を返す関数
pub fn list_of_depths(root: Option<&TreeNode>) -> Vec<LinkedList<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }

    // 現在のレベルのノードを保持するキュー（VecDeque）を用意
    let mut current_level: VecDeque<&TreeNode> = VecDeque::new();
    if let Some(root_node) = root {
        current_level.push_back(root_node);
    }

    while !current_level.is_empty() {
        let mut level_list = LinkedList::new();
        let mut next_level = VecDeque::new();

        while let Some(node) = current_level.pop_front() {
            // 現在のノードの値を連結リストに追加
            level_list.push_back(node.value);

            // 左の子があれば次のレベルへ追加
            if let Some(ref left) = node.left {
                next_level.push_back(&**left);
            }
            // 右の子があれば次のレベルへ追加
            if let Some(ref right) = node.right {
                next_level.push_back(&**right);
            }
            
        }

        result.push(level_list);
        current_level = next_level;
    }
    result
}
/*
各ノードを一度ずつ訪問するため、全体で n 個のノードがあるなら、時間計算量は O(n) となります。
また、探索中にキューや結果のリストに一時的にノードが保持されるため、最悪ケースでは n 個のノードが保持され、空間計算量も O(n) となります。
*/