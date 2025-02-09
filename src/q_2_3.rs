#[derive(Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}

// delete_middle_node 関数
// 与えられたノード（middle_node）に対して、その次のノードの値とリンクをコピーし、
// 結果的に middle_node の次のノードをリストから除外します。
// ※もし middle_node が最後のノードの場合は削除できないので false を返します。
pub fn delete_middle_node<T>(middle_node: &mut ListNode<T>) -> bool {
    // middle_node.next が Some でなければ、末尾のノードなので削除できない
    if let Some(mut next_box) = middle_node.next.take() {
        // 次のノードの値を現在のノードに上書き
        middle_node.value = next_box.value;
        // 現在のノードの next を、次のノードの next に設定することで、次のノードを飛ばす
        middle_node.next = next_box.next.take();
        true
    } else {
        // 削除対象が末尾の場合は削除不可
        false
    }
}
/*
時間計算量: O(1)（定数回の操作で済む）
空間計算量: O(1)（追加のデータ構造は不要）
*/

/*
関数は、与えられたノード（middle_node）が末尾でない場合にのみ、次のノード（next_box）を取り出して middle_node にその内容を上書きします。
具体的には、middle_node.next.take() により、次のノードの所有権を一時的に取得します。
取得した次のノードの値を現在のノードの値に代入し、middle_node.next を次のノードの next に設定することで、リスト上では次のノードが削除された状態となります。
*/