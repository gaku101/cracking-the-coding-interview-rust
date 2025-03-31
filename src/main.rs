mod q_4_7;

use q_4_7::build_order;

fn main() {
    // サンプルのプロジェクトと依存関係
    let projects = vec!["a", "b", "c", "d", "e", "f"];
    let dependencies = vec![("a", "d"), ("f", "b"), ("b", "d"), ("f", "a"), ("d", "c")];

    match build_order(projects, dependencies) {
        Ok(order) => println!("ビルド順序: {:?}", order),
        Err(err) => println!("エラー: {}", err),
    }
}
