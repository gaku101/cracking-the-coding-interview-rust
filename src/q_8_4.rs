pub fn get_subsets<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
    match set.split_first() {
        // 先頭要素がある場合
        Some((first, rest)) => {
            // rest の部分集合を再帰的に取得
            let mut subsets = get_subsets(rest);

            // 生成済みの部分集合数だけループし、
            // 各 subset のクローンに first を push して追加
            let len = subsets.len();
            for i in 0..len {
                let mut new_subset = subsets[i].clone();
                new_subset.push(first.clone());
                subsets.push(new_subset);
            }

            subsets
        }
        // 空スライスなら空集合を返す
        None => vec![Vec::new()],
    }
}
/*
時間計算量：O(n·2ⁿ)
空間計算量：O(n·2ⁿ) （＋再帰スタック O(n)）
*/
