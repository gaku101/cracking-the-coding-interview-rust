#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}

// kth_to_last 関数は、リストの最後から k 番目の要素への参照を返す
pub fn kth_to_last<T>(head: &Option<Box<ListNode<T>>>, k: usize) -> Option<&T> {
    // as_ref() を使って、Option<Box<ListNode<T>>> から Option<&Box<ListNode<T>>> へ変換する
    let mut pointer1 = head.as_ref();
    let mut pointer2 = head.as_ref();

    // pointer1 を k 回進める
    for _ in 0..k {
        match pointer1 {
            Some(node) => pointer1 = node.next.as_ref(),
            None => return None, // リストの長さが k より短い場合
        }
    }

    // pointer1 が末尾に達するまで、pointer1 と pointer2 を同時に進める
    while let Some(node) = pointer1 {
        pointer1 = node.next.as_ref();
        // pointer2 は必ず Some であることが保証される
        pointer2 = pointer2?.next.as_ref();
    }

    // pointer2 が求める最後から k 番目のノードを指している
    pointer2.map(|node| &node.value)
}
/*
時間計算量: リスト全体を一度だけ走査するので、O(n)
空間計算量: 追加のデータ構造を用いず、ポインタのみで操作するため、O(1)
*/


/*
Rust の ? 演算子は、「早期リターン」のために使われるシンタックスシュガーです。具体的には、Option や Result 型に対して用いると、以下のような動作をします：

もし対象が Some(value) や Ok(value) であれば、その中身の value を取り出します。
もし対象が None や Err(e) であれば、現在の関数から即座に None（またはエラー）を返します。
今回のコードの行

rust
コピーする
pointer2 = pointer2?.next.as_ref();
では、pointer2 は Option<&Box<ListNode<T>>> 型です。pointer2? によって、pointer2 が Some であればその中の値を取り出し、.next.as_ref() を呼び出します。もし pointer2 が None であれば、この時点で関数は None を返す（早期リターンする）という意味になります。

この書き方によって、match 文などで明示的に Some や None の場合分けをする必要がなく、コードが簡潔になります。
*/