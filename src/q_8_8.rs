pub fn permute_unique<T>(items: &[T]) -> Vec<Vec<T>>
where
    T: Eq + Ord + Clone + std::fmt::Debug,
{
    let mut results = Vec::new();
    let mut curr = Vec::with_capacity(items.len());
    let mut used = vec![false; items.len()];

    let mut sorted: Vec<T> = items.to_vec();
    sorted.sort();

    backtrack(&sorted, &mut used, &mut curr, &mut results);
    results
}

fn backtrack<T>(sorted: &[T], used: &mut [bool], curr: &mut Vec<T>, results: &mut Vec<Vec<T>>)
where
    T: Eq + Clone + std::fmt::Debug,
{
    if curr.len() == sorted.len() {
        results.push(curr.clone());
        return;
    }

    for i in 0..sorted.len() {
        if used[i] {
            continue;
        }
        // 重複要素をスキップ：同じ要素で，かつ直前の同一要素がまだ使われていなければ飛ばす
        if i > 0 && sorted[i] == sorted[i - 1] && !used[i - 1] {
            continue;
        }

        used[i] = true;
        curr.push(sorted[i].clone());
        println!("After push: curr = {:?}", curr);
        backtrack(sorted, used, curr, results);
        println!("Before pop: curr = {:?}", curr);
        curr.pop();
        used[i] = false;
    }
}
/*
時間計算量: O(n! * n)

n 個の要素からなる順列は最大で n! 通り。そのうち各順列を curr.clone() するコストが O(n)。

空間計算量: O(n! * n)

ユニークな順列をすべて保持する結果構造が O(n! * n)、再帰の呼び出し深さに伴う補助スタックが O(n)。
*/
