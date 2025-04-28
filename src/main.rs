mod q_6_1;
use q_6_1::find_heavy_bottle;

fn main() {
    // 19本は1.0g、8番目(インデックス7)だけ1.1gとする
    let mut weights = vec![1.0; 20];
    weights[7] = 1.1;

    let heavy = find_heavy_bottle(&weights);
    println!("重い瓶のインデックス: {}", heavy); // 出力: 7
}
