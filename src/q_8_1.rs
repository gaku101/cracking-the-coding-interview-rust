// 階段の段数 n を与えると、1,2,3 段ずつ上がる
// パターン数を返す。
// メモ化再帰で O(n) に抑えています。
pub fn count_ways(n: i32) -> i64 {
    let mut memo = vec![-1; (n as usize) + 1];
    count_ways_memo(n, &mut memo)
}

fn count_ways_memo(n: i32, memo: &mut Vec<i64>) -> i64 {
    // println!("n = {}", n);
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    let idx = n as usize;
    if memo[idx] != -1 {
        return memo[idx];
    }
    let ways =
        count_ways_memo(n - 1, memo) + count_ways_memo(n - 2, memo) + count_ways_memo(n - 3, memo);
    memo[idx] = ways;
    println!("idx = {}", idx);
    println!("ways = {}", ways);
    ways
}
