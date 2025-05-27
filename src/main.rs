mod q_8_8;
use q_8_8::permute_unique;

fn main() {
    let nums = vec![1, 1, 2];
    let perms = permute_unique(&nums);

    for p in perms {
        println!("{:?}", p);
    }
    // 出力例（順序は探索順による）:
    // [1, 1, 2]
    // [1, 2, 1]
    // [2, 1, 1]
}
