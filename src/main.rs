mod q_8_2;
use q_8_2::find_path;

fn main() {
    // true: 通行可, false: 通行不可
    let grid = vec![
        vec![true, true, true, true],
        vec![true, false, true, true],
        vec![true, true, true, false],
        vec![false, true, true, true],
    ];

    match find_path(&grid) {
        Some(path) => {
            println!("パスが見つかりました:");
            for (r, c) in path {
                println!("→ ({}, {})", r, c);
            }
        }
        None => println!("通れるパスは存在しません。"),
    }
}
