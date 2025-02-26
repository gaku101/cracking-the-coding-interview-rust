#[derive(Debug)]
pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub fn minimal_tree(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let mid = arr.len() / 2;

    Some(Box::new(TreeNode {
        value: arr[mid],
        left: minimal_tree(&arr[0..mid]),
        right: minimal_tree(&arr[mid + 1..]),
    }))
}
