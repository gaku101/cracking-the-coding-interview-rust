mod q_4_11;

use q_4_11::TreeNode;

fn main() {
    // サンプルの二分探索木の構築
    //
    //          10
    //         /  \
    //        5    15
    //       / \   /  \
    //      3   7 13  20
    //           \
    //            8
    //
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);
    root.insert(13);
    root.insert(20);
    root.insert(8);

    // get_random() を複数回呼び出してランダムノードの値を表示
    println!("ランダムに選ばれたノードの値:");
    for _ in 0..10 {
        let random_node = root.get_random();
        println!("{}", random_node.val);
    }
}
