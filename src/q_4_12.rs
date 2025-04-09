use std::collections::HashMap;

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

pub fn count_paths_with_sum(root: &Option<Box<TreeNode>>, target_sum: i32) -> i32 {
    // 該当するパスの数
    let mut path_count = 0;
    // 累積和の出現回数を記録するハッシュマップ
    let mut path_map = HashMap::new();
    count_paths(root, 0, target_sum, &mut path_map, &mut path_count);
    path_count
}

fn count_paths(
    node: &Option<Box<TreeNode>>,
    running_sum: i32,// 累積和
    target_sum: i32,// 対象の合計値
    path_map: &mut HashMap<i32, i32>,
    path_count: &mut i32,
) {
    if let Some(ref current) = node {
        let running_sum = running_sum + current.val;

        // 累積和が目標に一致する場合はひとまず1を加算
        if running_sum == target_sum {
            *path_count += 1;
        }

        if let Some(count) = path_map.get(&(running_sum - target_sum)) {
            *path_count += *count;
        }

        *path_map.entry(running_sum).or_insert(0) += 1;

        count_paths(&current.left, running_sum, target_sum, path_map, path_count);
        count_paths(
            &current.right,
            running_sum,
            target_sum,
            path_map,
            path_count,
        );

        if let Some(entry) = path_map.get_mut(&running_sum) {
            *entry -= 1;
        }
    }
}
/*
計算量

Time Complexity: O(n)
→ 各ノードを1度訪問（平均的な状況の場合）

Space Complexity: O(n)
→ 最悪の場合（木が一方に偏っている場合）の再帰呼び出しの深さとハッシュマップのサイズ
*/