pub fn recursive_multiply(a: u32, b: u32) -> u32 {
    let (smaller, larger) = if a < b { (a, b) } else { (b, a) };
    multiply_helper(smaller, larger)
}

// 再帰の本体。smaller が 0 または 1 のときはベースケース。
// それ以外では smaller を半減して部分問題を解き、結果を合成する。
fn multiply_helper(smaller: u32, larger: u32) -> u32 {
    if smaller == 0 {
        0
    } else if smaller == 1 {
        larger
    } else {
        // smaller を半分にして部分積を計算
        let half_prod = multiply_helper(smaller >> 1, larger);
        // smaller が偶数なら half_prod*2、奇数なら half_prod*2 + larger
        if smaller & 1 == 0 {
            half_prod + half_prod
        } else {
            half_prod + half_prod + larger
        }
    }
}

/*
時間計算量: O(log n)
└ smaller を半分にしながら再帰するため、再帰呼び出しの深さは約 log₂(smaller) に比例します。

空間計算量: O(log n)
└ 再帰呼び出しのスタック使用量が呼び出し深さに比例します。
*/
