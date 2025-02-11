#[derive(Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> Self {
        ListNode { value, next: None }
    }
}

pub fn sum_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut p1 = l1;
    let mut p2 = l2;

    // 結果
    let mut result: Option<Box<ListNode>> = None;
    /*
    result_tail は result へのミュータブルな参照（&mut）として定義されているため、
    result_tail を通じて行った更新は、直接 result の内容を変更します。
     */
    let mut result_tail = &mut result;

    while p1.is_some() || p2.is_some() || carry != 0 {
        let x = p1.as_ref().map_or(0, |node| node.value);
        let y = p2.as_ref().map_or(0, |node| node.value);

        let sum = x + y + carry;
        carry = sum / 10;
        let new_digit = sum % 10;

        *result_tail = Some(Box::new(ListNode::new(new_digit)));
        result_tail = &mut result_tail.as_mut().unwrap().next;

        if let Some(node) = p1 {
            p1 = node.next;
        }
        if let Some(node) = p2 {
            p2 = node.next;
        }
    }
    result
}
/*
時間計算量：O(max(n, m)) （n, m はそれぞれのリストの要素数）
空間計算量：O(max(n, m)) （結果のリストの長さに比例）

*/


/*
result はリストの先頭（ヘッド）を保持しており、result_tail はそのリスト内の「次に新しいノードを追加する場所」への可変参照として機能します。

具体的には、

初期状態では、result は None ですが、最初にノードを追加するとそのノードが result に設定されます。
result_tail は最初 &mut result を指しており、ここに新しいノードを代入すると、result 自体が更新されます。
その後、result_tail は追加したノードの next フィールド（次のノードを指す場所）への可変参照に更新されます。
この仕組みでは、result は常にリストの先頭（ヘッド）を保持し続けるため、たとえ result_tail がリストの末尾の位置に移動しても、result が指す先頭は変わりません。
つまり、result_tail を更新して新しいノードを連結していくと、リストの先頭は result によって維持され、そこから辿ることで全てのノードにアクセスできる状態になり
*/

/*
Rustのas_refは、AsRef<T>トレイトのメソッドで、&selfを&Tに変換するためのものです。これは、ある型を参照として別の型に変換する際に便利です。

as_refの基本
fn as_ref(&self) -> &T
	•	selfを&T（参照型）に変換する。
	•	AsRef<T>トレイトを実装している型ならばas_ref()を呼び出せる。

as_ref() の利点
	1.	ジェネリックに使える
	•	T: AsRef<U> の形で、型を柔軟に受け取れる。
	2.	所有権を持つ型を参照型に変換できる
	•	String → &str
	•	Vec<T> → &[T]
	•	PathBuf → &Path
	3.	コピーを避けられる
	•	.clone() ではなく .as_ref() を使うことで不要なコピーを防げる。
*/

/*
result_tail を &mut result として定義する理由は、「結果リストに新しいノードを連結（追加）する際に、どこにノードを追加すればよいかを直接参照できるようにするため」です。具体的な理由は以下の通りです。

末尾の場所を常に保持するためのポインタとして利用

ここでは、結果リスト（result）の末尾、つまり「次に新しいノードを追加すべき場所」を指し示すために、result_tail を使っています。
初めはリストが空（None）なので、result_tail はリストの先頭を表す &mut result となります。
新しいノードの追加を簡潔に行うため

新しいノードを追加する際は、*result_tail = Some(Box::new(ListNode::new(new_digit))); のように書くことで、現在 result_tail が指している場所に新たなノードを設定できます。
ノード追加後、result_tail を新しく追加したノードの next フィールドへの可変参照に更新することで、次回の追加場所を簡単に追跡できるようになります。
「ポインタ・トゥ・ポインタ（pointer to pointer）」のテクニック

この方法は、C言語などで見られる「ポインタ・トゥ・ポインタ」のテクニックと同様です。
リストの先頭（result）そのものではなく、その中に格納されている Option<Box<ListNode>> への可変参照（つまり、どこに新しいノードを追加するかの場所）を持つことで、末尾の更新を直接行えるようにしています。
*/

/*
result_tail.as_mut() の呼び出し

as_mut() メソッドは、Option の中身が Some であれば、その中身への可変参照を返します。
つまり、result_tail.as_mut() は Option<&mut Box<ListNode>> という型になり、result_tail が Some の場合はその内部の Box<ListNode> への可変参照を取得します。
unwrap() によるアンラップ

ここでは、result_tail が必ず Some であることが保証されているため、unwrap() を使って Some の中身を取り出します。
結果として、result_tail.as_mut().unwrap() は &mut Box<ListNode> になります。
*/