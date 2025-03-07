mod q_4_5;

use q_4_5::{is_valid_bst, TreeNode      };

fn main() {
    // BST として有効なツリーを作成
    let root = Box::new(TreeNode {
        value: 10,
        left: Some(Box::new(TreeNode {
            value: 5,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 8,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            value: 15,
            left: Some(Box::new(TreeNode {
                value: 12,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 20,
                left: None,
                right: None,
            })),
        })),
    });

    println!("Is valid BST: {}", is_valid_bst(Some(&root)));

    // ここで、BST の条件に反するノードを作って、検証してみる例
    let invalid_root = Box::new(TreeNode {
        value: 10,
        left: Some(Box::new(TreeNode {
            value: 5,
            left: None,
            right: Some(Box::new(TreeNode {
                value: 12, // 5 の右側に 12 があるので、BST の条件に反する
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            value: 15,
            left: None,
            right: None,
        })),
    });

    println!(
        "Is valid BST (after modification): {}",
        is_valid_bst(Some(&invalid_root))
    );
}
