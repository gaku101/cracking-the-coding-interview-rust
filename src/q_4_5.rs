#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn is_valid_bst(root: Option<&TreeNode>) -> bool {
    validate_bst(root, None, None)
}

// ヘルパー関数: 現在のノードが、min, max の範囲内にあるかを再帰的にチェックする
fn validate_bst(node: Option<&TreeNode>, min:Option<i32>, max: Option<i32>) -> bool {
    match node {
        None => true,// 空の木は BST とみなす
        Some(n) => {
            // 下限が設定されている場合、現在の値はそれより大きくなければならない
            if let Some(min_val) = min {
                if n.value <= min_val {
                    return false;
                }
            }

            if let Some(max_val) = max {
                if n.value >= max_val {
                    return  false;
                }
            }

            // 左部分木は、上限を現在の値に更新してチェック
            // 右部分木は、下限を現在の値に更新してチェック
            validate_bst(n.left.as_deref(), min, Some(n.value)) &&
            validate_bst(n.right.as_deref(), Some(n.value), max) 
        }
    }
}
