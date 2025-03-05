mod q_4_4;

use q_4_4::{is_balanced, TreeNode};

fn main() {
    // 例として簡単な木を構築（左右に1つずつ子ノードがある）
    let tree = Some(Box::new(TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    }));

    println!("Tree is balanced: {}", is_balanced(&tree));
}

/*
 • 時間計算量:
　各ノードを一度だけ訪問するため、全体の時間計算量は O(n) となります。
 • 空間計算量:
　再帰呼び出しにより、スタックに積まれるフレーム数は木の高さに依存します。
　・バランスした木の場合は木の高さは O(log n)
　・最悪の場合（偏った木）では O(n)
　となります。
*/