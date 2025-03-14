use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    // 新しいノードを作成するための補助関数
    pub fn new(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
            parent: None,
        }))
    }
}

// 右部分木から最も左側のノードを探すヘルパー関数
fn leftmost(node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let mut current = node;
    loop {
        let next_left = {
            let current_borrow = current.borrow();
            current_borrow.left.clone()
        };
        if let Some(left) = next_left {
            current = left;
        } else {
            break;
        }
    }
    current
}
pub fn successor(node: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    // Case 1: 右部分木がある場合、右部分木の最も左側のノードが後継
    if let Some(right) = node.borrow().right.clone() {
        return Some(leftmost(right));
    }

    // Case 2: 右部分木がない場合、親へのポインタをたどる
    let mut current = Rc::clone(node);
    loop {
        // 借用をブロックで限定して、parent を取得する
        let parent_opt = {
            let current_borrow = current.borrow();
            current_borrow.parent.clone()
        };
        match parent_opt {
            Some(parent_weak) => {
                if let Some(parent_rc) = parent_weak.upgrade() {
                    // borrow のスコープは上記ブロックで終了しているので安全
                    let is_left_child = {
                        let parent_borrow = parent_rc.borrow();
                        if let Some(ref left_child) = parent_borrow.left {
                            Rc::ptr_eq(left_child, &current)
                        } else {
                            false
                        }
                    };
                    if is_left_child {
                        return Some(Rc::clone(&parent_rc));
                    }
                    current = parent_rc; // 借用は既に解放されているので代入可能
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    // 後継が見つからなければ None を返す
    None
}

/* 
時間計算量: O(h) 
  (h は木の高さを表す。最悪の場合、アンバランスな木では h は n と同等になる)
空間計算量: O(1) 
  (再帰や反復でのポインタ更新に必要な分を除けば、追加で使用するメモリは一定量のみ)
*/

/*
Rc<RefCell>を必要とする理由は、Rustの所有権と借用のルールに関連しています。以下にその理由を説明します。

所有権の共有 (Rc):

Rc（Reference Counted）は、複数の所有者が同じデータを共有できるようにするためのスマートポインタです。ツリー構造では、親ノードと子ノードが相互に参照し合うことがあるため、所有権の共有が必要です。Rcを使うことで、複数のノードが同じ子ノードを所有することができます。
内部可変性 (RefCell):

Rustのデフォルトでは、データは不変（immutable）です。しかし、ツリー構造ではノードの値や子ノードを変更する必要があるため、内部可変性が必要です。RefCellは、実行時に借用チェックを行うことで、コンパイル時には不変なデータを実行時に可変にすることができます。これにより、ツリーのノードを安全に変更することができます。
具体的には、以下のような理由でRc<RefCell<TreeNode>>が使われます：

共有所有権: 親ノードと子ノードが相互に参照し合うため、Rcを使って所有権を共有します。
内部可変性: ノードの値や子ノードを変更する必要があるため、RefCellを使って内部可変性を提供します。
これにより、ツリー構造を安全かつ効率的に操作することができます。
*/

/*
upgradeは、RustのWeakポインタを強い参照であるRcポインタに変換するためのメソッドです。以下にその詳細を説明します。

WeakとRcの違い
Rc (Reference Counted):

Rcは参照カウント付きスマートポインタで、所有権を共有するために使用されます。複数のRcポインタが同じデータを指すことができます。
Rcは強い参照を持ち、参照カウントが0になるまでデータは解放されません。
Weak:

WeakはRcの弱い参照で、所有権を持ちません。Weakポインタは参照カウントに影響を与えず、循環参照を防ぐために使用されます。
Weakポインタはデータが既に解放されている可能性があるため、データにアクセスする前にupgradeメソッドを使って強い参照に変換する必要があります。
upgradeメソッド
upgradeメソッドは、WeakポインタをOption<Rc<T>>に変換します。
もしデータがまだ有効であれば、Some(Rc<T>)を返します。データが既に解放されている場合は、Noneを返します。
*/
