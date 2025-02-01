use std::collections::HashMap;

pub fn is_palindrome_permutation(s: &str) -> bool {
    let mut char_count = HashMap::new();

    for c in s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
    {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let odd_count = char_count.values().filter(|&count| count % 2 != 0).count();

    odd_count <= 1
}
/*
時間計算量: O(n)（1回スキャンでカウント、もう1回で奇数回の文字数を確認）
空間計算量: O(c)（異なる文字の種類数 c に比例するメモリ使用）
*/

/*
|c| はクロージャの引数を定義している部分です。c は、コレクションの各要素を表しています。
この例では、文字列などの要素として、各文字が c になります。
*/

/*
|&&count| はクロージャの引数の部分です。
ここで使われている &&count の意味は、char_count.values() が &i32 などの参照を返すため、
&count は参照で、&&count でその参照をさらにアンラップして値そのもの（出現回数）にアクセスしていることを意味します。
count は整数（例えば出現回数）を指しています
*/

pub fn is_palindrome_permutation_bitwise(s: &str) -> bool {
    let mut bitmask: u32 = 0;

    for c in s
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
    {
        let bit_index = c as u32 - 'a' as u32;
        bitmask ^= 1 << bit_index; // 対応するビットを反転
    }

    // 1ビット以下がONなら回文順列
    bitmask == 0 || (bitmask & (bitmask - 1)) == 0
}
/*
時間計算量: O(n)（1回スキャンのみ）
空間計算量: O(1)（u32 1つだけ）
*/

/*
1 << bit_index は、ビットシフト演算です。1 を左に bit_index 回シフトする操作です。
	•	1 は 2進数で 000000000000000000000001 という状態です（32ビットの整数の場合）。
	•	これを左に bit_index 回シフトすることで、対応するビット位置に 1 をセットしたビットマスクを作成します。

例えば、bit_index = 3 なら、1 << 3 は次のようになります。
000000000000000000000001  // 1 << 0
000000000000000000000010  // 1 << 1
000000000000000000000100  // 1 << 2
000000000000000000001000  // 1 << 3
*/

/*
bitmask ^= 1 << bit_index

^= 演算子は 排他的論理和（XOR） を意味します。bitmask の対応するビットと、1 << bit_index のビットを反転させます。具体的には以下のように動作します。
	•	^ は排他的論理和（XOR）です。次の2つの条件に基づいてビットを反転します。
	1.	両方のビットが同じ場合、結果は 0 になります。
	2.	両方のビットが異なる場合、結果は 1 になります。

これにより、bitmask の特定のビットが反転します。

例えば、bitmask = 00000000000000000000000000000001 で、bit_index = 3 のとき、
bitmask ^= 1 << 3;
この場合、1 << 3 は 000000000000000000001000 です。XOR 演算を行うと、bitmask の3番目のビットが反転し、次のような結果になります：
00000000000000000000000000000001  // bitmask
^
000000000000000000001000            // 1 << 3
--------------------------------
000000000000000000001001            // 結果
結果的に、bitmask の3番目のビットが反転し、000000000000000000001001 となります。
*/