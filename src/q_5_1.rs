/// 整数 N の i から j のビットをクリアし、
/// M を i ビット分左シフトして N に挿入する関数
///
/// 例: N = 10000000000, M = 10011, i = 2, j = 6
///     → 結果は 10001001100 となる
pub fn insert_number(n: u32, m: u32, i: u32, j: u32) -> u32 {
    // 全ビット1の値（32ビット全体の1）
    // Rustでは !0 で全ビットが1の値を取得できます。（例：32ビットの整数であれば 0xFFFFFFFF）
    let all_ones: u32 = !0;
    
    // 左側の部分: ビット j+1 以降は全て1
    let left = all_ones << (j + 1);
    
    // 右側の部分: ビット i-1 までは全て1
    let right = (1 << i) - 1;
    
    // マスク: i ～ j のビットだけを0にする
    let mask = left | right;
    
    // N の i ～ j のビットをクリアする
    let n_cleared = n & mask;
    
    // M を i ビット左シフトして配置
    let m_shifted = m << i;
    
    // 結果として、クリア済みNとシフト済みMの OR 演算
    n_cleared | m_shifted
}

/*
計算量
Time Complexity: O(1)
→ ビット演算のみで定数時間で処理できる。

Space Complexity: O(1)
→ 補助的な変数を定数個だけ使用するため。
*/