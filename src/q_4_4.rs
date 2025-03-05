#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn check_balance(&self) -> Option<usize> {
        let left_height = match &self.left {
            Some(node) => node.check_balance()?,
            None => 0,
        };
        let right_height = match &self.right {
            Some(node) => node.check_balance()?,
            None => 0,
        };

        if (left_height as isize - right_height as isize).abs() > 1 {
            return None;
        }

        Some(1 + left_height.max(right_height))
    }
}

pub fn is_balanced<T>(root: &Option<Box<TreeNode<T>>>) -> bool {
    match root {
        Some(node) => node.check_balance().is_some(),
        None => true,
    }
}
