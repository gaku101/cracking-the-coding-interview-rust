pub fn urlify1(s: &str, true_length: usize) -> String {
    let mut result = String::new();

    for c in s.chars().take(true_length) {
        if c == ' ' {
            result.push_str("%20");
        } else {
            result.push(c);
        }
    }

    result
}
/*
時間計算量: O(n)（文字列の長さに比例）
空間計算量: O(n)（String の確保が必要）
*/

// take(n) は、イテレータから最初の n 要素だけを取得し、それ以降の要素を無視する制限付きイテレータを生成します。

// push_str は、Rust の String 型に対して 文字列スライス (&str) を追加するメソッド です。

/*
1. usize の特徴
	1.	符号なし (Unsigned)
	•	usize は 負の値を持たない 符号なし整数型です。
	•	0 以上の整数 (0, 1, 2, ...) のみ扱うことができます。
	2.	プラットフォーム依存のサイズ
	•	usize のビット幅は CPU アーキテクチャに依存 します。
	•	32bit システム → usize は 32bit (0～4,294,967,295)
	•	64bit システム → usize は 64bit (0～18,446,744,073,709,551,615)
	•	これは、ポインタ（メモリアドレス）と同じサイズ になるよう設計されているためです。
	3.	インデックスとしてよく使われる
	•	配列やベクターのインデックス (.len() や .get()) などは、usize 型を返します。

    let arr = [10, 20, 30];
    println!("{}", arr.len()); // 出力: 3

    arr.len() の戻り値は usize なので、例えば以下のように usize 型の変数に代入できます。

    let size: usize = arr.len();
    println!("{}", size); // 3
*/

pub fn urlify2(s: &mut Vec<char>, true_length: usize) {
    let mut space_count = 0;

    for i in 0..true_length {
        if s[i] == ' ' {
            space_count += 1;
        }
    }

    let mut index = true_length + space_count * 2;

    for i in (0..true_length).rev() {
        if s[i] == ' ' {
            s[index - 3..index].copy_from_slice(&['%', '2', '0']);
            index -= 3;
        } else {
            s[index - 1] = s[i];
            index -= 1;
        }
    }
}
/*
時間計算量: O(n)（1回スキャンでカウント + 1回スキャンで変更）
空間計算量: O(1)（余分なメモリは使用しない）
*/
