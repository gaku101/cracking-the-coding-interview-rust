use std::collections::HashMap;

pub fn count_ways(n: usize) -> usize {
    let coins = [25, 10, 5, 1];
    let mut memo = HashMap::new();
    count_recursive(n, 0, &coins, &mut memo)
}

fn count_recursive(
    amount: usize,
    index: usize,
    coins: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if index >= coins.len() - 1 {
        return 1; // 残りをすべて1セントで埋める方法は1通り
    }

    if let Some(&cached) = memo.get(&(amount, index)) {
        return cached;
    }

    let coin = coins[index];
    let mut ways = 0;
    let mut i = 0;

    while coin * i <= amount {
        let remaining = amount - coin * i;
        ways += count_recursive(remaining, index + 1, coins, memo);
        i += 1;
    }

    memo.insert((amount, index), ways);
    ways
}
/*
時間計算量： O(N * C)

N: 金額（セント単位）、C: コインの種類数（ここでは最大4）

空間計算量： O(N * C)（メモ化のためのHashMap）
*/