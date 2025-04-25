/// モノクロ画面 screen に
/// 幅 width、行 y の座標系で
/// x1 から x2 までの水平線を描画する
pub fn draw_line(screen: &mut [u8], width: usize, x1: usize, x2: usize, y: usize) {
    // 1行あたりのバイト数
    let bytes_per_row = width / 8;
    let start_byte = y * bytes_per_row + (x1 / 8); // y行目の先頭バイト位置 + x各座標が属するバイト位置。
    let end_byte = y * bytes_per_row + (x2 / 8);

    // % 8 で、バイト内の何ビット目か（0 が MSB、7 が LSB）。
    let start_offset = x1 % 8;
    let end_offset = x2 % 8;

    if start_byte == end_byte {
        // 同じバイト内に描画範囲が収まる場合
        // 例: 0b1111_1111 >> start_offset で右側 start_offset ビットを削り、
        //     0b1111_1111 << (7 - end_offset) で左側 (7-end_offset) ビットを削る
        let mask = (0xFF >> start_offset) & (0xFF << (7 - end_offset));
        screen[start_byte] |= mask;
    } else {
        // 開始バイトの部分描画: 右から start_offset ビット分だけ 0 に
        screen[start_byte] |= 0xFF >> start_offset;
        // 完全に線が通る中間バイトはすべて 0xFF
        for byte in screen[start_byte + 1..end_byte].iter_mut() {
            *byte = 0xFF;
        }
        // 終了バイトの部分描画: 左から (7-end_offset) ビット分だけ 0 に
        screen[end_byte] |= 0xFF << (7 - end_offset);
    }
}

/*
時間計算量: O(n)

描画範囲に含まれるバイト数（≒ (x2/8)-(x1/8)+1）に比例します。

空間計算量: O(1)

補助変数は定数個のみ使用しています。
*/

/*
行オフセット (row offset)

画面全体を１次元配列（screen: &mut [u8]）で表現しているとき、各「行」は横幅 width のピクセル分を連続したバイトで表します。

たとえば width = 32 なら、１行あたり 32 / 8 = 4 バイト使います。

行オフセットとは「y 行目の先頭が配列の何バイト目にあたるか」を示す値で、

rust
コピーする
編集する
let bytes_per_row = width / 8;        // 1行あたりのバイト数
let row_offset   = y * bytes_per_row; // y行目の先頭バイト位置
と計算します。

これに「行内で何バイト目か」を加えることで、目的のバイトインデックス start_byte／end_byte が得られます。

ビットオフセット (bit offset)

１バイト（u8）は「8ピクセル」を表し、ビット位置は 0〜7 で番号を振ります。

オフセット 0 → MSB（最上位ビット）、画面上の左端のピクセル

オフセット 7 → LSB（最下位ビット）、画面上の右端のピクセル

ビットオフセットとは「そのバイトの中で、何番目のビットにあたるか」を示す値で、

rust
コピーする
編集する
let bit_offset = x % 8;
と計算します（x は画面上のピクセルの横座標）。
*/

/*
1. 0xFF >> start_offset の役割
0xFF は 8 ビットすべて 1、つまり 0b1111_1111。

>> start_offset で右に start_offset ビットだけシフトすると、下位（右側）の start_offset ビットが捨てられ、
左側には 0 が詰められます。

例：start_offset = 2 のとき

ruby
コピーする
編集する
0xFF        = 1111_1111
>> 2        = 0011_1111
これにより「開始位置以前のビットだけ 0、以降は 1」のマスクが得られます。

2. 0xFF << (7 - end_offset) の役割
同じく 0xFF = 1111_1111。

<< (7 - end_offset) で左に (7 - end_offset) ビットだけシフトすると、上位（左側）の (7 - end_offset) ビットが捨てられ、
右側には 0 が詰められます。

例：end_offset = 5 のとき、7 - end_offset = 2

bash
コピーする
編集する
0xFF        = 1111_1111
<< 2        = 1111_1100
これにより「終了位置以降のビットだけ 0、以降は 1」のマスクが得られます。

3. 両者を AND して範囲マスクを作成
rust
コピーする
編集する
let mask = (0xFF >> start_offset) & (0xFF << (7 - end_offset));
左のマスク (…0011_1111) と右のマスク (1111_1100) をビットごとに AND すると、
start_offset から end_offset の間だけ 1、残りは 0 のビットパターンになります。

例：start_offset=2, end_offset=5 のとき

bash
コピーする
編集する
   0011_1111  (0xFF >> 2)
AND 1111_1100  (0xFF << 2)
=   0011_1100  (= 0x3C)
これで「このバイトの中で線を引くべきビット位置だけ」が 1 のマスク mask が完成します。

4. OR 演算で画面データに反映
rust
コピーする
編集する
screen[start_byte] |= mask;
screen[start_byte] は該当バイトの現在のビットパターン。

|= mask でマスクの 1 の部分をすべて 1 に上書きします（既に 1 のビットはそのまま、0 のビットだけが 1 に変わる）。

結果として、同一バイト内の start_offset から end_offset までのビットだけが確実に “線がある” 状態（1）になります。







あなた:
|= maskを文法的に解説


ChatGPT:
Rust における |= mask は、ビットごとの論理和（OR）と代入を同時に行う「複合代入演算子」です。文法的には以下のように分解できます。

rust
コピーする
編集する
screen[start_byte] |= mask;
左辺（被代入式）

screen[start_byte]

これは「配列（スライス） screen の start_byte 番目要素」を指します。

可変参照（&mut [u8]）なので、要素への書き換えが可能です。

演算子 |=

a |= b は、内部的に

rust
コピーする
編集する
a = a | b;
と同等です。

| はビット単位の OR 演算子。二つのビットがいずれか一方でも 1 なら結果のビットが 1 になります。

= は代入演算子。右辺の式を評価して左辺に書き戻します。

|= はこれらをまとめたもので、「左辺の現在値と右辺を OR し、その結果を左辺に再代入」します。

右辺（オペランド）

mask

ここでは「線を引きたいビット位置だけ 1 にしたマスク」です。

処理の流れ

まず screen[start_byte] の現在の値（たとえば 0b1010_0001）と mask（たとえば 0b0011_1100）をビットごとに OR します。

コピーする
編集する
1010_0001
| 0011_1100
1011_1101

markdown
コピーする
編集する
2. 得られた新しいビットパターン（`0b1011_1101`）を、同じ要素 `screen[start_byte]` に書き戻します。

デスティネーション評価

|= の左辺は一度だけ評価され、あとの代入先として効率よく扱われます。

つまり、オーバーヘッドなく「同じ場所への読み取り→演算→書き戻し」が行われます。

まとめると、x |= mask; は「x = x | mask;」の省略形で、元の値をビットごとの OR で更新する演算子です。ネルサンプルとして、元のビットを保持しつつ特定のビットだけを確実に 1 にしたい場面でよく使われます。
 */