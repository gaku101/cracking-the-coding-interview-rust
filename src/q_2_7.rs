use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

// ヘッドから末尾まで走査し、リストの長さと末尾ノードを返す
fn get_tail_and_length(
    head: &Option<Rc<RefCell<ListNode>>>,
) -> (usize, Option<Rc<RefCell<ListNode>>>) {
    let mut length = 0;
    let mut current = head.clone();
    let mut tail = None;
    while let Some(node) = current {
        length += 1;
        tail = Some(node.clone());
        current = node.borrow().next.clone();
    }
    (length, tail)
}

// 2 つのリストの交差ノードを返す関数
pub fn find_intersection(
    head1: &Option<Rc<RefCell<ListNode>>>,
    head2: &Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    let (len1, tail1) = get_tail_and_length(head1);
    let (len2, tail2) = get_tail_and_length(head2);

    if tail1.is_none() || tail2.is_none() {
        return None;
    }

    if !Rc::ptr_eq(tail1.as_ref().unwrap(), tail2.as_ref().unwrap()) {
        return None;
    }

    let mut shorter = if len1 < len2 {
        head1.clone()
    } else {
        head2.clone()
    };
    let mut longer = if len1 < len2 {
        head2.clone()
    } else {
        head1.clone()
    };

    let diff = if len1 > len2 {
        len1 - len2
    } else {
        len2 - len1
    };
    for _ in 0..diff {
        if let Some(node) = longer {
            longer = node.borrow().next.clone();
        }
    }

    while let (Some(node1), Some(node2)) = (shorter.clone(), longer.clone()) {
        if Rc::ptr_eq(&node1, &node2) {
            return Some(node1);
        }
        shorter = node1.borrow().next.clone();
        longer = node2.borrow().next.clone();
    }
    None
}

/*
時間計算量: O(n + m)
→ 各リストを一度ずつ走査して長さと末尾を求めるため、n と m（それぞれのリストの長さ）の合計の時間がかかります。

空間計算量: O(1)
→ 追加で使用する領域は定数量で、リストのノード自体は共有しているため、追加の空間はほとんど必要ありません。
*/

/*
Rc<T> (Reference Counted)

Rc<T> は、複数の所有者による共有参照を可能にするスマートポインタです。
参照カウント方式で、複数の変数やデータ構造が同じデータ（この場合はノード）を参照できるようにします。
例えば、リンクリストの交差（2 つのリストが同じノードを共有する）を表現する場合に有用です。
RefCell<T>

RefCell<T> は、内部可変性（interior mutability）を提供する型です。
コンパイル時ではなく実行時に借用ルール（mutable と immutable の排他制御）をチェックします。
これにより、Rc で共有しているデータでも、動的に変更（可変借用）することが可能になります。

*/

/*
Rc::ptr_eq は、Rust の標準ライブラリで定義されている Rc 型に対する関数で、2 つの Rc<T> が「同じ」オブジェクト（同一のメモリアドレス）を参照しているかどうかを判定するために使われます。具体的には以下のような点があります。

ポインタの等価性
単に中身（値）が等しいかどうかではなく、内部の参照先のアドレスが同じかどうか、すなわち同一のアロケーションを指しているかをチェックします。

用途
例えば、リンクリストの交差判定などで、2 つのリストのノードが実際に同じメモリアロケーションを共有しているか確認したい場合に有用です。
*/
