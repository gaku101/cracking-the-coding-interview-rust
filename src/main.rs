mod q_8_6;
use q_8_6::hanoi;

fn main() {
    // ディスク数
    let n = 3;
    // A→B（補助）→C へ移動
    hanoi(n, 'A', 'B', 'C');
}
