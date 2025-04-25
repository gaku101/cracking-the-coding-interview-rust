mod q_5_8;
use q_5_8::draw_line;

fn main() {
    // 幅 32px、行数 3 のモノクロ画面を用意（各行 32/8 = 4 バイト）
    let width = 32;
    let mut screen = vec![0u8; width/8 * 3];

    // 2 行目 (y=1) の x=5 から x=27 までに水平線を描画
    draw_line(&mut screen, width, 5, 27, 1);

    // 結果をビット列で表示
    for row in 0..3 {
        let slice = &screen[row * (width/8) .. (row+1) * (width/8)];
        for byte in slice {
            print!("{:08b} ", byte);
        }
        println!();
    }
}