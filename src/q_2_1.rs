use std::collections::HashSet;

#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn push(head: &mut Option<Box<ListNode>>, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: head.take(),
        });
        *head = Some(new_node)
    }
}

/// この方法では各ノードを一度ずつ走査し、すでに出現した値ならばそのノードをリストから除去します。
pub fn remove_dups(head: &mut Option<Box<ListNode>>) {
    let mut seen = HashSet::new();
    // cur は head 内のあるノードへの可変参照を保持する
    let mut cur = head;

    while cur.is_some() {
        // ブロックを作成し、ここでの node への可変借用を終了させる
        let remove_current = {
            // cur.as_mut().unwrap() により、現在のノードへの可変参照を取得
            let node = cur.as_mut().unwrap();
            // 既に見た値なら削除対象とする
            if seen.contains(&node.val) {
                true
            } else {
                seen.insert(node.val);
                false
            }
        };

        if remove_current {
            // 現在のノードを削除するため、node.next を取り出し cur に再代入する
            let next = cur.as_mut().unwrap().next.take();
            *cur = next;
        } else {
            // 現在のノードは削除しないので、cur を次のノードに進める
            cur = &mut cur.as_mut().unwrap().next;
        }
    }
}

// リストの内容を表示するヘルパー関数
pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut cur = head;
    while let Some(ref node) = cur {
        print!("{} -> ", node.val);
        cur = &node.next;
    }
    println!("None");
}

/*
時間計算量:
リスト内の各ノードを1度ずつ訪問するため、要素数をnとすると O(n) です。
※ハッシュセットの挿入・探索は平均 O(1) とみなされます。

空間計算量:
重複検出のためにハッシュセットを使用しており、最悪の場合リスト内すべての要素を保持するため O(n) の追加空間が必要です。
*/

/*
Box<T>
Rustでは、再帰的なデータ構造（自分自身を含む構造） を作る場合、Box<T> のようなヒープ確保（heap allocation） を行うスマートポインタを使用する必要があります。
Box<ListNode> の意味は：
	•	ListNode のインスタンスをヒープ上に確保する
	•	Box<ListNode> を使って、そのポインタをスタック上に保持する
もし Box を使わずに next: Option<ListNode> にしてしまうと、コンパイルエラー（無限のサイズを持つ構造体） になってしまいます。
*/

/*
next: head.take()
	•	head.take() は head の中身を取り出し (Some の場合)、head を None にする
	•	つまり、new_node.next = head のように、既存のリストを 新しいノードの next に設定
*/

/*
HashSet の基本

HashSet は、値の集合を保持するデータ構造 であり、以下の特徴を持ちます：
	•	重複する値を自動的に排除
	•	要素の順番は保証されない
	•	O(1) の時間計算量で insert（追加）や contains（存在確認）が可能
	•	値の型は Hash + Eq トレイトを実装している必要がある
	•	i32, String などはデフォルトでサポート
*/

/*
if remove_current {
    let next = cur.as_mut().unwrap().next.take();
    *cur = next;
}
🔹 何をしているか
	•	cur.as_mut().unwrap().next.take();
	•	cur が指しているノードの next の所有権を取得
	•	取得した後、next の値は None になる（take() の動作）
	•	*cur = next;
	•	cur に next を代入し、現在のノードをスキップして次のノードに更新する
	•	現在のノードを削除する処理
どんな場合に使うか
	•	現在のノードを削除したいとき
	•	cur の値（Option<Box<ListNode>>）を直接書き換えるとき
*/

/*
cur = &mut cur.as_mut().unwrap().next;
🔹 何をしているか
	•	cur.as_mut().unwrap() で 現在のノードの可変参照を取得
	•	&mut cur.as_mut().unwrap().next で 次のノードへの可変参照を cur に代入
	•	cur の指す場所が 次のノードの next フィールド に変わる
	•	現在のノードを削除せずにスキップして次へ進む処理

🔹 どんな場合に使うか
	•	現在のノードを削除せず、単に次のノードに進みたいとき
	•	リストを1つずつたどるとき
 */