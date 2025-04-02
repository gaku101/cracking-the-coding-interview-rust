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

pub fn bst_sequences(root: &Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![vec![]];
    }

    let node = root.as_ref().unwrap();
    let prefix = vec![node.val];

    let left_seq = bst_sequences(&node.left);
    let right_seq = bst_sequences(&node.right);

    let mut result = Vec::new();

    for left in left_seq.iter() {
        for right in right_seq.iter() {
            let mut weaved = Vec::new();
            let mut prefix_clone = prefix.clone();
            weave_lists(left, right, &mut prefix_clone, &mut weaved);
            result.extend(weaved);
        }
    }
    result
}

/// 2つのリスト（first, second）を、各リスト内の順序を保ったまま交互に織り交ぜた全パターンを生成する補助関数
///
/// 引数:
/// - first: 交互配置の元となる1つ目のリスト
/// - second: 交互配置の元となる2つ目のリスト
/// - prefix: 現在までに決定済みのシーケンス（呼び出し元で先頭の要素が入っている）
/// - results: 結果として得られた全シーケンスを格納するベクタ
fn weave_lists(first: &[i32], second: &[i32],prefix: &mut Vec<i32>,results: &mut Vec<Vec<i32>>) {
    // どちらかのリストが空になったら、残りを連結して結果に追加
    if first.is_empty() || second.is_empty() {
        let mut result = prefix.clone();
        result.extend_from_slice(first);
        result.extend_from_slice(second);
        results.push(result);
        return;
    }

    // firstリストから先頭要素を取って再帰呼び出し
    {
        let head_first = first[0];
        prefix.push(head_first);
        weave_lists(&first[1..], second, prefix, results);
        prefix.pop();
    }

    // secondリストから先頭要素を取って再帰呼び出し
    {
        let head_second = second[0];
        prefix.push(head_second);
        weave_lists(first, &second[1..], prefix, results);
        prefix.pop();
    }
}

/*
時間計算量: 最悪の場合、生成されるシーケンス数が指数関数的に増加するため O(2^n)（nはノード数）となります。

空間計算量: 再帰呼び出しのスタック深度は木の高さ O(h) ですが、最終的に保持するシーケンスの数も指数関数的なため、全体としては非常に大きな領域が必要になる可能性があります。
*/