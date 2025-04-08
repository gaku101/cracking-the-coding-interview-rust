use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
    pub size: usize,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            size: 1,
        }
    }

    pub fn insert(&mut self, val: i32) {
        if val <= self.val {
            match self.left.as_mut() {
                Some(left_node) => {
                    left_node.insert(val);
                }
                None => self.left = Some(Box::new(TreeNode::new(val))),
            }
        } else {
            match self.right.as_mut() {
                Some(right_node) => {
                    right_node.insert(val);
                }
                None => self.right = Some(Box::new(TreeNode::new(val))),
            }
        }
        self.size += 1;
    }

    pub fn get_random(&self) -> &TreeNode {
        let left_size = self.left.as_ref().map(|node| node.size).unwrap_or(0);
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.size);
        match index.cmp(&left_size) {
            Ordering::Less => self.left.as_ref().unwrap().get_random(),
            Ordering::Equal => self,
            Ordering::Greater => self.right.as_ref().unwrap().get_random(),
        }
    }
}
/*
時間計算量: get_random() はツリーの高さ h に対して O(h) の時間で動作します。

空間計算量: 再帰呼び出しのスタックの深さに依存して O(h) となります。
*/
