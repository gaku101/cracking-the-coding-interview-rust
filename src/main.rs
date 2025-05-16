mod q_8_3;
use q_8_3::find_magic_index;

fn main() {
    let arr = vec![-1, 0, 2, 3, 4, 4, 5];
    match find_magic_index(&arr) {
        Some(i) => println!("マジック・インデックスを発見: i = {}", i),
        None => println!("マジック・インデックスは存在しません。"),
    }
}
